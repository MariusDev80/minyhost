//! End-to-end test of the whole server lifecycle, against the real APIs.
//!
//! Downloads Java and a server jar (~120 MB), so it is ignored by default.
//! Run it with:
//!   cargo test --manifest-path src-tauri/Cargo.toml -- --ignored --nocapture

use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::core::content::Source;
use crate::core::create::{create_server, NewServer};
use crate::core::process::{ProcessManager, ServerEvent, ServerStatus, StopOutcome};
use crate::core::providers::{self, Loader};
use crate::download::http_client;
use crate::paths::AppPaths;

#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn vanilla_lifecycle() {
    run_lifecycle(Loader::Vanilla).await;
}

#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn paper_lifecycle() {
    run_lifecycle(Loader::Paper).await;
}

#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn fabric_lifecycle() {
    run_lifecycle(Loader::Fabric).await;
}

#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn neoforge_lifecycle() {
    run_lifecycle(Loader::Neoforge).await;
}

#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn forge_lifecycle() {
    run_lifecycle(Loader::Forge).await;
}

/// The oldest Forge offered: Java 8 and the legacy `forge-<version>.jar` layout.
#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn forge_oldest_lifecycle() {
    run_lifecycle_of(Loader::Forge, "1.12.2").await;
}

/// Creates the latest version of `loader`, starts it, sends a command, stops it.
async fn run_lifecycle(loader: Loader) {
    let http = http_client().unwrap();
    let versions = providers::list_versions(&http, loader).await.unwrap();
    run_lifecycle_of(loader, versions.first().unwrap()).await;
}

async fn run_lifecycle_of(loader: Loader, mc_version: &str) {
    // Shared between runs so Java is only downloaded once.
    let root = std::env::temp_dir().join("minyhost-e2e");
    let paths = AppPaths::new(root.clone());
    let http = http_client().unwrap();

    let mc_version = mc_version.to_string();
    println!(
        "creating a {loader:?} {mc_version} server in {}",
        root.display()
    );

    let input = NewServer {
        name: "Test E2E".into(),
        loader,
        mc_version,
        memory_mb: 2048,
        eula_accepted: true,
        settings: Default::default(),
    };
    let instance = create_server(&http, &paths, input, |p| println!("{p:?}"))
        .await
        .unwrap();
    start_and_stop(&paths, &instance.id).await;
}

/// The official Cobblemon modpack (Fabric): pinned loader, mods, overrides.
#[tokio::test]
#[ignore = "downloads Java, Minecraft and a modpack, takes a few minutes"]
async fn modrinth_modpack_lifecycle() {
    run_modpack(
        Source::Modrinth,
        "cobblemon-fabric",
        Loader::Fabric,
        Some("Cobblemon"),
    )
    .await;
}

#[tokio::test]
#[ignore = "downloads Java, Minecraft and a modpack, takes a few minutes"]
async fn modrinth_neoforge_modpack_lifecycle() {
    run_modpack(
        Source::Modrinth,
        "create-oneblock",
        Loader::Neoforge,
        Some("Create"),
    )
    .await;
}

#[tokio::test]
#[ignore = "downloads Java, Minecraft and a modpack, takes a few minutes"]
async fn modrinth_forge_modpack_lifecycle() {
    run_modpack(Source::Modrinth, "slimes-adventure", Loader::Forge, None).await;
}

/// A CurseForge pack with a server pack (Forge 1.12.2, Java 8).
#[tokio::test]
#[ignore = "needs CURSEFORGE_API_KEY, downloads Java, Minecraft and a modpack"]
async fn curseforge_server_pack_lifecycle() {
    run_modpack(Source::Curseforge, "296062", Loader::Forge, None).await;
}

/// A CurseForge pack without a server pack: built from the client manifest.
#[tokio::test]
#[ignore = "needs CURSEFORGE_API_KEY, downloads Java, Minecraft and a modpack"]
async fn curseforge_manifest_modpack_lifecycle() {
    run_modpack(Source::Curseforge, "1258033", Loader::Fabric, None).await;
}

