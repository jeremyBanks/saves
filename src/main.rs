#![allow(dead_code)]
use git2::BranchType;
use git2::Repository;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;
use tracing::trace;
use tracing_subscriber::prelude::*;

mod domutils;
mod durationutils;
mod celeste_stats;
mod steam_app;
mod stringutils;
use home::home_dir;
use once_cell::sync::Lazy;
use tracing_unwrap::OptionExt;
use tracing_unwrap::ResultExt;
mod daemon;
use crate::celeste_stats::celeste_stats;
use crate::daemon::*;

use crate::steam_app::CELESTE;

#[derive(Debug)]
struct SteamEnv {
    steam_exe: PathBuf,
    username: String,
}

impl SteamEnv {
    pub fn get() -> Option<Self> {
        let env = std::env::vars().collect::<BTreeMap<_, _>>();

        if env.get("SteamEnv").map(|s| s.as_str())? != "1" {
            return None;
        }

        let steam_exe = env.get("STEAMSCRIPT").unwrap_or_log().into();

        let username = env.get("SteamUser").unwrap_or_log().into();

        Some(SteamEnv {
            steam_exe,
            username,
        })
    }
}

pub const NAME: &str = match option_env!("CARGO_PKG_NAME") {
    Some(name) => name,
    None => "celeste-save",
};

fn main() {
    // This is blocking and probably slow, but the easiest alternatives didn't work once
    // we had forked daemon threads going.
    let file_appender =
        tracing_appender::rolling::never(LOG_DIR.clone(), NAME.to_string() + ".log");

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
        info!("Steam environment detected: {steam:#?}. Daemonizing.");
        daemonize();
    } else {
        info!("Not in Steam environment");
    }

    let repo = git_repo();
    
    if let Ok(mut _origin) = repo.find_remote("origin") {
        info!("Pulling changes from remote origin");
        let mut cmd = std::process::Command::new("git");
        cmd.arg("fetch");
        cmd.arg("--verbose");
        cmd.arg("origin");
        cmd.env("GIT_DIR", &*GIT_DIR);
        cmd.status().unwrap_or_log();
    } else {
        trace!("No origin remote found, not pushing");
    }

    info!("Launching Celeste");

    let celeste = CELESTE.launch();

    info!("Waiting for Celeste to exit");

    celeste.wait_for_exit();

    info!("Celeste has exited. Reading save files.");

    let mut files = BTreeMap::new();

    for entry in CELESTE.saves_dir().read_dir().unwrap_or_log() {
        let entry = entry.unwrap_or_log();
        let path = entry.path();
        if path.extension().map(|s| s == "celeste").unwrap_or(false) {
            let contents = std::fs::read_to_string(path).unwrap_or_log();
            files.insert(entry.file_name(), contents);
        }
    }

    let mut generated = BTreeMap::new();
    for (name, body) in files.iter() {
        let name = name.to_str().unwrap_or_log();
        if name == "settings.celeste" {
            continue;
        }
        let stats = celeste_stats(&body);
        generated.insert(name.replace(".celeste", ".html"), stats);
    }

    for (name, stats) in generated.into_iter() {
        files.insert(name.into(), stats);
    }

    let mut tree = repo.treebuilder(None).unwrap_or_log();
    for (name, body) in files.iter() {
        let mut blob = repo.blob_writer(Some(name.as_ref())).unwrap_or_log();
        blob.write_all(body.as_bytes()).unwrap_or_log();
        let blob = blob.commit().unwrap_or_log();
        tree.insert(name, blob, 0o100_644).unwrap_or_log();
    }
    let tree = tree.write().unwrap_or_log();
    let tree = repo.find_tree(tree).unwrap_or_log();

    let branch = repo.find_branch("celeste", BranchType::Local).ok();
    let existing_tree = branch
        .as_ref()
        .map(|b| b.get().peel_to_tree().unwrap_or_log());

    if Some(tree.id()) == existing_tree.map(|t| t.id()) {
        info!("No changes to save.");
    } else {
        // TODO: if upstream doesn't match, add both as parents?
        let parents = branch
            .map(|b| vec![b.get().peel_to_commit().unwrap_or_log()])
            .unwrap_or_default();

        let signature = repo.signature().unwrap_or_log();
        let commit = repo
            .commit(
                Some("refs/heads/celeste"),
                &signature,
                &signature,
                &tree.id().to_string(),
                &tree,
                parents[..].iter().collect_vec().as_ref(),
            )
            .unwrap_or_log();

        info!("Committed {commit:?} to git branch 'celeste'");

        if let Ok(mut _origin) = repo.find_remote("origin") {
            info!("Pushing changes to remote 'origin");
            // We shell out instead of figuring out the auth dance.
            let mut cmd = std::process::Command::new("git");
            cmd.arg("push");
            cmd.arg("--verbose");
            cmd.arg("origin");
            cmd.arg("celeste:celeste");
            cmd.env("GIT_DIR", &*GIT_DIR);
            cmd.status().unwrap_or_log();
        } else {
            trace!("No origin remote found, not pushing");
        }
    }
}

static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = home_dir().unwrap_or_log();
    path.push(String::new() + "." + crate::NAME);
    path.push("logs");
    path
});

static GIT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = home_dir().unwrap_or_log();
    path.push(String::new() + "." + crate::NAME);
    path.push("git");
    path
});

pub fn git_repo() -> Repository {
    match Repository::open_bare(&*GIT_DIR) {
        Ok(repo) => repo,
        Err(err) => {
            trace!("Error opening git repo: {err:?}");
            info!("Creating new git repo at {:?}", &*GIT_DIR);
            std::fs::create_dir_all(&*GIT_DIR).unwrap_or_log();
            Repository::init_bare(&*GIT_DIR).unwrap_or_log()
        }
    }
}
