#![allow(dead_code)]
use std::path::PathBuf;

use fork::fork;
use fork::Fork;
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
use tracing_unwrap::OptionExt;

use crate::steam_app::CELESTE;

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

    trace!("env = {:#?}", std::env::vars().collect::<IndexMap<_, _>>());

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