/// Creates a server from the latest version of a pack made for `loader`
/// (CurseForge packs do not always tag their loader).
async fn run_modpack(source: Source, project: &str, loader: Loader, expected_mod: Option<&str>) {
    use crate::core::content::{self, modpack};
    use crate::core::create::{create_from_modpack, NewModpackServer};

    let root = std::env::temp_dir().join("minyhost-e2e");
    let paths = AppPaths::new(root.clone());
    let http = http_client().unwrap();

    let loader_name = format!("{loader:?}").to_lowercase();
    let versions = modpack::versions(&http, source, project).await.unwrap();
    let version = versions
        .iter()
        .find(|v| v.supported && (v.loaders.is_empty() || v.loaders.contains(&loader_name)))
        .unwrap();
    println!("creating from {} ({:?})", version.name, version.loaders);

    let input = NewModpackServer {
        name: "Test Modpack".into(),
        memory_mb: 4096,
        eula_accepted: true,
        source,
        project_id: project.into(),
        version_id: version.id.clone(),
    };
    let created = create_from_modpack(&http, &paths, input, |p| println!("{p:?}"))
        .await
        .unwrap();
    println!("manual downloads: {:?}", created.manual);
    if source == Source::Modrinth {
        assert!(created.manual.is_empty());
    }
    let instance = created.instance;
    assert_eq!(instance.loader, loader);
    assert!(instance.modpack.is_some());

    let dir = paths.server_dir(&instance.id);
    let installed = content::list_installed(&dir, instance.loader).unwrap();
    println!("{} files installed", installed.len());
    if let Some(expected) = expected_mod {
        assert!(installed
            .iter()
            .any(|item| item.meta.as_ref().is_some_and(|m| m.title == expected)));
    }
    assert!(
        paths.downloads_dir().read_dir().unwrap().next().is_none(),
        "archive removed"
    );

    start_and_stop(&paths, &instance.id).await;
}

