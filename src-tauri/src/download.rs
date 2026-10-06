//! HTTP helpers: shared client, JSON requests and file downloads with hash
//! verification.

use std::path::Path;

use serde::de::DeserializeOwned;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::error::{AppError, AppResult};

/// Identifies the app to external APIs (required by Modrinth, PaperMC…).
const USER_AGENT: &str = concat!(
    "MinyHost/",
    env!("CARGO_PKG_VERSION"),
    " (github.com/MariusDev80/minyhost)"
);

/// Expected hash of a downloaded file, as given by the API (hex string).
#[derive(Debug, Clone)]
pub enum Checksum {
    Sha1(String),
    Sha256(String),
}

/// Client used for every outgoing request. Build it once and share it.
pub fn http_client() -> AppResult<reqwest::Client> {
    Ok(reqwest::Client::builder().user_agent(USER_AGENT).build()?)
}

/// GET a URL and parse its JSON body.
pub async fn get_json<T: DeserializeOwned>(http: &reqwest::Client, url: &str) -> AppResult<T> {
    let response = http.get(url).send().await?.error_for_status()?;
    Ok(response.json().await?)
}

/// Downloads `url` to `dest`, checking `checksum` when the API provides one.
///
/// The file is written to `<dest>.part` and only renamed once complete and
/// verified, so `dest` never holds a broken file. `on_progress` receives the
/// percentage (0-100) when the size is known.
pub async fn download_file(
    http: &reqwest::Client,
    url: &str,
    dest: &Path,
    checksum: Option<&Checksum>,
    mut on_progress: impl FnMut(u8),
) -> AppResult<()> {
    let mut response = http.get(url).send().await?.error_for_status()?;
    let total = response.content_length();

    let part_path = dest.with_extension("part");
    let mut file = tokio::fs::File::create(&part_path).await?;
    let mut hasher = Hasher::for_checksum(checksum);
    let mut downloaded: u64 = 0;
    let mut last_percent = None;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;

        if let Some(total) = total.filter(|total| *total > 0) {
            let percent = (downloaded * 100 / total).min(100) as u8;
            // Only report changes, to avoid flooding the UI with events.
            if last_percent != Some(percent) {
                last_percent = Some(percent);
                on_progress(percent);
            }
        }
    }
    file.flush().await?;
    drop(file);

    if let Some(expected) = checksum {
        if !hasher.matches(expected) {
            let _ = tokio::fs::remove_file(&part_path).await;
            return Err(AppError::HashMismatch(url.to_string()));
        }
    }

    tokio::fs::rename(&part_path, dest).await?;
    Ok(())
}

/// Computes the hash while the file is being downloaded.
enum Hasher {
    None,
    Sha1(Sha1),
    Sha256(Sha256),
}

impl Hasher {
    fn for_checksum(checksum: Option<&Checksum>) -> Self {
        match checksum {
            None => Self::None,
            Some(Checksum::Sha1(_)) => Self::Sha1(Sha1::new()),
            Some(Checksum::Sha256(_)) => Self::Sha256(Sha256::new()),
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::None => {}
            Self::Sha1(hasher) => hasher.update(bytes),
            Self::Sha256(hasher) => hasher.update(bytes),
        }
    }

    fn matches(self, expected: &Checksum) -> bool {
        let actual = match self {
            Self::None => return false,
            Self::Sha1(hasher) => to_hex(&hasher.finalize()),
            Self::Sha256(hasher) => to_hex(&hasher.finalize()),
        };
        let expected = match expected {
            Checksum::Sha1(hex) | Checksum::Sha256(hex) => hex,
        };
        actual.eq_ignore_ascii_case(expected)
    }
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_value() {
        let mut hasher = Hasher::for_checksum(Some(&Checksum::Sha256(String::new())));
        hasher.update(b"abc");
        let expected = Checksum::Sha256(
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD".into(),
        );
        assert!(hasher.matches(&expected));
    }

    #[test]
    fn sha1_detects_mismatch() {
        let mut hasher = Hasher::for_checksum(Some(&Checksum::Sha1(String::new())));
        hasher.update(b"abc");
        assert!(!hasher.matches(&Checksum::Sha1("0000".into())));
    }
}
