//! Servers created from a modpack (Modrinth or CurseForge).
//!
//! Creation (`create.rs`) runs in two parts around the usual pipeline:
//! 1. `prepare` downloads the pack archive and reads which Minecraft version
//!    and loader (exact version) it was made for,
//! 2. once Java and the loader are installed, `install` puts the pack's files
//!    in the server folder.
//!
//! Modrinth (`.mrpack`, https://support.modrinth.com/en/articles/8802351):
//! `modrinth.index.json` lists the files with their hashes and where they run
//! (`env.server`), then `overrides/` and `server-overrides/` are copied as is.
//!
//! CurseForge: when the author published a server pack, it is used (minus its
//! start scripts and jars: MinyHost installs the loader itself). Otherwise the
//! client pack's `manifest.json` lists the files; those tagged client-only are
//! skipped, and those their author keeps on CurseForge's website are left for
//! the user to download (`ManualDownload`).
//!
//! Archive paths are never trusted: entries that would land outside the
//! server folder are skipped.

use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::{
    content_dir, curseforge, modrinth, record, safe_file_name, ContentKind, ContentMeta,
    ManualDownload, Source,
};
use crate::core::providers::Loader;
use crate::download::{download_file, Checksum};
use crate::error::{AppError, AppResult};
use crate::paths::AppPaths;

const PARALLEL_DOWNLOADS: usize = 6;
/// Attempts per file: with hundreds of files, a network hiccup is likely.
const DOWNLOAD_ATTEMPTS: usize = 3;
/// The only hosts a `.mrpack` may download from (Modrinth's format rules).
const MRPACK_HOSTS: [&str; 4] = [
    "cdn.modrinth.com",
    "github.com",
    "raw.githubusercontent.com",
    "gitlab.com",
];
/// CurseForge categories installed from a client manifest.
const CURSEFORGE_MODS: u32 = 6;
const CURSEFORGE_DATAPACKS: u32 = 6945;

/// One version of a modpack, to choose from. Mirrored by `ModpackVersion`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModpackVersion {
    pub id: String,
    pub name: String,
    pub mc_versions: Vec<String>,
    pub loaders: Vec<String>,
    /// Made for a loader MinyHost can host.
    pub supported: bool,
}

/// Versions of a modpack, newest first.
pub async fn versions(
    http: &reqwest::Client,
    source: Source,
    project_id: &str,
) -> AppResult<Vec<ModpackVersion>> {
    match source {
        Source::Modrinth => {
            let versions = modrinth::versions(http, project_id, &[], None).await?;
            Ok(versions
                .into_iter()
                .map(|version| ModpackVersion {
                    supported: version.loaders.iter().any(|l| loader_named(l).is_some()),
                    id: version.id,
                    name: version.name,
                    mc_versions: version.game_versions,
                    loaders: version.loaders,
                })
                .collect())
        }
        Source::Curseforge => {
            let files = curseforge::project_files(http, project_id).await?;
            Ok(files
                .into_iter()
                .map(|file| {
                    let (mc_versions, loaders): (Vec<String>, Vec<String>) = file
                        .game_versions
                        .iter()
                        .filter(|tag| !matches!(tag.as_str(), "Client" | "Server"))
                        .cloned()
                        .partition(|tag| tag.starts_with(|c: char| c.is_ascii_digit()));
                    let loaders: Vec<String> = loaders.iter().map(|l| l.to_lowercase()).collect();
                    ModpackVersion {
                        // Some packs do not tag their loader: let them through.
                        supported: loaders.is_empty()
                            || loaders.iter().any(|l| loader_named(l).is_some()),
                        id: file.id.to_string(),
                        name: file.display_name,
                        mc_versions,
                        loaders,
                    }
                })
                .collect())
        }
    }
}

fn loader_named(name: &str) -> Option<Loader> {
    match name {
        "fabric" | "fabric-loader" => Some(Loader::Fabric),
        "neoforge" => Some(Loader::Neoforge),
        "forge" => Some(Loader::Forge),
        _ => None,
    }
}

