//! Minimal, read-only reader for NBT, the binary format of Minecraft save
//! files (`level.dat`, `game_rules.dat`…). Spec: https://minecraft.wiki/w/NBT_format
//!
//! A file is one named tag (usually a compound), often gzip-compressed.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;

use crate::error::{AppError, AppResult};

/// Nesting limit, so a corrupted file cannot overflow the stack.
const MAX_DEPTH: usize = 512;

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(BTreeMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    /// Child of a compound (`None` for other tags or missing keys).
    pub fn get(&self, key: &str) -> Option<&Tag> {
        match self {
            Tag::Compound(map) => map.get(key),
            _ => None,
        }
    }

    pub fn as_compound(&self) -> Option<&BTreeMap<String, Tag>> {
        match self {
            Tag::Compound(map) => Some(map),
            _ => None,
        }
    }
}

/// Reads an NBT file, gzip-compressed or not.
pub fn read_file(path: &Path) -> AppResult<Tag> {
    let raw = std::fs::read(path)?;
    let is_gzip = raw.starts_with(&[0x1f, 0x8b]);
    let bytes = if is_gzip {
        let mut out = Vec::new();
        GzDecoder::new(raw.as_slice()).read_to_end(&mut out)?;
        out
    } else {
        raw
    };
    parse(&bytes)
}

/// Parses the root tag (its name is ignored).
pub fn parse(bytes: &[u8]) -> AppResult<Tag> {
    let mut reader = Reader { bytes, pos: 0 };
    let tag_type = reader.u8()?;
    reader.string()?;
    reader.payload(tag_type, 0)
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> AppResult<&[u8]> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|end| *end <= self.bytes.len());
        let end = end.ok_or_else(|| invalid("unexpected end of data"))?;
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn array<const N: usize>(&mut self) -> AppResult<[u8; N]> {
        let mut out = [0; N];
        out.copy_from_slice(self.take(N)?);
        Ok(out)
    }

    fn u8(&mut self) -> AppResult<u8> {
        Ok(self.take(1)?[0])
    }

    fn i32(&mut self) -> AppResult<i32> {
        Ok(i32::from_be_bytes(self.array()?))
    }

    /// Length prefix of arrays and lists (negative means empty).
    fn len(&mut self) -> AppResult<usize> {
        Ok(usize::try_from(self.i32()?).unwrap_or(0))
    }

    fn string(&mut self) -> AppResult<String> {
        let len = usize::from(u16::from_be_bytes(self.array()?));
        // Java's "modified UTF-8": identical to UTF-8 for everyday text.
        Ok(String::from_utf8_lossy(self.take(len)?).into_owned())
    }

    fn payload(&mut self, tag_type: u8, depth: usize) -> AppResult<Tag> {
        if depth > MAX_DEPTH {
            return Err(invalid("nesting too deep"));
        }
        Ok(match tag_type {
            1 => Tag::Byte(i8::from_be_bytes(self.array()?)),
            2 => Tag::Short(i16::from_be_bytes(self.array()?)),
            3 => Tag::Int(self.i32()?),
            4 => Tag::Long(i64::from_be_bytes(self.array()?)),
            5 => Tag::Float(f32::from_be_bytes(self.array()?)),
            6 => Tag::Double(f64::from_be_bytes(self.array()?)),
            7 => {
                let len = self.len()?;
                Tag::ByteArray(self.take(len)?.iter().map(|b| *b as i8).collect())
            }
            8 => Tag::String(self.string()?),
            9 => {
                let item_type = self.u8()?;
                let len = self.len()?;
                let mut items = Vec::new();
                for _ in 0..len {
                    items.push(self.payload(item_type, depth + 1)?);
                }
                Tag::List(items)
            }
            10 => {
                let mut map = BTreeMap::new();
                loop {
                    let child_type = self.u8()?;
                    if child_type == 0 {
                        break;
                    }
                    let name = self.string()?;
                    map.insert(name, self.payload(child_type, depth + 1)?);
                }
                Tag::Compound(map)
            }
            11 => {
                let len = self.len()?;
                let mut items = Vec::new();
                for _ in 0..len {
                    items.push(self.i32()?);
                }
                Tag::IntArray(items)
            }
            12 => {
                let len = self.len()?;
                let mut items = Vec::new();
                for _ in 0..len {
                    items.push(i64::from_be_bytes(self.array()?));
                }
                Tag::LongArray(items)
            }
            other => return Err(invalid(&format!("unknown tag type {other}"))),
        })
    }
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidData(format!("NBT: {reason}"))
}

/// Builds NBT bytes in tests (the app itself never writes NBT).
#[cfg(test)]
pub mod test_writer {
    pub fn named(tag_type: u8, name: &str, payload: &[u8]) -> Vec<u8> {
        let mut out = vec![tag_type];
        out.extend((name.len() as u16).to_be_bytes());
        out.extend(name.as_bytes());
        out.extend(payload);
        out
    }

    pub fn compound(children: &[Vec<u8>]) -> Vec<u8> {
        let mut out: Vec<u8> = children.concat();
        out.push(0);
        out
    }

    pub fn string(text: &str) -> Vec<u8> {
        let mut out = (text.len() as u16).to_be_bytes().to_vec();
        out.extend(text.as_bytes());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::test_writer::*;
    use super::*;

    #[test]
    fn reads_nested_compounds() {
        let bytes = named(
            10,
            "",
            &compound(&[
                named(1, "flag", &[1]),
                named(3, "count", &42i32.to_be_bytes()),
                named(10, "inner", &compound(&[named(8, "text", &string("hé"))])),
            ]),
        );
        let root = parse(&bytes).unwrap();
        assert_eq!(root.get("flag"), Some(&Tag::Byte(1)));
        assert_eq!(root.get("count"), Some(&Tag::Int(42)));
        assert_eq!(
            root.get("inner").and_then(|inner| inner.get("text")),
            Some(&Tag::String("hé".into()))
        );
    }

    #[test]
    fn truncated_data_is_an_error() {
        let bytes = named(10, "", &[3, 0, 1, b'x', 0]);
        assert!(parse(&bytes).is_err());
    }
}
