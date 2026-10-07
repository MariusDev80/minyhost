//! Comparing Minecraft versions ("1.21.10" < "1.21.11" < "26.1").

/// `true` if `version` is `min` or newer.
pub fn at_least(version: &str, min: &str) -> bool {
    parse(version) >= parse(min)
}

/// "1.21.10" -> [1, 21, 10]. Missing parts count as 0 ("1.19" == "1.19.0").
fn parse(version: &str) -> [u32; 3] {
    // Pre-releases ("1.21.11-rc1") compare like their release.
    let release = version.split('-').next().unwrap_or(version);
    let mut parts = [0; 3];
    for (slot, part) in parts.iter_mut().zip(release.split('.')) {
        *slot = part.parse().unwrap_or(0);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_numerically() {
        assert!(at_least("1.21.10", "1.21.9"));
        assert!(!at_least("1.21.9", "1.21.10"));
        assert!(at_least("1.19", "1.19.0"));
        assert!(at_least("26.1", "1.21.11"));
        assert!(!at_least("1.12.2", "1.13.2"));
    }
}
