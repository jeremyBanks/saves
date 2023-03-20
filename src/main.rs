#![allow(dead_code)]
use std::path::PathBuf;

use fork::Fork;
use fork::fork;
use tracing::info;
use tracing::trace;
use tracing_subscriber::prelude::*;

mod domutils;
mod durationutils;
mod old_main;
mod steam_app;
mod stringutils;
use home::home_dir;
use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::steam_app::CELESTE;

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

    info!("Launching Celeste");

    let celeste = CELESTE.launch();

    info!("Daemonizing to wait for Celeste to exit");

    // Double-fork to detach from the process Steam's going to kill for never launching a UI.
    let Fork::Child = fork().unwrap() else {
        std::process::exit(0);
    };
    let Fork::Child = fork().unwrap() else {
        std::process::exit(0);
    };
    
    info!("I'm a daemon!");

    celeste.wait_for_exit();

    info!("Celeste has exited.");
}

static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = home_dir().unwrap();
    path.push(".celeste-saves");
    path.push("logs");
    path
});