/// A downloaded modpack, ready to be installed into a new server.
pub struct Prepared {
    pub loader: Loader,
    pub mc_version: String,
    /// `None`: latest stable one.
    pub loader_version: Option<String>,
    /// The modpack itself, remembered in `instance.json`.
    pub meta: ContentMeta,
    archive: TempFile,
    plan: Plan,
}

enum Plan {
    Modrinth {
        files: Vec<MrpackFile>,
    },
    Curseforge {
        manifest: CurseforgeManifest,
        mod_id: u32,
        server_pack: Option<u32>,
    },
}

/// Removed when dropped, whatever happens to the creation.
struct TempFile(PathBuf);

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MrpackIndex {
    game: String,
    files: Vec<MrpackFile>,
    dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct MrpackFile {
    path: String,
    hashes: MrpackHashes,
    env: Option<MrpackEnv>,
    downloads: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct MrpackHashes {
    sha1: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MrpackEnv {
    server: String,
}

#[derive(Debug, Deserialize)]
struct CurseforgeManifest {
    minecraft: CurseforgeMinecraft,
    #[serde(default)]
    files: Vec<CurseforgeFileRef>,
    #[serde(default = "default_overrides")]
    overrides: String,
}

fn default_overrides() -> String {
    "overrides".into()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CurseforgeMinecraft {
    version: String,
    #[serde(default)]
    mod_loaders: Vec<CurseforgeModLoader>,
}

#[derive(Debug, Deserialize)]
struct CurseforgeModLoader {
    /// "forge-47.2.0", "neoforge-21.1.77", "fabric-0.16.5"…
    id: String,
    #[serde(default)]
    primary: bool,
}

#[derive(Debug, Deserialize)]
struct CurseforgeFileRef {
    #[serde(rename = "projectID")]
    project_id: u32,
    #[serde(rename = "fileID")]
    file_id: u32,
    #[serde(default = "default_required")]
    required: bool,
}

fn default_required() -> bool {
    true
}

/// Downloads the pack (`on_progress`: 0-100) and reads what it needs.
pub async fn prepare(
    http: &reqwest::Client,
    paths: &AppPaths,
    source: Source,
    project_id: &str,
    version_id: &str,
    on_progress: impl FnMut(u8),
) -> AppResult<Prepared> {
    let dir = paths.downloads_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let archive = TempFile(dir.join(format!("{}.zip", uuid::Uuid::new_v4())));
    match source {
        Source::Modrinth => prepare_modrinth(http, archive, version_id, on_progress).await,
        Source::Curseforge => {
            prepare_curseforge(http, archive, project_id, version_id, on_progress).await
        }
    }
}

async fn prepare_modrinth(
    http: &reqwest::Client,
    archive: TempFile,
    version_id: &str,
    on_progress: impl FnMut(u8),
) -> AppResult<Prepared> {
    let version = modrinth::get_version(http, version_id).await?;
    let project = modrinth::get_project(http, &version.project_id).await?;
    let file = modrinth::primary_file(&version)?;
    let checksum = file.hashes.sha1.clone().map(Checksum::Sha1);
    download_file(http, &file.url, &archive.0, checksum.as_ref(), on_progress).await?;

    let index: MrpackIndex = read_json(&archive.0, "modrinth.index.json").await?;
    if index.game != "minecraft" {
        return Err(AppError::InvalidData(format!("mrpack for {}", index.game)));
    }
    let mc_version = index
        .dependencies
        .get("minecraft")
        .cloned()
        .ok_or_else(|| AppError::InvalidData("mrpack without Minecraft version".into()))?;
    let (loader, loader_version) = mrpack_loader(&index.dependencies)?;
    let files = index
        .files
        .into_iter()
        .filter(|file| {
            file.env
                .as_ref()
                .is_none_or(|env| env.server != "unsupported")
        })
        .collect();

    Ok(Prepared {
        loader,
        mc_version,
        loader_version,
        meta: modrinth::project_meta(&project, &version),
        archive,
        plan: Plan::Modrinth { files },
    })
}

/// Loader named in the `dependencies` of `modrinth.index.json`.
fn mrpack_loader(dependencies: &BTreeMap<String, String>) -> AppResult<(Loader, Option<String>)> {
    for (key, loader) in [
        ("neoforge", Loader::Neoforge),
        ("forge", Loader::Forge),
        ("fabric-loader", Loader::Fabric),
    ] {
        if let Some(version) = dependencies.get(key) {
            return Ok((loader, Some(version.clone())));
        }
    }
    if dependencies.contains_key("quilt-loader") {
        return Err(AppError::ModpackUnsupported("Quilt".into()));
    }
    // Only Minecraft itself: datapacks and resources on a vanilla server.
    Ok((Loader::Vanilla, None))
}

async fn prepare_curseforge(
    http: &reqwest::Client,
    archive: TempFile,
    project_id: &str,
    version_id: &str,
    on_progress: impl FnMut(u8),
) -> AppResult<Prepared> {
    let invalid = |id: &str| AppError::InvalidInput(format!("CurseForge id {id:?}"));
    let mod_id: u32 = project_id.parse().map_err(|_| invalid(project_id))?;
    let file_id: u32 = version_id.parse().map_err(|_| invalid(version_id))?;
    let project = curseforge::project(http, project_id).await?;
    let file = curseforge::file(http, mod_id, file_id).await?;
    let remote = curseforge::remote_file(&file);
    let url = remote
        .url
        .ok_or_else(|| AppError::ContentUnavailable(project.name.clone()))?;
    download_file(
        http,
        &url,
        &archive.0,
        remote.checksum.as_ref(),
        on_progress,
    )
    .await?;

    let manifest: CurseforgeManifest = read_json(&archive.0, "manifest.json").await?;
    let primary = manifest
        .minecraft
        .mod_loaders
        .iter()
        .find(|loader| loader.primary)
        .or_else(|| manifest.minecraft.mod_loaders.first());
    let (loader, loader_version) = match primary {
        Some(loader) => curseforge_loader(&loader.id, &manifest.minecraft.version)?,
        None => (Loader::Vanilla, None),
    };

    Ok(Prepared {
        loader,
        mc_version: manifest.minecraft.version.clone(),
        loader_version,
        meta: curseforge::meta(&project, &file),
        archive,
        plan: Plan::Curseforge {
            manifest,
            mod_id,
            server_pack: file.server_pack_file_id,
        },
    })
}

/// "forge-47.2.0" -> Forge 47.2.0.
fn curseforge_loader(id: &str, mc_version: &str) -> AppResult<(Loader, Option<String>)> {
    let (name, version) = id
        .split_once('-')
        .ok_or_else(|| AppError::InvalidData(format!("mod loader {id:?}")))?;
    match name {
        "forge" => Ok((Loader::Forge, Some(version.to_string()))),
        // NeoForge for 1.20.1 was a Forge fork, published under another
        // name: its packs run on Forge.
        "neoforge" if mc_version == "1.20.1" => Ok((Loader::Forge, None)),
        "neoforge" => Ok((Loader::Neoforge, Some(version.to_string()))),
        "fabric" => Ok((Loader::Fabric, Some(version.to_string()))),
        _ => Err(AppError::ModpackUnsupported(name.to_string())),
    }
}

/// Puts the pack's files into the new server folder. Returns the files the
/// user has to download by hand (CurseForge only).
pub async fn install(
    http: &reqwest::Client,
    server_dir: &Path,
    pack: &Prepared,
    on_progress: impl FnMut(u8),
) -> AppResult<Vec<ManualDownload>> {
    match &pack.plan {
        Plan::Modrinth { files } => {
            install_mrpack(http, server_dir, &pack.archive.0, files, on_progress).await?;
            Ok(Vec::new())
        }
        Plan::Curseforge {
            manifest,
            mod_id,
            server_pack,
        } => {
            install_curseforge(
                http,
                server_dir,
                &pack.archive.0,
                manifest,
                *mod_id,
                *server_pack,
                on_progress,
            )
            .await
        }
    }
}

async fn install_mrpack(
    http: &reqwest::Client,
    server_dir: &Path,
    archive: &Path,
    files: &[MrpackFile],
    on_progress: impl FnMut(u8),
) -> AppResult<()> {
    // Names and icons for the content tab, and a second opinion on where
    // files run: packs often list client-only mods (texture features,
    // minimaps…) for servers too, which then crash at start.
    let sha1s: Vec<String> = files.iter().map(|file| file.hashes.sha1.clone()).collect();
    let mut identified = modrinth::identify(http, &sha1s)
        .await
        .unwrap_or_else(|err| {
            eprintln!("could not identify the modpack files: {err}");
            HashMap::new()
        });

    let mut jobs = Vec::new();
    let mut recorded = Vec::new();
    for file in files {
        let found = identified.remove(&file.hashes.sha1);
        if found.as_ref().is_some_and(|found| found.client_only) {
            continue;
        }
        let url = file
            .downloads
            .iter()
            .find(|url| is_mrpack_host(url))
            .ok_or_else(|| AppError::InvalidData(format!("download of {}", file.path)))?;
        let dest = server_dir.join(safe_relative_path(&file.path)?);
        if let Some(found) = found {
            recorded.push((dest.clone(), found.meta));
        }
        jobs.push(Job {
            url: url.clone(),
            dest,
            checksum: Some(Checksum::Sha1(file.hashes.sha1.clone())),
        });
    }
    download_all(http, &jobs, on_progress).await?;

    let (from, to) = (archive.to_path_buf(), server_dir.to_path_buf());
    blocking(move || {
        extract_folder(&from, "overrides", &to)?;
        extract_folder(&from, "server-overrides", &to)
    })
    .await?;
    record(server_dir, recorded)
}

fn is_mrpack_host(url: &str) -> bool {
    reqwest::Url::parse(url).is_ok_and(|url| {
        url.scheme() == "https"
            && url
                .host_str()
                .is_some_and(|host| MRPACK_HOSTS.contains(&host))
    })
}

async fn install_curseforge(
    http: &reqwest::Client,
    server_dir: &Path,
    archive: &Path,
    manifest: &CurseforgeManifest,
    mod_id: u32,
    server_pack: Option<u32>,
    mut on_progress: impl FnMut(u8),
) -> AppResult<Vec<ManualDownload>> {
    if let Some(pack_id) = server_pack {
        if install_server_pack(http, server_dir, mod_id, pack_id, &mut on_progress).await? {
            return Ok(Vec::new());
        }
    }

    let file_ids: Vec<u32> = manifest
        .files
        .iter()
        .filter(|file| file.required)
        .map(|file| file.file_id)
        .collect();
    let mut project_ids: Vec<u32> = manifest.files.iter().map(|f| f.project_id).collect();
    project_ids.sort_unstable();
    project_ids.dedup();
    let files = curseforge::files(http, &file_ids).await?;
    let projects: HashMap<u32, curseforge::Mod> = curseforge::projects(http, &project_ids)
        .await?
        .into_iter()
        .map(|project| (project.id, project))
        .collect();

    let mods_dir = content_dir(server_dir, ContentKind::Mod)?;
    let datapacks_dir = content_dir(server_dir, ContentKind::Datapack)?;
    let mut jobs = Vec::new();
    let mut recorded = Vec::new();
    let mut manual = Vec::new();
    for file in &files {
        if file.is_client_only() {
            continue;
        }
        let Some(project) = projects.get(&file.mod_id) else {
            continue;
        };
        // Resource packs, shaders… are only useful to players.
        let folder = match project.class_id {
            Some(CURSEFORGE_MODS) => &mods_dir,
            Some(CURSEFORGE_DATAPACKS) => &datapacks_dir,
            _ => continue,
        };
        let remote = curseforge::remote_file(file);
        let Some(url) = remote.url else {
            manual.push(ManualDownload {
                title: project.name.clone(),
                file_name: file.file_name.clone(),
                page_url: project.links.website_url.clone(),
            });
            continue;
        };
        let dest = folder.join(safe_file_name(&remote.file_name)?);
        recorded.push((dest.clone(), curseforge::meta(project, file)));
        jobs.push(Job {
            url,
            dest,
            checksum: remote.checksum,
        });
    }
    download_all(http, &jobs, on_progress).await?;

    let (from, to, overrides) = (
        archive.to_path_buf(),
        server_dir.to_path_buf(),
        manifest.overrides.clone(),
    );
    blocking(move || extract_folder(&from, &overrides, &to)).await?;
    record(server_dir, recorded)?;
    Ok(manual)
}

/// Unpacks the author's server pack. `false` if it cannot be downloaded or
/// holds no mods (some only contain a script fetching them at first start).
async fn install_server_pack(
    http: &reqwest::Client,
    server_dir: &Path,
    mod_id: u32,
    pack_id: u32,
    on_progress: &mut impl FnMut(u8),
) -> AppResult<bool> {
    let file = curseforge::file(http, mod_id, pack_id).await?;
    let remote = curseforge::remote_file(&file);
    let Some(url) = remote.url else {
        return Ok(false);
    };
    let archive = TempFile(server_dir.join(".server-pack.zip"));
    download_file(
        http,
        &url,
        &archive.0,
        remote.checksum.as_ref(),
        on_progress,
    )
    .await?;

    let (from, to) = (archive.0.clone(), server_dir.to_path_buf());
    blocking(move || extract_server_pack(&from, &to)).await?;
    Ok(has_mods(server_dir))
}

fn has_mods(server_dir: &Path) -> bool {
    std::fs::read_dir(server_dir.join("mods")).is_ok_and(|entries| {
        entries
            .filter_map(|entry| entry.ok())
            .any(|entry| entry.file_name().to_string_lossy().ends_with(".jar"))
    })
}

/// A file to download into the server folder.
struct Job {
    url: String,
    dest: PathBuf,
    checksum: Option<Checksum>,
}

/// Downloads every job, a few at a time; `on_progress` gets the share done.
async fn download_all(
    http: &reqwest::Client,
    jobs: &[Job],
    mut on_progress: impl FnMut(u8),
) -> AppResult<()> {
    let total = jobs.len();
    let semaphore = Arc::new(Semaphore::new(PARALLEL_DOWNLOADS));
    let mut tasks = JoinSet::new();
    for job in jobs {
        let (http, semaphore) = (http.clone(), semaphore.clone());
        let (url, dest, checksum) = (job.url.clone(), job.dest.clone(), job.checksum.clone());
        tasks.spawn(async move {
            let _permit = semaphore
                .acquire_owned()
                .await
                .map_err(|err| AppError::Io(std::io::Error::other(err)))?;
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let mut attempt = 1;
            loop {
                match download_file(&http, &url, &dest, checksum.as_ref(), |_| {}).await {
                    Err(AppError::Network(_)) if attempt < DOWNLOAD_ATTEMPTS => attempt += 1,
                    result => return result,
                }
            }
        });
    }

    let (mut done, mut last_percent) = (0, 0);
    on_progress(0);
    // An error drops `tasks`, which cancels the downloads still running.
    while let Some(result) = tasks.join_next().await {
        result.map_err(|err| AppError::Io(std::io::Error::other(err)))??;
        done += 1;
        let percent = (done * 100 / total) as u8;
        // Only report changes, to avoid flooding the UI with events.
        if percent != last_percent {
            last_percent = percent;
            on_progress(percent);
        }
    }
    Ok(())
}

/// "mods/sodium.jar" -> a path inside the server folder, or an error if it
/// tries to leave it.
fn safe_relative_path(path: &str) -> AppResult<PathBuf> {
    let mut relative = PathBuf::new();
    for part in path.split(['/', '\\']).filter(|part| !part.is_empty()) {
        relative.push(safe_file_name(part)?);
    }
    if relative.as_os_str().is_empty() {
        return Err(AppError::InvalidData(format!("path {path:?}")));
    }
    Ok(relative)
}

async fn read_json<T: serde::de::DeserializeOwned + Send + 'static>(
    archive: &Path,
    entry: &'static str,
) -> AppResult<T> {
    let archive = archive.to_path_buf();
    blocking(move || {
        let mut zip = zip::ZipArchive::new(std::fs::File::open(&archive)?)?;
        let mut text = String::new();
        zip.by_name(entry)?.read_to_string(&mut text)?;
        Ok(serde_json::from_str(&text)?)
    })
    .await
}

/// Unzipping is blocking work: keep it off the async threads.
async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> AppResult<T> + Send + 'static,
) -> AppResult<T> {
    tokio::task::spawn_blocking(work)
        .await
        .map_err(|err| AppError::Io(std::io::Error::other(err)))?
}

/// Copies the entries under `folder/` into `dest`.
fn extract_folder(archive: &Path, folder: &str, dest: &Path) -> AppResult<()> {
    extract(archive, dest, |path| {
        path.strip_prefix(folder).ok().map(Path::to_path_buf)
    })
}

/// Copies a server pack, without what MinyHost replaces: start scripts, the
/// jars at its root (loader, Minecraft) and `libraries/`.
fn extract_server_pack(archive: &Path, dest: &Path) -> AppResult<()> {
    let root = pack_root(&entry_names(archive)?);
    extract(archive, dest, |path| {
        let relative = match &root {
            Some(root) => path.strip_prefix(root).ok()?.to_path_buf(),
            None => path.to_path_buf(),
        };
        (!is_server_pack_leftover(&relative)).then_some(relative)
    })
}

fn entry_names(archive: &Path) -> AppResult<Vec<PathBuf>> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(archive)?)?;
    let mut names = Vec::new();
    for index in 0..zip.len() {
        if let Some(name) = zip.by_index(index)?.enclosed_name() {
            names.push(name);
        }
    }
    Ok(names)
}

