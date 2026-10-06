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
    };
    let instance = create_server(&http, &paths, input, |p| println!("{p:?}"))
        .await
        .unwrap();

    let events = Arc::new(Mutex::new(Vec::new()));
    let sink_events = events.clone();
    let manager = ProcessManager::new(Arc::new(move |event| {
        if let ServerEvent::Console { line, .. } = &event {
            println!("> {line}");
        }
        sink_events.lock().unwrap().push(event);
    }));

    manager.start(&paths, &instance.id).await.unwrap();
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
