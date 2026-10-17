//! Forge and NeoForge official installers, and how to start what they install.
//!
//! Both loaders ship an installer jar that, run with `--installServer`,
//! downloads Minecraft and the libraries, then patches the game. What it
//! leaves behind depends on the version:
//! - Forge 1.17+ and NeoForge: a `libraries/` tree and a `run.bat` / `run.sh`
//!   whose `java` line holds the arguments to start the server,
//! - older Forge: a runnable `forge-<version>.jar`, renamed `server.jar` here.
//!
//! `launch_args` turns either layout into the arguments given to `java`.

use std::path::Path;
use std::process::Stdio;

use crate::download::{get_text, Checksum};
use crate::error::{AppError, AppResult};
use crate::paths::server_files;

/// Name of the downloaded installer in the server folder (removed once run).
pub const INSTALLER_JAR: &str = "installer.jar";

/// How many lines of the installer output are kept in the error.
const ERROR_TAIL_LINES: usize = 15;

/// Maven repositories publish the SHA-1 of each file next to it (`<file>.sha1`).
pub async fn maven_checksum(http: &reqwest::Client, url: &str) -> AppResult<Checksum> {
    let text = get_text(http, &format!("{url}.sha1")).await?;
    // Some repositories append the file name after the hash.
    let hash = text.split_whitespace().next().unwrap_or_default();
    if hash.len() != 40 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AppError::InvalidData(format!("checksum of {url}")));
    }
    Ok(Checksum::Sha1(hash.to_string()))
}

/// Runs the installer downloaded into `dir`, then removes it.
pub async fn run(java: &Path, dir: &Path) -> AppResult<()> {
    let mut command = tokio::process::Command::new(java);
    command
        .args(["-jar", INSTALLER_JAR, "--installServer"])
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let output = command.output().await?;

    if !output.status.success() {
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let lines: Vec<&str> = text.lines().collect();
        let tail = lines[lines.len().saturating_sub(ERROR_TAIL_LINES)..].join("\n");
        return Err(AppError::LoaderInstall(tail));
    }

    for leftover in [INSTALLER_JAR.to_string(), format!("{INSTALLER_JAR}.log")] {
        let _ = tokio::fs::remove_file(dir.join(leftover)).await;
    }
    rename_legacy_jar(dir)
}

/// Older Forge installs a runnable `forge-<mc>-<version>.jar`: name it
/// `server.jar` like every other server type.
fn rename_legacy_jar(dir: &Path) -> AppResult<()> {
    if dir.join(start_script()).exists() {
        return Ok(());
    }
    let jar = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .find(|name| {
            name.starts_with("forge-") && name.ends_with(".jar") && !name.contains("installer")
        })
        .ok_or_else(|| AppError::MissingFiles(dir.join("forge-*.jar").display().to_string()))?;
    std::fs::rename(dir.join(jar), dir.join(server_files::JAR))?;
    Ok(())
}

/// Start script written by the installer for this OS.
fn start_script() -> &'static str {
    if cfg!(windows) {
        "run.bat"
    } else {
        "run.sh"
    }
}

/// What follows the memory options on the `java` command line: `-jar
/// server.jar`, or the arguments of the installer's start script.
pub fn launch_args(dir: &Path) -> AppResult<Vec<String>> {
    if dir.join(server_files::JAR).exists() {
        return Ok(vec!["-jar".into(), server_files::JAR.into()]);
    }
    let script = dir.join(start_script());
    let text = std::fs::read_to_string(&script)
        .map_err(|_| AppError::MissingFiles(dir.join(server_files::JAR).display().to_string()))?;
    parse_start_script(&text).ok_or_else(|| AppError::InvalidData(script.display().to_string()))
}

/// Arguments of the last `java` line (recent Forge scripts first run a Java
/// version check), without the ones meant for the user (`@user_jvm_args.txt`,
/// the script's own arguments).
fn parse_start_script(text: &str) -> Option<Vec<String>> {
    let line = text
        .lines()
        .rev()
        .map(str::trim)
        .find(|line| line.starts_with("java "))?;
    let args: Vec<String> = line
        .split_whitespace()
        .skip(1)
        .filter(|arg| !matches!(*arg, "@user_jvm_args.txt" | "%*" | "\"$@\"" | "$@"))
        .map(String::from)
        .collect();
    (!args.is_empty()).then_some(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_java_line_of_neoforge_scripts() {
        let bat = "@echo off\r\nREM Forge requires a configured set of both JVM and program arguments.\r\n\
                   java @user_jvm_args.txt @libraries/net/neoforged/neoforge/21.1.209/win_args.txt %*\r\npause\r\n";
        assert_eq!(
            parse_start_script(bat).unwrap(),
            vec!["@libraries/net/neoforged/neoforge/21.1.209/win_args.txt"]
        );
        let sh =
            "#!/usr/bin/env sh\njava @user_jvm_args.txt -jar forge-1.20.4-49.0.3-shim.jar \"$@\"\n";
        assert_eq!(
            parse_start_script(sh).unwrap(),
            vec!["-jar", "forge-1.20.4-49.0.3-shim.jar"]
        );
        let forge = "@echo off\r\njava -jar forge-26.3-66.0.10-shim.jar --onlyCheckJava\r\n\
                     if %ERRORLEVEL% NEQ 0 (\r\n    goto :exit\r\n)\r\n\
                     java @user_jvm_args.txt @libraries/net/minecraftforge/forge/26.3-66.0.10/win_args.txt %*\r\n\
                     :exit\r\npause\r\n";
        assert_eq!(
            parse_start_script(forge).unwrap(),
            vec!["@libraries/net/minecraftforge/forge/26.3-66.0.10/win_args.txt"]
        );
        assert!(parse_start_script("@echo off\npause\n").is_none());
    }

    #[test]
    fn plain_jar_comes_first() {
        let dir = std::env::temp_dir().join("minyhost-launch-args-test");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(server_files::JAR), b"").unwrap();
        assert_eq!(launch_args(&dir).unwrap(), vec!["-jar", "server.jar"]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
