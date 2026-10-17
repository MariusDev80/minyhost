fn main() {
    // Read by `option_env!` in `core/content/curseforge.rs`: rebuild when it changes.
    println!("cargo:rerun-if-env-changed=CURSEFORGE_API_KEY");
    tauri_build::build()
}
