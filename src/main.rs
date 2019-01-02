#![feature(try_from)]
use crate::{domutils::DomUtils, durationutils::DurationUtils};
use minidom::Element;
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::Duration};

mod domutils;
mod durationutils {
    pub trait DurationUtils {
        fn formatted(&self) -> String;
    }

    impl DurationUtils for std::time::Duration {
        fn formatted(&self) -> String {
            let mut pieces = String::new();

            let millis_left = self.as_millis();
            let millis = millis_left % 1000;
            let seconds_left = millis_left / 1000;
            let seconds = seconds_left % 60;
            let minutes_left = seconds_left / 60;
            let minutes = minutes_left % 60;
            let hours = minutes_left / 60;

            if !pieces.is_empty() {
                pieces.push_str(&format!("{:>02}h", hours));
            } else if hours > 0 {
                pieces.push_str(&format!("{:>2}h", hours));
            }

            if !pieces.is_empty() {
                pieces.push_str(&format!("{:>02}m", minutes));
            } else if minutes > 0 {
                pieces.push_str(&format!("{:>2}m", minutes));
            }

            if !pieces.is_empty() {
                pieces.push_str(&format!("{:>02}", seconds));
            } else if seconds > 0 {
                pieces.push_str(&format!("{:>2}", seconds));
            }

            if !pieces.is_empty() || millis > 0 {
                pieces.push_str(&format!(".{:>03}s", millis));
            } else {
                pieces.push_str("0 ");
            }

            format!("{:>13}", pieces)
        }
    }
}

