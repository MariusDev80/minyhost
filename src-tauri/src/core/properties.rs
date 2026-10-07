//! Reading and writing `server.properties`.
//!
//! Lines are kept in order (comments included), so a file edited by hand or
//! by the server keeps its layout when we change one value.

use std::path::Path;

use crate::error::AppResult;

#[derive(Debug, Clone, Default)]
pub struct Properties {
    lines: Vec<Line>,
}

#[derive(Debug, Clone)]
enum Line {
    Entry {
        key: String,
        value: String,
    },
    /// Comment or blank line, kept as is.
    Other(String),
}

impl Properties {
    pub fn parse(text: &str) -> Self {
        let lines = text
            .lines()
            .map(|line| {
                let trimmed = line.trim_start();
                if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
                    return Line::Other(line.to_string());
                }
                match trimmed.split_once('=') {
                    Some((key, value)) => Line::Entry {
                        key: key.trim().to_string(),
                        value: value.to_string(),
                    },
                    None => Line::Other(line.to_string()),
                }
            })
            .collect();
        Self { lines }
    }

    pub fn load(path: &Path) -> AppResult<Self> {
        Ok(Self::parse(&std::fs::read_to_string(path)?))
    }

    pub fn save(&self, path: &Path) -> AppResult<()> {
        std::fs::write(path, self.to_text())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.lines.iter().find_map(|line| match line {
            Line::Entry { key: k, value } if k == key => Some(value.as_str()),
            _ => None,
        })
    }

    /// Like `get`, with Java escapes decoded (`é` -> `é`, `\:` -> `:`),
    /// for values shown to the user.
    pub fn get_text(&self, key: &str) -> Option<String> {
        self.get(key).map(unescape)
    }

    /// Updates `key`, or appends it if missing. Non-ASCII characters are
    /// escaped (`é`), which every Minecraft version reads correctly.
    pub fn set(&mut self, key: &str, value: &str) {
        let value = escape(value);
        for line in &mut self.lines {
            if let Line::Entry { key: k, value: v } = line {
                if k == key {
                    *v = value;
                    return;
                }
            }
        }
        self.lines.push(Line::Entry {
            key: key.to_string(),
            value,
        });
    }

    pub fn to_text(&self) -> String {
        let mut text = String::new();
        for line in &self.lines {
            match line {
                Line::Entry { key, value } => text.push_str(&format!("{key}={value}")),
                Line::Other(raw) => text.push_str(raw),
            }
            text.push('\n');
        }
        text
    }
}

/// Settings written when a server is created. Safe by default (CLAUDE.md):
/// whitelist on and enforced, online mode on.
pub fn defaults(port: u16, motd: &str) -> Properties {
    let mut properties = Properties::default();
    properties.set("motd", motd);
    properties.set("server-port", &port.to_string());
    properties.set("online-mode", "true");
    properties.set("white-list", "true");
    properties.set("enforce-whitelist", "true");
    properties
}

fn escape(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii() {
                c.to_string()
            } else {
                // Characters outside the BMP become a UTF-16 surrogate pair.
                c.encode_utf16(&mut [0; 2])
                    .iter()
                    .map(|unit| format!("\\u{unit:04x}"))
                    .collect()
            }
        })
        .collect()
}

/// Decodes the escapes Java writes in `.properties` files.
fn unescape(raw: &str) -> String {
    let mut units: Vec<u16> = Vec::new();
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            units.extend(c.encode_utf16(&mut [0; 2]).iter());
            continue;
        }
        match chars.next() {
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                units.push(u16::from_str_radix(&hex, 16).unwrap_or(u16::from(b'?')));
            }
            Some('t') => units.push(u16::from(b'\t')),
            Some('n') => units.push(u16::from(b'\n')),
            Some('r') => units.push(u16::from(b'\r')),
            Some(other) => units.extend(other.encode_utf16(&mut [0; 2]).iter()),
            None => {}
        }
    }
    // UTF-16 so that escaped surrogate pairs (emoji) decode correctly.
    String::from_utf16_lossy(&units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_text_decodes_escapes() {
        let mut properties = Properties::parse("level-type=minecraft\\:flat\n");
        properties.set("motd", "Été 🎉");
        assert_eq!(properties.get_text("motd").as_deref(), Some("Été 🎉"));
        assert_eq!(
            properties.get_text("level-type").as_deref(),
            Some("minecraft:flat")
        );
    }

    #[test]
    fn parse_keeps_comments_and_order() {
        let text = "#Minecraft server properties\nmotd=Hello\nserver-port=25565\n";
        let properties = Properties::parse(text);
        assert_eq!(properties.get("server-port"), Some("25565"));
        assert_eq!(properties.to_text(), text);
    }

    #[test]
    fn set_updates_or_appends() {
        let mut properties = Properties::parse("server-port=25565\n");
        properties.set("server-port", "25566");
        properties.set("pvp", "false");
        assert_eq!(properties.to_text(), "server-port=25566\npvp=false\n");
    }

    #[test]
    fn non_ascii_values_are_escaped() {
        let mut properties = Properties::default();
        properties.set("motd", "Été");
        assert_eq!(properties.get("motd"), Some("\\u00c9t\\u00e9"));
    }

    #[test]
    fn defaults_are_safe() {
        let properties = defaults(25570, "Test");
        assert_eq!(properties.get("online-mode"), Some("true"));
        assert_eq!(properties.get("white-list"), Some("true"));
        assert_eq!(properties.get("server-port"), Some("25570"));
    }
}
