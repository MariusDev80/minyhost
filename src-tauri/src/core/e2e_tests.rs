//! End-to-end test of the whole server lifecycle, against the real APIs.
//!
//! Downloads Java and a server jar (~120 MB), so it is ignored by default.
//! Run it with:
//!   cargo test --manifest-path src-tauri/Cargo.toml -- --ignored --nocapture

use std::sync::{Arc, Mutex};
use std::time::Duration;

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

/// Creates the latest version of `loader`, starts it, sends a command, stops it.
async fn run_lifecycle(loader: Loader) {
    // Shared between runs so Java is only downloaded once.
    let root = std::env::temp_dir().join("minyhost-e2e");
    let paths = AppPaths::new(root.clone());
    let http = http_client().unwrap();

    let versions = providers::list_versions(&http, loader).await.unwrap();
    let mc_version = versions.first().unwrap().clone();
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

    manager.start(&instance.id).await.unwrap();
    let ready = tokio::time::timeout(Duration::from_secs(180), async {
        while manager.status(&instance.id).await != ServerStatus::Running {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await;
    assert!(ready.is_ok(), "server did not reach Running");

    manager.send_command(&instance.id, "list").await.unwrap();
    let outcome = manager.stop(&instance.id).await.unwrap();
    assert!(matches!(outcome, StopOutcome::Graceful));
    assert_eq!(manager.status(&instance.id).await, ServerStatus::Stopped);

    let crashed = events
        .lock()
        .unwrap()
        .iter()
        .any(|event| matches!(event, ServerEvent::Crashed { .. }));
    assert!(!crashed, "a clean stop must not be reported as a crash");

    std::fs::remove_dir_all(paths.server_dir(&instance.id)).unwrap();
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
