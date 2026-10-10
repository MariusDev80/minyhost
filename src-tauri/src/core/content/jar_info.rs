//! What a mod jar says about itself: its ids, the mods it requires, and
//! (Fabric only) whether it only runs on the client.
//!
//! - Fabric: `fabric.mod.json` (`id`, `provides`, `depends`, `environment`),
//! - Forge / NeoForge: `META-INF/mods.toml` or `META-INF/neoforge.mods.toml`
//!   (`[[mods]] modId`, `[[dependencies.<id>]]` with `mandatory` or `type`).
//!
//! Only the few keys above are read, so a tiny reader replaces a TOML parser.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;

/// Dependencies every mod has, provided by the server itself.
const PLATFORM_IDS: [&str; 5] = ["minecraft", "java", "forge", "neoforge", "fabricloader"];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct JarInfo {
    pub ids: BTreeSet<String>,
    pub requires: BTreeSet<String>,
    pub client_only: bool,
}

/// `None` if the jar cannot be read or is not a mod.
pub fn read(jar: &Path) -> Option<JarInfo> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(jar).ok()?).ok()?;
    let mut entry = |name: &str| -> Option<String> {
        let mut text = String::new();
        zip.by_name(name).ok()?.read_to_string(&mut text).ok()?;
        Some(text)
    };
    if let Some(text) = entry("fabric.mod.json") {
        return fabric(&text);
    }
    entry("META-INF/neoforge.mods.toml")
        .or_else(|| entry("META-INF/mods.toml"))
        .map(|text| forge(&text))
}

fn fabric(text: &str) -> Option<JarInfo> {
    let manifest: serde_json::Value = serde_json::from_str(text).ok()?;
    let mut info = JarInfo {
        client_only: manifest.get("environment").and_then(|env| env.as_str()) == Some("client"),
        ..JarInfo::default()
    };
    info.ids.extend(
        manifest
            .get("id")
            .and_then(|id| id.as_str())
            .map(String::from),
    );
    if let Some(provides) = manifest.get("provides").and_then(|p| p.as_array()) {
        info.ids.extend(
            provides
                .iter()
                .filter_map(|id| id.as_str())
                .map(String::from),
        );
    }
    if let Some(depends) = manifest.get("depends").and_then(|d| d.as_object()) {
        info.requires.extend(
            depends
                .keys()
                .filter(|id| !PLATFORM_IDS.contains(&id.as_str()))
                .cloned(),
        );
    }
    Some(info)
}

fn forge(text: &str) -> JarInfo {
    #[derive(Default)]
    struct Dependency {
        id: Option<String>,
        mandatory: Option<bool>,
        kind: Option<String>,
    }

    let mut info = JarInfo::default();
    let mut in_mods = false;
    let mut dependency: Option<Dependency> = None;
    let finish = |dependency: Option<Dependency>, info: &mut JarInfo| {
        let Some(Dependency {
            id: Some(id),
            mandatory,
            kind,
        }) = dependency
        else {
            return;
        };
        // Old files say `mandatory`, newer ones `type`; neither means required.
        let required = match (mandatory, kind.as_deref()) {
            (Some(mandatory), _) => mandatory,
            (None, Some(kind)) => kind == "required",
            (None, None) => true,
        };
        if required && !PLATFORM_IDS.contains(&id.as_str()) {
            info.requires.insert(id);
        }
    };

    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            finish(dependency.take(), &mut info);
            in_mods = line.starts_with("[[mods]]");
            if line.starts_with("[[dependencies") {
                dependency = Some(Dependency::default());
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = toml_value(value);
        match (key.trim(), dependency.as_mut()) {
            ("modId", None) if in_mods => {
                info.ids.insert(value);
            }
            ("modId", Some(dependency)) => dependency.id = Some(value),
            ("mandatory", Some(dependency)) => dependency.mandatory = Some(value == "true"),
            ("type", Some(dependency)) => dependency.kind = Some(value.to_lowercase()),
            _ => {}
        }
    }
    finish(dependency, &mut info);
    info
}

/// `"prism" # comment` -> `prism`, `true` -> `true`.
fn toml_value(raw: &str) -> String {
    let raw = raw.trim();
    match raw.strip_prefix('"') {
        Some(quoted) => quoted.split('"').next().unwrap_or_default().to_string(),
        None => raw.split('#').next().unwrap_or_default().trim().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|id| id.to_string()).collect()
    }

    #[test]
    fn reads_forge_mods_toml() {
        let toml = r#"
modLoader="javafml"
[[mods]]
modId="legendarytooltips" # the mod
version="1.4.0"
[[dependencies.legendarytooltips]]
    modId="forge"
    mandatory=true
[[dependencies.legendarytooltips]]
    modId="prism"
    mandatory=true
    versionRange="[1.0.1,)"
[[dependencies.legendarytooltips]]
    modId="jei"
    mandatory=false
"#;
        let info = forge(toml);
        assert_eq!(info.ids, set(&["legendarytooltips"]));
        assert_eq!(info.requires, set(&["prism"]));
        assert!(!info.client_only);
    }

    #[test]
    fn reads_neoforge_dependency_types() {
        let toml = r#"
[[mods]]
modId="create"
[[dependencies.create]]
modId="ponder"
type="required"
[[dependencies.create]]
modId="jei"
type="optional"
[[dependencies.create]]
modId="neoforge"
type="required"
"#;
        let info = forge(toml);
        assert_eq!(info.requires, set(&["ponder"]));
    }

    #[test]
    fn reads_fabric_mod_json() {
        let json = r#"{"id":"etf","provides":["etf_api"],"environment":"client",
            "depends":{"fabricloader":">=0.15","minecraft":"1.21.1","fabric-api":"*"}}"#;
        let info = fabric(json).unwrap();
        assert_eq!(info.ids, set(&["etf", "etf_api"]));
        assert_eq!(info.requires, set(&["fabric-api"]));
        assert!(info.client_only);
    }
}
