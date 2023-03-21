use crate::domutils::DomUtils;
use minidom::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Settings {
    pub version: [u8; 4],
    pub last_save_id: u8,
}
impl Settings {
    pub fn from_xml(string: &str) -> Self {
        let root: Element = string.parse().unwrap();
        assert!("Settings" == root.name());

        let version_str = root.expect_child("Version").text();
        let version_vec: Vec<u8> = version_str
            .split('.')
            .map(|part| part.parse().unwrap())
            .collect();
        assert!(version_vec.len() == 4);
        let version = [
            version_vec[0],
            version_vec[1],
            version_vec[2],
            version_vec[3],
        ];

        let last_save_id = root.expect_parse_child("LastSaveFile");

        Self {
            version,
            last_save_id,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Settings;

    const SETTINGS_A: &str = include_str!("../test-data/a/settings.celeste");

    #[test]
    fn loads_version() {
        let settings = Settings::from_xml(SETTINGS_A);
        assert_eq!([1, 2, 6, 1], settings.version);
    }
}
