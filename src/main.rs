#![allow(dead_code)]
use std::collections::BTreeMap;
use std::path::PathBuf;
use tracing::info;
use tracing::trace;
use tracing_subscriber::prelude::*;

mod domutils;
mod durationutils;
mod old_main;
mod steam_app;
mod stringutils;
use home::home_dir;
use once_cell::sync::Lazy;
use tracing_unwrap::OptionExt;
mod proc;
use crate::proc::*;

use crate::steam_app::CELESTE;

#[derive(Debug)]
struct SteamEnv {
    steam_exe: PathBuf,
    username: String,

}

impl SteamEnv {
    pub fn get() -> Option<Self> {
        let env = std::env::vars().collect::<BTreeMap<_, _>>();
        
        if env.get("SteamEnv").map(|s| s.as_str()) == Some("1") {
            return None;
        }

        let steam_exe = env.get("STEAMSCRIPT").unwrap_or_log().into();

        let username = env.get("SteamUser").unwrap_or_log().into();

        Some(SteamEnv {
            steam_exe,
            username
        })
    }
}

fn main() {
    let file_appender =
        tracing_appender::rolling::never(LOG_DIR.clone(), concat!(env!("CARGO_PKG_NAME"), ".log"));
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

    trace!("env = {:#?}", std::env::vars().collect::<BTreeMap<_, _>>());

    let steam = SteamEnv::get();

    if let Some(steam) = steam {
        info!("Steam environment detected: {steam:#?}");
        daemonize();
    } else {
        info!("Not in Steam environment");
    }

    info!("Launching Celeste");

    let celeste = CELESTE.launch();

    info!("Waiting for Celeste to exit");

    celeste.wait_for_exit();

    info!("Celeste has exited.");
}

static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = home_dir().unwrap_or_log();
    path.push("logs");
    path
});
