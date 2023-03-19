#![allow(unused)]
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use tracing::trace;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;

mod domutils;
mod durationutils;
mod old_main;
mod stringutils;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use once_cell::sync::Lazy;
use indexmap::IndexMap;
use home::home_dir;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ProcInfo {
    pub pid: u32,
    pub timestamp: u32,
    pub exe_path: Arc<PathBuf>,
    pub cwd_path: Arc<PathBuf>,
}

impl ProcInfo {
    pub fn for_path(path: impl AsRef<Path>) -> ProcInfo {
        let path = path.as_ref();

        let pid: u32 = path.components().last().unwrap().as_os_str().to_str().unwrap().parse().unwrap();

        let mut stat_path = path.to_path_buf();
        stat_path.push("stat");
        // read stat file, split by spaces, and then parse 

        let mut cwd_path = path.to_path_buf();  
        cwd_path.push("cwd");
        let cwd_path = cwd_path.read_link().unwrap();

        let mut exe_path = path.to_path_buf();  
        exe_path.push("exe");
        let exe_path = exe_path.read_link().unwrap();


        ProcInfo {
            pid,
            timestamp: todo!(),
            cwd_path: cwd_path.into(),
            exe_path: exe_path.into(),
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

    for entry in std::fs::read_dir("/proc").unwrap() {
        let entry = entry.unwrap();

        let mut path = entry.path();
        path.push("cwd");

        // fs.read_link()
        // entry.file_type().unwrap().is
    }
    

    // We launch Celeste through Steam, which can take an inconsistent amount of time. We poll every two seconds
    // to see if there are any new processes matching our "Celeste process" filter. Once we find one, we wait another
    // four seconds (to make sure that the actual game process is running, and not just its own launcher), then
    // we take a snapshot of every process currently matching our "celeste process" filter. We poll every four seconds
    // (if polling is neccessary, ideally we'd use some event instead) to see determine when Celeste has exited.
}

static STEAM_GAME_ID: usize = 504_230;

static HOME: Lazy<PathBuf> = Lazy::new(|| home_dir().expect("can't find HOME"));

static SAVE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path  = HOME.clone();
    path.push(".local");
    path.push("Celeste");
    path.push("Saves");
    assert!(path.is_dir(), "can't find Saves");
    path
});

static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path  = HOME.clone();
    path.push(".celeste-saves");
    path.push("logs");
    path
});

