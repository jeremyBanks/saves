use home::home_dir;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tracing_unwrap::OptionExt;

pub static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| home_dir().unwrap_or_log());

pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| HOME_DIR.join(".".to_string() + crate::NAME));

pub static BIN_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("bin"));

pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("log"));

pub static ETC_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("etc"));

pub static GIT_DIR: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("git"));

pub static STEAM_DIR: Lazy<PathBuf> = Lazy::new(|| HOME_DIR.join(".local/share/Steam"));

pub static STEAM_USER_DATA_DIR: Lazy<PathBuf> = Lazy::new(|| STEAM_DIR.join("userdata"));
