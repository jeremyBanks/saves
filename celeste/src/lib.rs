use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod domutils;
mod save;
mod settings;

pub use crate::save::*;
pub use crate::settings::*;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Saves {
    pub path: PathBuf,
    pub saves: [Option<Save>; 3],
    pub settings: Settings,
}

impl Saves {
    /// Search for Saves files in common locations and loads them into a Vec.
    pub fn load_all() -> Vec<Saves> {
        let mut potential_paths: Vec<PathBuf> = Vec::new();

        {
            let mut path = dirs::home_dir().expect("to be able to find users' home directory");
            path.push("Library");
            path.push("Application Support");
            path.push("Celeste");
            path.push("Saves");
            potential_paths.push(path);
        }

        for root in &[r"C:\", "/mnt/c", r"D:\", "/mnt/d"] {
            let mut path = PathBuf::new();
            path.push(root);
            path.push("Program Files");
            path.push("Celeste");
            path.push("Saves");
            potential_paths.push(path);
        }

        let present_paths: Vec<_> = potential_paths
            .into_iter()
            .filter(|path| {
                let mut settings_path = PathBuf::new();
                settings_path.push(path);
                settings_path.push("settings.celeste");
                dbg!(&settings_path);
                dbg!(&settings_path.exists());
                settings_path.is_file()
            })
            .collect();

        present_paths.into_iter().map(Saves::load).collect()
    }

    /// Loads the Saves located at a given path.
    pub fn load(path: impl Into<PathBuf>) -> Saves {
        let path = path.into();

        let settings = std::fs::read_to_string({
            let mut settings_path = path.clone();
            settings_path.push("settings.celeste");
            settings_path
        })
        .map(|string| Settings::from_xml(&string))
        .expect("to be able to read the settings file");

        let mut saves = [None, None, None];
        for (i, item) in saves.iter_mut().enumerate() {
            *item = std::fs::read_to_string({
                let mut save_path = path.clone();
                save_path.push(format!("{}.celeste", i));
                save_path
            })
            .map(|string| Save::from_xml(&string))
            .ok();
        }

        Saves {
            path,
            saves,
            settings,
        }
    }
}
