#![allow(non_upper_case_globals)]

use fork::fork;
use fork::Fork;
use keyvalues_parser::Vdf;
use procfs;
use smartstring::alias::String as SmartString;
use tracing::trace;
use std::collections::BTreeMap;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tracing::debug;
use tracing::info;
use tracing::instrument;

use home::home_dir;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SteamApp {
    pub id: u32,
    pub name: SmartString,
    pub path: SmartString,
}

#[derive(Debug)]
pub struct AppProcess {
    pub app: SteamApp,
    pub process: procfs::process::Process,
    pub stat: procfs::process::Stat,
    pub cwd: PathBuf,
    pub exe: PathBuf,
}

impl SteamApp {
    #[instrument]
    pub fn all() -> BTreeMap<u32, SteamApp> {
        ALL_APPS.clone()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    #[instrument]
    pub fn launch(&self) -> AppProcess {
        if let Some(process) = self.find_process() {
            return process;
        }

        let mut command = Command::new("steam");
        command.arg(format!("steam://rungameid/{}", self.id()));
        command.stdin(Stdio::null());

        info!("Launching {:?} as {:?}", self.name, &command);

        // Double-fork to make sure we don't end up owning Steam's process.
        if matches!(fork().unwrap(), Fork::Child) {
            std::process::exit({
                fork::setsid().unwrap();
                // We don't fork::close_fd() because we still want to see errors
                // from the parent process, if it fails to spawn the child, but
                // we don't care about Steam/the game's output.

                if matches!(fork().unwrap(), Fork::Child) {
                    panic!("{:?}", command.exec())
                } else {
                    0
                }
            });
        }

        debug!("Waiting for {:?} to start...", self.name);

        loop {
            if let Some(process) = self.find_process() {
                return process;
            }
            sleep(Duration::from_millis(1024));
        }
    }

    #[instrument]
    pub fn find_process(&self) -> Option<AppProcess> {
        trace!("Checking for process...");
        for process in procfs::process::all_processes().unwrap() {
            let process = process.unwrap();
            if process.cwd().unwrap_or_default() == self.app_dir() {
                return Some(AppProcess {
                    app: self.clone(),
                    stat: process.stat().unwrap(),
                    cwd: process.cwd().unwrap(),
                    exe: process.exe().unwrap(),
                    process,
                });
            }
        }

        None
    }

    pub fn app_dir(&self) -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(".local");
        path.push("share");
        path.push("Steam");
        path.push("steamapps");
        path.push("common");
        path.push(self.path.as_str());
        path
    }

    pub fn saves_dir(&self) -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(".local");
        path.push("share");
        path.push(self.path.as_str());
        path.push("Saves");
        path
    }
}

impl AppProcess {
    pub fn still_alive(&self) -> bool {
        // if we can't read this file, we assume the process is terminated.
        self.process.stat().is_ok()
    }

    pub fn wait_for_exit(&self) {
        debug!("Waiting for {:?} to exit...", self.app.name);
        while self.still_alive() {
            sleep(Duration::from_millis(1024));
        }
    }
}

pub static STEAM_APPS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = home_dir().unwrap();
    path.push(".local");
    path.push("share");
    path.push("Steam");
    path.push("steamapps");
    assert!(path.is_dir(), "can't find steamapps");
    path
});

pub static ALL_APPS: Lazy<BTreeMap<u32, SteamApp>> = Lazy::new(|| {
    let mut all_apps = BTreeMap::new();

    // TODO: check all libraries, not just the primary one

    for entry in STEAM_APPS_DIR.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        if !file_name.starts_with("appmanifest_") {
            continue;
        }
        if !file_name.ends_with(".acf") {
            continue;
        }
        let id = u32::from_str(&file_name["appmanifest_".len()..file_name.len() - ".acf".len()])
            .unwrap();
        let manifest = std::fs::read_to_string(entry.path()).unwrap();
        let manifest = Vdf::parse(&manifest).unwrap();
        let manifest = manifest.value.clone();
        let manifest = manifest.get_obj().unwrap();
        dbg!(&manifest);
        let path = manifest
            .get("installdir")
            .clone()
            .unwrap()
            .get(0)
            .unwrap()
            .get_str()
            .unwrap()
            .to_string()
            .into();
        let name = manifest
            .get("name")
            .clone()
            .unwrap()
            .get(0)
            .unwrap()
            .get_str()
            .unwrap()
            .to_string()
            .into();

        all_apps.insert(id, SteamApp { id, path, name });
    }

    all_apps
});

pub static CELESTE: Lazy<&'static SteamApp> = Lazy::new(|| ALL_APPS.get(&504230).unwrap());
