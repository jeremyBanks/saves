#![allow(unused)]
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use fork::Fork;
use fork::daemon;
use fork::fork;
use tracing::instrument;
use tracing::trace;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;

mod domutils;
mod durationutils;
mod old_main;
mod stringutils;
use home::home_dir;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CelesteProcess {
    pub pid: i32,
    pub timestamp: u64,
    pub cwd: PathBuf,
    pub exe: PathBuf,
}

impl CelesteProcess {
    #[instrument]
    pub fn all() -> BTreeSet<CelesteProcess> {
        let mut celeste_processes = BTreeSet::new();

        for process in procfs::process::all_processes().unwrap() {
            let process = process.unwrap();
            let pid = process.pid();

            let Ok(cwd) = process.cwd() else { continue };
            let cwd_file_name = cwd.file_name().unwrap_or_default();
            if cwd_file_name != "Celeste" {
                continue;
            }

            let Ok(exe) = process.exe() else { continue };
            let exe_file_name = exe.file_name().unwrap_or_default().to_string_lossy();
            if !exe_file_name.starts_with("Celeste") {
                continue;
            }

            let Ok(stat) = process.stat() else { continue };
            let timestamp = stat.starttime;

            celeste_processes.insert(CelesteProcess {
                pid,
                timestamp,
                cwd,
                exe,
            });
        }

        celeste_processes
    }

    #[instrument]
    pub fn get() -> Option<Self> {
        for process in Self::all() {
            return Some(process);
        }
        None
    }

    #[instrument]
    pub fn get_or_new() -> Self {
        Self::get().unwrap_or_else(Self::new)
    }

    #[instrument]
    pub fn new() -> Self {
        info!("Launching Celeste...");

        // Double-fork to make sure we don't end up owning Steam's process.
        if matches!(fork().unwrap(), Fork::Child) {
            std::process::exit({
                fork::setsid().unwrap();
                fork::chdir().unwrap();
                // We don't fork::close_fd() because we still want to see errors
                // from the parent process, if it fails to spawn the child, but
                // we don't care about Steam/the game's output.

                if matches!(fork().unwrap(), Fork::Child) {
                    Command::new("steam")
                        .arg("steam://rungameid/504230")
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .unwrap()
                        .code()
                        .unwrap()
                } else {
                    0
                }
            });
        }

        Self::wait_for_new()
    }

    #[instrument]
    pub fn wait_for_new() -> Self {
        loop {
            for process in Self::all() {
                return process;
            }
            debug!("waiting for Celeste to start");
            sleep(Duration::from_millis(1024));
        }
    }

    #[instrument]
    pub fn still_alive(&self) -> bool {
        let pid = self.pid;
        let Ok(processes) = procfs::process::Process::new(pid) else { return false };
        let Ok(stat) = processes.stat() else { return false };
        let timestamp = stat.starttime;
        self.timestamp == timestamp
            && self.cwd == processes.cwd().unwrap_or_default()
            && self.exe == processes.exe().unwrap_or_default()
    }

    #[instrument]
    pub fn wait_for_exit(&self) {
        loop {
            if !self.still_alive() {
                return;
            }
            debug!("waiting for Celeste to exit");
            sleep(Duration::from_millis(1024));
        }
    }
}

fn main() {
    let file_appender = tracing_appender::rolling::hourly(LOG_DIR.clone(), "log");
    let (file_appender, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::EnvFilter::from_default_env()),
        )
        .init();

    trace!("env = {:#?}", std::env::vars().collect::<IndexMap<_, _>>());

    info!("Launching or finding Celeste...");

    let celeste = CelesteProcess::get_or_new();

    info!("Celeste is now running. {celeste:?}");

    celeste.wait_for_exit();

    info!("Celeste is now closed.");
}

static HOME: Lazy<PathBuf> = Lazy::new(|| home_dir().expect("can't find HOME"));

static SAVE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = HOME.clone();
    path.push(".local");
    path.push("Celeste");
    path.push("Saves");
    assert!(path.is_dir(), "can't find Saves");
    path
});

static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = HOME.clone();
    path.push(".celeste-saves");
    path.push("logs");
    path
});