/// The single folder wrapping a server pack ("MyPack-Server-1.2/"), if any.
fn pack_root(names: &[PathBuf]) -> Option<PathBuf> {
    let first = PathBuf::from(names.first()?.components().next()?.as_os_str());
    let wrapped = names.iter().all(|name| name.starts_with(&first))
        && names.iter().any(|name| {
            name.starts_with(first.join("mods")) || name.starts_with(first.join("config"))
        });
    wrapped.then_some(first)
}

fn is_server_pack_leftover(path: &Path) -> bool {
    let mut components = path.components();
    let Some(first) = components.next() else {
        return true;
    };
    let first = first.as_os_str().to_string_lossy().to_lowercase();
    if first == "libraries" {
        return true;
    }
    if components.next().is_some() {
        return false;
    }
    let is_program = [".jar", ".bat", ".sh", ".cmd", ".ps1", ".command", ".exe"]
        .iter()
        .any(|ext| first.ends_with(ext));
    is_program
        || matches!(
            first.as_str(),
            "eula.txt" | "user_jvm_args.txt" | "variables.txt"
        )
}

/// Writes the entries `map` keeps (renamed by it) into `dest`. Entries whose
/// name could escape `dest` are skipped.
fn extract(archive: &Path, dest: &Path, map: impl Fn(&Path) -> Option<PathBuf>) -> AppResult<()> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(archive)?)?;
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index)?;
        let Some(name) = entry.enclosed_name() else {
            continue;
        };
        let Some(relative) = map(&name).filter(|path| !path.as_os_str().is_empty()) else {
            continue;
        };
        let out = dest.join(relative);
        if entry.is_dir() {
            std::fs::create_dir_all(&out)?;
            continue;
        }
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::io::copy(&mut entry, &mut std::fs::File::create(&out)?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn zip_with(path: &Path, entries: &[(&str, &str)]) {
        let mut zip = zip::ZipWriter::new(std::fs::File::create(path).unwrap());
        for (name, content) in entries {
            zip.start_file(*name, zip::write::SimpleFileOptions::default())
                .unwrap();
            zip.write_all(content.as_bytes()).unwrap();
        }
        zip.finish().unwrap();
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("minyhost-modpack-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn reads_loaders() {
        let deps = |pairs: &[(&str, &str)]| {
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<_, _>>()
        };
        let (loader, version) = mrpack_loader(&deps(&[
            ("minecraft", "1.21.1"),
            ("fabric-loader", "0.16.5"),
        ]))
        .unwrap();
        assert_eq!(
            (loader, version.as_deref()),
            (Loader::Fabric, Some("0.16.5"))
        );
        assert!(mrpack_loader(&deps(&[("quilt-loader", "0.26")])).is_err());

        let (loader, version) = curseforge_loader("forge-47.2.0", "1.20.1").unwrap();
        assert_eq!(
            (loader, version.as_deref()),
            (Loader::Forge, Some("47.2.0"))
        );
        let (loader, version) = curseforge_loader("neoforge-21.1.77", "1.21.1").unwrap();
        assert_eq!(
            (loader, version.as_deref()),
            (Loader::Neoforge, Some("21.1.77"))
        );
        assert_eq!(
            curseforge_loader("neoforge-47.1.106", "1.20.1").unwrap().0,
            Loader::Forge
        );
    }

    #[test]
    fn pack_paths_stay_inside_the_server() {
        assert_eq!(
            safe_relative_path("mods/sodium.jar").unwrap(),
            PathBuf::from("mods").join("sodium.jar")
        );
        assert!(safe_relative_path("../evil.jar").is_err());
        assert!(safe_relative_path("mods/../../evil.jar").is_err());
        assert!(safe_relative_path("C:/evil.jar").is_err());
        assert!(safe_relative_path("").is_err());
        assert!(is_mrpack_host("https://cdn.modrinth.com/data/x.jar"));
        assert!(!is_mrpack_host("https://evil.example/x.jar"));
    }

    #[test]
    fn extracts_overrides_only() {
        let dir = temp_dir("overrides");
        let archive = dir.join("pack.zip");
        zip_with(
            &archive,
            &[
                ("modrinth.index.json", "{}"),
                ("overrides/config/a.toml", "a"),
                ("server-overrides/config/a.toml", "server"),
                ("../escape.txt", "x"),
            ],
        );
        let server = dir.join("server");
        extract_folder(&archive, "overrides", &server).unwrap();
        extract_folder(&archive, "server-overrides", &server).unwrap();
        assert_eq!(
            std::fs::read_to_string(server.join("config/a.toml")).unwrap(),
            "server"
        );
        assert!(!server.join("modrinth.index.json").exists());
        assert!(!dir.join("escape.txt").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn server_pack_keeps_content_only() {
        let dir = temp_dir("server-pack");
        let archive = dir.join("server.zip");
        zip_with(
            &archive,
            &[
                ("Pack-Server-1.0/startserver.bat", "java -jar forge.jar"),
                ("Pack-Server-1.0/forge-1.20.1-47.2.0.jar", "jar"),
                ("Pack-Server-1.0/eula.txt", "eula=true"),
                ("Pack-Server-1.0/server.properties", "level-type=skyblock"),
                ("Pack-Server-1.0/libraries/a.jar", "lib"),
                ("Pack-Server-1.0/mods/create.jar", "mod"),
                ("Pack-Server-1.0/config/create.toml", "cfg"),
            ],
        );
        let server = dir.join("server");
        extract_server_pack(&archive, &server).unwrap();
        assert!(server.join("mods/create.jar").exists());
        assert!(server.join("config/create.toml").exists());
        assert!(server.join("server.properties").exists());
        for leftover in [
            "startserver.bat",
            "forge-1.20.1-47.2.0.jar",
            "eula.txt",
            "libraries",
        ] {
            assert!(!server.join(leftover).exists(), "{leftover}");
        }
        assert!(has_mods(&server));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn unwrapped_server_pack_is_not_stripped() {
        let names = [PathBuf::from("mods/a.jar"), PathBuf::from("mods/b.jar")];
        assert_eq!(pack_root(&names), None);
    }
}
