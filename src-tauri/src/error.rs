//! Common error type, converted into a user-readable message for the frontend.
//!
//! Each variant is sent to the frontend as `{ code, detail }`:
//! - `code` selects the French message in `src/i18n/fr.ts` (`errors.<code>`),
//! - `detail` is the technical message, useful for logs and bug reports.
//!
//! To add an error: add a variant here, give it a code in `AppError::code`,
//! then add the matching text in `fr.ts` and the code in `src/types/index.ts`.

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("file error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid data: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid archive: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("invalid data: {0}")]
    InvalidData(String),

    #[error("checksum mismatch for {0}")]
    HashMismatch(String),

    #[error("server not found: {0}")]
    ServerNotFound(String),

    #[error("Minecraft version {0} is not available")]
    VersionUnavailable(String),

    #[error("no Java {0} runtime available for this system")]
    JavaUnavailable(u32),

    #[error("port {0} is already in use")]
    PortInUse(u16),

    #[error("server is already running")]
    AlreadyRunning,

    #[error("server is not running")]
    NotRunning,

    #[error("the Minecraft EULA has not been accepted")]
    EulaNotAccepted,

    #[error("missing file: {0}")]
    MissingFiles(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid Minecraft pseudo: {0}")]
    InvalidPlayerName(String),

    #[error("no Minecraft account named {0}")]
    PlayerNotFound(String),
}

impl AppError {
    /// Stable identifier used by the frontend to pick a translated message.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Network(_) => "network",
            Self::Io(_) => "io",
            Self::Json(_) => "invalidData",
            Self::Zip(_) => "invalidData",
            Self::InvalidData(_) => "invalidData",
            Self::HashMismatch(_) => "hashMismatch",
            Self::ServerNotFound(_) => "serverNotFound",
            Self::VersionUnavailable(_) => "versionUnavailable",
            Self::JavaUnavailable(_) => "javaUnavailable",
            Self::PortInUse(_) => "portInUse",
            Self::AlreadyRunning => "alreadyRunning",
            Self::NotRunning => "notRunning",
            Self::EulaNotAccepted => "eulaNotAccepted",
            Self::MissingFiles(_) => "missingFiles",
            Self::InvalidInput(_) => "invalidInput",
            Self::InvalidPlayerName(_) => "invalidPlayerName",
            Self::PlayerNotFound(_) => "playerNotFound",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("detail", &self.to_string())?;
        state.end()
    }
}