/// Starts the server, waits until players can join, sends a command, stops
/// it, then deletes it.
async fn start_and_stop(paths: &AppPaths, id: &str) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink_events = events.clone();
    let manager = ProcessManager::new(
        paths.clone(),
        Arc::new(move |event| {
            if let ServerEvent::Console { line, .. } = &event {
                println!("> {line}");
            }
            sink_events.lock().unwrap().push(event);
        }),
    );

    manager.start(id).await.unwrap();
    let ready = tokio::time::timeout(Duration::from_secs(300), async {
        while manager.status(id).await != ServerStatus::Running {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await;
    assert!(ready.is_ok(), "server did not reach Running");

    manager.send_command(id, "list").await.unwrap();
    let outcome = manager.stop(id).await.unwrap();
    assert!(matches!(outcome, StopOutcome::Graceful));
    assert_eq!(manager.status(id).await, ServerStatus::Stopped);

    let crashed = events
        .lock()
        .unwrap()
        .iter()
        .any(|event| matches!(event, ServerEvent::Crashed { .. }));
    assert!(!crashed, "a clean stop must not be reported as a crash");

    std::fs::remove_dir_all(paths.server_dir(id)).unwrap();
}

/// Search, install with dependencies, update, then plugins and datapacks.
#[tokio::test]
#[ignore = "calls the Modrinth API"]
async fn modrinth_content() {
    use crate::core::content::{self, ContentKind, Source, Target};

    let http = http_client().unwrap();
    let dir = std::env::temp_dir().join("minyhost-e2e-content");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mods = Target {
        kind: ContentKind::Mod,
        mc_version: "1.21.1".into(),
        loader: Loader::Fabric,
    };
    let page = content::search(&http, Source::Modrinth, mods.kind, "chunky", Some(&mods), 0)
        .await
        .unwrap();
    let chunky = page.hits.iter().find(|hit| hit.title == "Chunky").unwrap();

    // Chunky needs Fabric API, installed along with it.
    let outcome = content::install(&http, &dir, Source::Modrinth, &chunky.project_id, &mods)
        .await
        .unwrap();
    println!("{outcome:?}");
    assert_eq!(outcome.installed.len(), 2);
    let installed = content::list_installed(&dir, Loader::Fabric).unwrap();
    assert_eq!(installed.len(), 2);
    assert!(installed
        .iter()
        .any(|item| item.meta.as_ref().unwrap().dependency));

    // Installing again replaces the file, dependencies are left alone.
    let again = content::install(&http, &dir, Source::Modrinth, &chunky.project_id, &mods)
        .await
        .unwrap();
    assert_eq!(again.installed, vec!["Chunky".to_string()]);
    assert_eq!(
        content::list_installed(&dir, Loader::Fabric).unwrap().len(),
        2
    );

    // A plugin and a datapack on a Paper server.
    for (kind, query) in [
        (ContentKind::Plugin, "chunky"),
        (ContentKind::Datapack, "veinminer"),
    ] {
        let target = Target {
            kind,
            mc_version: "1.21.1".into(),
            loader: Loader::Paper,
        };
        let page = content::search(&http, Source::Modrinth, kind, query, Some(&target), 0)
            .await
            .unwrap();
        let hit = page.hits.first().unwrap();
        content::install(&http, &dir, Source::Modrinth, &hit.project_id, &target)
            .await
            .unwrap();
    }
    let paper = content::list_installed(&dir, Loader::Paper).unwrap();
    assert!(paper.iter().any(|item| item.kind == ContentKind::Plugin));
    assert!(dir.join("world/datapacks").read_dir().unwrap().count() == 1);

    std::fs::remove_dir_all(&dir).unwrap();
}

/// What the modpack picker loads: a page of results, then each one's versions.
#[tokio::test]
#[ignore = "needs CURSEFORGE_API_KEY, calls the CurseForge API"]
async fn curseforge_modpack_picker() {
    use crate::core::content::{self, modpack, ContentKind};

    let http = http_client().unwrap();
    for offset in [0, 20] {
        let page = content::search(
            &http,
            Source::Curseforge,
            ContentKind::Modpack,
            "",
            None,
            offset,
        )
        .await
        .unwrap();
        println!("offset {offset}: {} hits / {}", page.hits.len(), page.total);
        for hit in &page.hits {
            match modpack::versions(&http, Source::Curseforge, &hit.project_id).await {
                Ok(versions) => println!("  {} -> {} versions", hit.title, versions.len()),
                Err(err) => println!("  {} -> ERROR {err}", hit.title),
            }
        }
    }
}

/// Same as `modrinth_content`, from CurseForge.
#[tokio::test]
#[ignore = "needs CURSEFORGE_API_KEY, calls the CurseForge API"]
async fn curseforge_content() {
    use crate::core::content::{self, ContentKind, Target};

    let http = http_client().unwrap();
    let dir = std::env::temp_dir().join("minyhost-e2e-curseforge");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    // Waystones needs Balm, installed along with it.
    let mods = Target {
        kind: ContentKind::Mod,
        mc_version: "1.21.1".into(),
        loader: Loader::Fabric,
    };
    let page = content::search(
        &http,
        Source::Curseforge,
        mods.kind,
        "waystones",
        Some(&mods),
        0,
    )
    .await
    .unwrap();
    let waystones = page
        .hits
        .iter()
        .find(|hit| hit.title == "Waystones")
        .unwrap();
    let outcome = content::install(
        &http,
        &dir,
        Source::Curseforge,
        &waystones.project_id,
        &mods,
    )
    .await
    .unwrap();
    println!("{outcome:?}");
    assert!(outcome.installed.len() >= 2, "dependencies installed too");

    // A plugin (often tagged "1.21" only) and a datapack on a Paper server.
    for (kind, query) in [
        (ContentKind::Plugin, "worldedit"),
        (ContentKind::Datapack, ""),
    ] {
        let target = Target {
            kind,
            mc_version: "1.21.1".into(),
            loader: Loader::Paper,
        };
        let page = content::search(&http, Source::Curseforge, kind, query, Some(&target), 0)
            .await
            .unwrap();
        let hit = page.hits.first().unwrap();
        println!("installing {kind:?} {}", hit.title);
        let outcome = content::install(&http, &dir, Source::Curseforge, &hit.project_id, &target)
            .await
            .unwrap();
        println!("{outcome:?}");
    }
    let paper = content::list_installed(&dir, Loader::Paper).unwrap();
    assert!(paper.iter().any(|item| item.kind == ContentKind::Plugin));
    assert!(paper.iter().any(|item| item.kind == ContentKind::Datapack));

    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
#[ignore = "calls the Mojang APIs"]
async fn player_lookup_and_skin() {
    use crate::core::players;
    use crate::error::AppError;

    let http = http_client().unwrap();
    let notch = players::lookup(&http, "notch").await.unwrap();
    assert_eq!(notch.name, "Notch"); // exact case comes from Mojang
    assert_eq!(notch.uuid, "069a79f4-44e9-4726-a5be-fca90e38aaf5");

    let skin = players::skin_data_url(&http, &notch.uuid).await.unwrap();
    assert!(skin.starts_with("data:image/png;base64,"));

    let unknown = players::lookup(&http, "zz_nosuch_pl4yer").await;
    assert!(matches!(unknown, Err(AppError::PlayerNotFound(_))));
}

/// The world is the source of truth for game rules:
/// - rules chosen at creation are applied at first start (every Paper dimension),
/// - a change typed in the console is seen live, then read back from the save,
/// - a restart does not overwrite it.
#[tokio::test]
#[ignore = "downloads Java and Minecraft, takes a few minutes"]
async fn game_rules_follow_the_world() {
    use std::collections::BTreeMap;

    use crate::core::game_settings::{self, GameSettings, SettingValue};
    use crate::core::instances;
    use crate::core::properties::Properties;

    let root = std::env::temp_dir().join("minyhost-e2e");
    let paths = AppPaths::new(root.clone());
    let http = http_client().unwrap();
    let mc_version = providers::list_versions(&http, Loader::Paper)
        .await
        .unwrap()
        .remove(0);

    let input = NewServer {
        name: "Test settings".into(),
        loader: Loader::Paper,
        mc_version,
        memory_mb: 2048,
        eula_accepted: true,
        settings: GameSettings {
            properties: BTreeMap::from([("view-distance".into(), SettingValue::Int(6))]),
            game_rules: BTreeMap::from([("keep_inventory".into(), SettingValue::Bool(true))]),
        },
    };
    let instance = create_server(&http, &paths, input, |_| {}).await.unwrap();
    let id = instance.id.clone();
    let dir = paths.server_dir(&id);
    let file = Properties::load(&dir.join("server.properties")).unwrap();
    assert_eq!(file.get("view-distance"), Some("6"));
    assert_eq!(instance.pending_game_rules.len(), 1);

    let lines = Arc::new(Mutex::new(Vec::new()));
    let sink_lines = lines.clone();
    let manager = ProcessManager::new(
        paths.clone(),
        Arc::new(move |event| {
            if let ServerEvent::Console { line, .. } = event {
                sink_lines.lock().unwrap().push(line);
            }
        }),
    );
    let start = |manager: ProcessManager, id: String| async move {
        manager.start(&id).await.unwrap();
        tokio::time::timeout(Duration::from_secs(180), async {
            while manager.status(&id).await != ServerStatus::Running {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        })
        .await
        .unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;
    };
    let rules_now = |manager: ProcessManager, id: String| {
        let paths = paths.clone();
        async move {
            let instance = instances::load(&paths, &id).unwrap();
            let live = manager.live_rules(&id).await;
            game_settings::read(&paths.server_dir(&id), &instance, &live)
                .unwrap()
                .game_rules
        }
    };

    // 1. First start: the pending rule is applied in every dimension, then cleared.
    start(manager.clone(), id.clone()).await;
    assert!(instances::load(&paths, &id)
        .unwrap()
        .pending_game_rules
        .is_empty());
    manager
        .send_command(
            &id,
            "execute in minecraft:the_nether run gamerule keep_inventory",
        )
        .await
        .unwrap();

    // 2. Change typed in the console: seen live, without any save yet.
    manager
        .send_command(&id, "gamerule random_tick_speed 10")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_eq!(
        rules_now(manager.clone(), id.clone()).await["random_tick_speed"],
        SettingValue::Int(10)
    );

    // 3. Stopped: values are read back from the world save files.
    manager.stop(&id).await.unwrap();
    let saved = rules_now(manager.clone(), id.clone()).await;
    assert_eq!(saved["keep_inventory"], SettingValue::Bool(true));
    assert_eq!(saved["random_tick_speed"], SettingValue::Int(10));

    // 4. Restart: nothing overwrites the console change.
    start(manager.clone(), id.clone()).await;
    manager.stop(&id).await.unwrap();
    assert_eq!(
        rules_now(manager.clone(), id.clone()).await["random_tick_speed"],
        SettingValue::Int(10)
    );

    let lines = lines.lock().unwrap();
    for line in lines.iter().filter(|l| l.contains("ame rule")) {
        println!("{line}");
    }
    assert!(lines.iter().any(|l| l.contains("keep_inventory")
        && l.contains("currently set to")
        && l.ends_with("true")));
    std::fs::remove_dir_all(dir).unwrap();
}

/// Starts linking a playit.gg account: the claim code must be accepted by the
/// API and wait for the user. No account is needed (nobody approves it).
#[tokio::test]
#[ignore = "calls the playit.gg API"]
async fn playit_link_starts() {
    use crate::core::tunnel::{LinkState, TunnelManager};

    let root = std::env::temp_dir().join(format!("minyhost-e2e-playit-{}", uuid::Uuid::new_v4()));
    let paths = AppPaths::new(root.clone());
    let states = Arc::new(Mutex::new(Vec::new()));
    let sink_states = states.clone();
    let tunnels = TunnelManager::new(
        paths,
        Arc::new(move |state| sink_states.lock().unwrap().push(state)),
    );

    let url = tunnels.start_link().await.unwrap();
    assert!(url.starts_with("https://playit.gg/claim/"), "{url}");
    // A few polls against the real API: still waiting, no error.
    tokio::time::sleep(Duration::from_secs(6)).await;
    let state = tunnels.state().await;
    assert_eq!(state.link, LinkState::Linking { url });
    assert_eq!(state.link_error, None);

    tunnels.cancel_link().await;
    assert_eq!(tunnels.state().await.link, LinkState::Unlinked);
    let _ = std::fs::remove_dir_all(root);
}