fn main() {
    let saves = vec![include_str!("../0.celeste"), include_str!("../1.celeste")];

    for save in saves {
        let root = save.parse::<Element>().unwrap();
        let stats = Stats::from_save(&root);

        println!("{}", stats.name);
        for world_stats in stats.worlds {
            println!("  {}", world_stats.world);

            if world_stats.a_side.common.completed {
                print!("    A");
                if let Some(duration) = world_stats.a_side.common.single_run {
                    print!("   any%: {}", duration.formatted());
                    if !world_stats.has_winged_golden() {
                        print!(
                            "   min dashes: {:>3}",
                            world_stats.a_side.common.fewest_dashes.unwrap()
                        );
                    } else {
                        print!("   has winged berry");
                    }
                    if !world_stats.has_golden_a() {
                        print!(
                            "   min deaths: {:>3}",
                            world_stats.a_side.common.fewest_deaths.unwrap()
                        );
                    } else {
                        print!("   has golden berry");
                    }
                    println!();
                } else {
                    println!("   completed but not in a single run")
                }

                if let Some(duration) = world_stats.a_side.full_clear {
                    println!("    A   full: {}", duration.formatted());
                } else if world_stats.world.has_unlockables() {
                    if world_stats.world.red_berries() > 0 {
                        print!(
                            "    A   {:>2} / {:<2} red berries",
                            world_stats.a_side.common.berry_count(),
                            world_stats.world.red_berries()
                        );
                    } else {
                        print!("                           ");
                    }
                    if world_stats.a_side.cassette {
                        print!("   has cassette   ");
                    } else {
                        print!("   no  cassette   ");
                    }
                    if world_stats.a_side.cassette {
                        print!("   has crystal heart");
                    } else {
                        print!("   no  crystal heart");
                    }
                    println!();
                }
            }

            if world_stats.b_side.common.completed {
                print!("    B");

                if let Some(duration) = world_stats.b_side.common.single_run {
                    print!("   any%: {}", duration.formatted());
                    print!(
                        "   min dashes: {:>3}",
                        world_stats.b_side.common.fewest_dashes.unwrap()
                    );
                    if !world_stats.has_golden_b() {
                        print!(
                            "   min deaths: {:>3}",
                            world_stats.b_side.common.fewest_deaths.unwrap()
                        );
                    } else {
                        print!("   has golden berry");
                    }
                    println!();
                } else {
                    println!("   completed, but not in a single run")
                }
            }

            if world_stats.c_side.common.completed {
                print!("    C");

                if let Some(duration) = world_stats.c_side.common.single_run {
                    print!("   any%: {}", duration.formatted());
                    print!(
                        "   min dashes: {:>3}",
                        world_stats.c_side.common.fewest_dashes.unwrap()
                    );
                    if !world_stats.has_golden_c() {
                        print!(
                            "   min deaths: {:>3}",
                            world_stats.c_side.common.fewest_deaths.unwrap()
                        );
                    } else {
                        print!("   has golden berry");
                    }
                    println!();
                } else {
                    println!("   completed, but not in a single run")
                }
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub version: String,
    pub cheat_mode: bool,
    pub assist_mode: bool,
    pub variant_mode: bool,
    pub name: String,
    pub worlds: Vec<WorldStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldStats {
    pub world: World,
    pub a_side: ASideStats,
    pub b_side: BSideStats,
    pub c_side: CSideStats,
}

impl WorldStats {
    pub fn has_golden_a(&self) -> bool {
        self.a_side.common.berry_count() > self.world.red_berries()
    }

    pub fn has_golden_b(&self) -> bool {
        self.b_side.common.berry_count() > 0
    }

    pub fn has_golden_c(&self) -> bool {
        self.c_side.common.berry_count() > 0
    }

    pub fn has_winged_golden(&self) -> bool {
        self.a_side.common.berry_count() > self.world.red_berries() + 1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideStatsCommon {
    pub completed: bool,
    pub single_run: Option<Duration>,
    pub fewest_dashes: Option<u32>,
    pub fewest_deaths: Option<u32>,
    pub berries: BTreeSet<String>,
}

impl SideStatsCommon {
    pub fn berry_count(&self) -> u32 {
        self.berries.len() as u32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASideStats {
    pub cassette: bool,
    pub heart: bool,
    pub full_clear: Option<Duration>,
    pub common: SideStatsCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BSideStats {
    pub common: SideStatsCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSideStats {
    pub common: SideStatsCommon,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum World {
    Prologue,
    ForsakenCity,
    OldSite,
    CelestialResort,
    GoldenRidge,
    MirrorTemple,
    Reflection,
    TheSummit,
    Epilogue,
    Core,
}

pub use self::World::*;

impl World {
    pub fn name(self) -> &'static str {
        match self {
            Prologue => "Prologue",
            ForsakenCity => "Forsaken City",
            OldSite => "Old Site",
            CelestialResort => "Celestial Resort",
            GoldenRidge => "Golden Ridge",
            MirrorTemple => "Mirror Temple",
            Reflection => "Reflection",
            TheSummit => "The Summit",
            Epilogue => "Epilogue",
            Core => "Core",
        }
    }

    pub fn has_unlockables(self) -> bool {
        match self {
            Prologue | Epilogue => false,
            _ => true,
        }
    }

    pub fn red_berries(self) -> u32 {
        match self {
            Prologue | Reflection | Epilogue => 0,
            ForsakenCity => 20,
            OldSite => 18,
            CelestialResort => 25,
            GoldenRidge => 29,
            MirrorTemple => 31,
            TheSummit => 47,
            Core => 5,
        }
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<u32> for World {
    fn from(id: u32) -> Self {
        match id {
            0 => Prologue,
            1 => ForsakenCity,
            2 => OldSite,
            3 => CelestialResort,
            4 => GoldenRidge,
            5 => MirrorTemple,
            6 => Reflection,
            7 => TheSummit,
            8 => Epilogue,
            9 => Core,
            _ => panic!("unknown world ID"),
        }
    }
}

impl Into<u32> for World {
    fn into(self) -> u32 {
        match self {
            Prologue => 0,
            ForsakenCity => 1,
            OldSite => 2,
            CelestialResort => 3,
            GoldenRidge => 4,
            MirrorTemple => 5,
            Reflection => 6,
            TheSummit => 7,
            Epilogue => 8,
            Core => 9,
        }
    }
}

impl Stats {
    pub fn from_save(save_data: &minidom::Element) -> Self {
        assert!(save_data.name() == "SaveData");

        let version = save_data.expect_child("Version").text();
        assert!(version == "1.2.5.3");

        let name = save_data.expect_child("Name").text();

        let cheat_mode = save_data.expect_parse_child("CheatMode");
        let assist_mode = save_data.expect_parse_child("AssistMode");
        let variant_mode = save_data.expect_parse_child("VariantMode");

        let worlds = save_data
            .expect_child("Areas")
            .children()
            .map(WorldStats::from_save)
            // .filter(|stats| stats.world.has_unlockables())
            .collect();

        Self {
            version,
            name,
            cheat_mode,
            assist_mode,
            variant_mode,
            worlds,
        }
    }
}

impl WorldStats {
    pub fn from_save(area_stats: &minidom::Element) -> Self {
        assert!(area_stats.name() == "AreaStats");

        let world = area_stats.expect_parse_attr::<u32>("ID").into();

        let modes = area_stats
            .expect_child("Modes")
            .children()
            .collect::<Vec<_>>();
        assert!(modes.len() == 3);

        let sides_common = modes
            .iter()
            .map(|area_mode_stats| SideStatsCommon::from_save(area_mode_stats))
            .collect::<Vec<_>>();

        let a_side = ASideStats {
            cassette: area_stats.expect_parse_attr::<bool>("Cassette"),
            heart: modes[0].expect_parse_attr::<bool>("HeartGem"),
            full_clear: {
                let decimicroseconds: u64 = modes[0].expect_parse_attr("BestFullClearTime");
                if decimicroseconds == 0 {
                    None
                } else {
                    Some(Duration::from_nanos(decimicroseconds * 100))
                }
            },
            common: sides_common[0].clone(),
        };
        let b_side = BSideStats {
            common: sides_common[1].clone(),
        };
        let c_side = CSideStats {
            common: sides_common[2].clone(),
        };

        Self {
            world,
            a_side,
            b_side,
            c_side,
        }
    }
}

impl SideStatsCommon {
    pub fn from_save(area_mode_stats: &minidom::Element) -> Self {
        assert!(area_mode_stats.name() == "AreaModeStats");

        let completed = area_mode_stats.expect_parse_attr("Completed");

        let single_run;
        let fewest_dashes;
        let fewest_deaths;

        let single_run_completed_attr = area_mode_stats.attr("SingleRunCompleted");
        let single_run_completed = single_run_completed_attr == Some("true");
        if single_run_completed {
            single_run = {
                let decimicroseconds: u64 = area_mode_stats.expect_parse_attr("BestTime");
                Some(Duration::from_nanos(decimicroseconds * 100))
            };
            fewest_dashes = Some(area_mode_stats.expect_parse_attr("BestDashes"));
            fewest_deaths = Some(area_mode_stats.expect_parse_attr("BestDeaths"));
        } else {
            single_run = None;
            fewest_dashes = None;
            fewest_deaths = None;
        };

        let berries = area_mode_stats
            .expect_child("Strawberries")
            .children()
            .map(|entity_id| entity_id.attr("Key").unwrap().to_string())
            .collect::<BTreeSet<_>>();

        Self {
            completed,
            single_run,
            fewest_dashes,
            fewest_deaths,
            berries,
        }
    }
}
