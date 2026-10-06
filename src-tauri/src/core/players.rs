//! Mojang player profiles: pseudo -> UUID lookup, and skins.
//!
//! APIs (checked October 2026):
//! - `api.mojang.com/users/profiles/minecraft/<name>` -> `{ id, name }`, 404 if unknown,
//! - `sessionserver.mojang.com/session/minecraft/profile/<uuid>` -> skin URL (base64 JSON),
//! - `textures.minecraft.net/texture/<hash>` -> skin PNG (64x64).

use base64::Engine;
use serde::Deserialize;

use crate::download::get_json;
use crate::error::{AppError, AppResult};

const PROFILE_URL: &str = "https://api.mojang.com/users/profiles/minecraft";
const SESSION_URL: &str = "https://sessionserver.mojang.com/session/minecraft/profile";
const TEXTURES_HOST: &str = "textures.minecraft.net/texture/";
/// Steve's skin, shown for players without a custom skin. Downloaded from
/// Mojang at runtime, never bundled with the app (it is a Mojang asset).
const STEVE_SKIN_URL: &str =
    "https://textures.minecraft.net/texture/1a4af718455d4aab528e7a61f86fa25e6a369d1768dcb13f7df319a713eb810b";

/// A Minecraft account.
#[derive(Debug, Clone)]
pub struct Profile {
    /// With dashes, as in `whitelist.json` (`069a79f4-44e9-…`).
    pub uuid: String,
    /// Exact case, as registered at Mojang.
    pub name: String,
}

/// Minecraft pseudos: 3 to 16 letters, digits or underscores.
pub fn is_valid_name(name: &str) -> bool {
    (3..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Finds the account behind a pseudo.
pub async fn lookup(http: &reqwest::Client, name: &str) -> AppResult<Profile> {
    if !is_valid_name(name) {
        return Err(AppError::InvalidPlayerName(name.to_string()));
    }

    #[derive(Deserialize)]
    struct ProfileResponse {
        id: String,
        name: String,
    }

    let response = http.get(format!("{PROFILE_URL}/{name}")).send().await?;
    // 404 today, 204 in older versions of the API.
    if matches!(response.status().as_u16(), 204 | 404) {
        return Err(AppError::PlayerNotFound(name.to_string()));
    }
    let profile: ProfileResponse = response.error_for_status()?.json().await?;
    Ok(Profile {
        uuid: with_dashes(&profile.id),
        name: profile.name,
    })
}

/// The player's skin as a `data:image/png;base64,…` URL, ready for the UI.
/// Falls back to Steve when the player has no custom skin.
pub async fn skin_data_url(http: &reqwest::Client, uuid: &str) -> AppResult<String> {
    let url = custom_skin_url(http, uuid)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| STEVE_SKIN_URL.to_string());

    let bytes = http
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:image/png;base64,{encoded}"))
}

/// `None` when the player uses a default skin.
async fn custom_skin_url(http: &reqwest::Client, uuid: &str) -> AppResult<Option<String>> {
    #[derive(Deserialize)]
    struct Session {
        properties: Vec<Property>,
    }
    #[derive(Deserialize)]
    struct Property {
        name: String,
        value: String,
    }
    #[derive(Deserialize)]
    struct Textures {
        textures: TextureSet,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    struct TextureSet {
        skin: Option<Texture>,
    }
    #[derive(Deserialize)]
    struct Texture {
        url: String,
    }

    let session: Session =
        get_json(http, &format!("{SESSION_URL}/{}", uuid.replace('-', ""))).await?;
    let Some(property) = session
        .properties
        .into_iter()
        .find(|p| p.name == "textures")
    else {
        return Ok(None);
    };
    let json = base64::engine::general_purpose::STANDARD
        .decode(property.value)
        .map_err(|err| AppError::InvalidInput(format!("textures: {err}")))?;
    let textures: Textures = serde_json::from_slice(&json)?;

    // Mojang gives `http://` URLs; only accept its own texture server.
    Ok(textures
        .textures
        .skin
        .and_then(|skin| {
            skin.url
                .split_once(TEXTURES_HOST)
                .map(|(_, hash)| hash.to_string())
        })
        .map(|hash| format!("https://{TEXTURES_HOST}{hash}")))
}

/// `069a79f444e94726a5befca90e38aaf5` -> `069a79f4-44e9-4726-a5be-fca90e38aaf5`.
fn with_dashes(id: &str) -> String {
    if id.len() != 32 {
        return id.to_string();
    }
    format!(
        "{}-{}-{}-{}-{}",
        &id[0..8],
        &id[8..12],
        &id[12..16],
        &id[16..20],
        &id[20..32]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_pseudos() {
        assert!(is_valid_name("Marius80"));
        assert!(is_valid_name("a_b"));
        assert!(!is_valid_name("ab"));
        assert!(!is_valid_name("seventeen_chars__"));
        assert!(!is_valid_name("pseudo avec espace"));
        assert!(!is_valid_name("Élodie"));
    }

    #[test]
    fn adds_dashes_to_uuid() {
        assert_eq!(
            with_dashes("069a79f444e94726a5befca90e38aaf5"),
            "069a79f4-44e9-4726-a5be-fca90e38aaf5"
        );
    }
}
