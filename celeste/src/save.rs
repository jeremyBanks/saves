use crate::domutils::DomUtils;
use derive_more::Display;
use minidom::Element;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Save {
    pub version: [u8; 4],
    pub name: String,
    pub play_time: Duration,
    pub game_speed: u8,
    pub unlocked_areas: u8,
    pub total_deaths: u32,
    pub total_strawberries: u8,
    pub total_golden_strawberries: u8,
    pub total_jumps: u32,
    pub total_wall_jumps: u32,
    pub total_dashes: u32,
}

pub struct AreaStats {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Copy, Display, PartialEq, Eq)]
pub enum Area {
    Prologue,
    #[display(fmt = "Forsaken City")]
    ForsakenCity,
    #[display(fmt = "Old Site")]
    OldSite,
    #[display(fmt = "Celestial Resort")]
    CelestialResort,
    #[display(fmt = "Golden Ridge")]
    GoldenRidge,
    #[display(fmt = "Mirror Temple")]
    MirrorTemple,
    Reflection,
    #[display(fmt = "The Summit")]
    TheSummit,
    Epilogue,
    Core,
    Farewell,
}

pub struct PrologueArea {}

impl Save {
    pub fn from_xml(string: &str) -> Self {
        let root: Element = string.parse().unwrap();
        assert!(root.name() == "SaveData");

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

        let name = root.expect_child("Name").text();

        Self { version, name }
    }
}

impl Area {
    /// The number of this Area in Celete's world selection screen.
    /// Or None if this Area is the un-numbered Prologue or Epilogue.
    pub fn world_number(self) -> Option<u8> {
        match self {
            Area::ForsakenCity => Some(1),
            Area::OldSite => Some(2),
            Area::CelestialResort => Some(3),
            Area::GoldenRidge => Some(4),
            Area::MirrorTemple => Some(5),
            Area::Reflection => Some(6),
            Area::TheSummit => Some(7),
            Area::Core => Some(8),
            Area::Farewell => Some(9),
            Area::Prologue => None,
            Area::Epilogue => None,
        }
    }

    /// The ID of this Area in Celete's save data.
    pub fn id(self) -> u8 {
        match self {
            Area::Prologue => 0,
            Area::ForsakenCity => 1,
            Area::OldSite => 2,
            Area::CelestialResort => 3,
            Area::GoldenRidge => 4,
            Area::MirrorTemple => 5,
            Area::Reflection => 6,
            Area::TheSummit => 7,
            Area::Epilogue => 8,
            Area::Core => 9,
            Area::Farewell => 10,
        }
    }

    /// Returns the Area for an ID used in Celeste's save data.
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Area::Prologue,
            1 => Area::ForsakenCity,
            2 => Area::OldSite,
            3 => Area::CelestialResort,
            4 => Area::GoldenRidge,
            5 => Area::MirrorTemple,
            6 => Area::Reflection,
            7 => Area::TheSummit,
            8 => Area::Epilogue,
            9 => Area::Core,
            10 => Area::Farewell,
            _ => panic!("unknown Area ID"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Save;

    const SAVE_A_0: &str = include_str!("../test-data/a/0.celeste");
    const SAVE_A_1: &str = include_str!("../test-data/a/1.celeste");
    const SAVE_A_2: &str = include_str!("../test-data/a/2.celeste");

    #[test]
    fn loads_version() {
        let saves = [
            Save::from_xml(SAVE_A_0),
            Save::from_xml(SAVE_A_1),
            Save::from_xml(SAVE_A_2),
        ];
        assert_eq!([1, 2, 6, 1], saves[0].version);
        assert_eq!([1, 2, 6, 1], saves[1].version);
        assert_eq!([1, 2, 5, 3], saves[2].version);
    }
}
