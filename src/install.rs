use std::fs;
use std::os::unix::prelude::PermissionsExt;

use itertools::Itertools;
use tracing::info;
use tracing_unwrap::OptionExt;
use tracing_unwrap::ResultExt;

use crate::BIN_DIR;
use crate::ETC_DIR;
use crate::NAME;

pub fn install() {
    let argv = std::env::args().collect_vec();
    let own_binary = fs::read(&argv[0]).unwrap_or_log();

    info!("Installing.");

    let bin_path = BIN_DIR.join(NAME);
    fs::create_dir_all(&*BIN_DIR).unwrap_or_log();
    fs::write(&bin_path, &own_binary).unwrap_or_log();
    let mut perms = fs::metadata(&bin_path).unwrap_or_log().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin_path, perms).unwrap_or_log();
    let bin_path = bin_path.to_str().unwrap_or_log();

    let icon_path = ETC_DIR.join(format!("icon.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&icon_path, include_bytes!("../assets/icon.png")).unwrap_or_log();
    let icon_path = icon_path.to_str().unwrap_or_log();

    let banner_path = ETC_DIR.join(format!("banner.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&banner_path, include_bytes!("../assets/banner.png")).unwrap_or_log();
    let _banner_path = banner_path.to_str().unwrap_or_log();

    let capsule_path = ETC_DIR.join(format!("capsule.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&capsule_path, include_bytes!("../assets/capsule.png")).unwrap_or_log();
    let _capsule_path = capsule_path.to_str().unwrap_or_log();

    let logo_path = ETC_DIR.join(format!("logo.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&logo_path, include_bytes!("../assets/logo.png")).unwrap_or_log();
    let _logo_path = logo_path.to_str().unwrap_or_log();

    let desktop_path = ETC_DIR.join(format!("{NAME}.desktop"));
    fs::write(
        &desktop_path,
        format!(
            "#!/usr/bin/env xdg-open
[Desktop Entry]
Type=Application
Name=Celeste with Sync
Comment=Play Celeste and sync saves to git
Categories=Game
Exec={bin_path}
Icon={icon_path}
"
        ),
    )
    .unwrap_or_log();
    let mut perms = fs::metadata(&desktop_path).unwrap_or_log().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&desktop_path, perms).unwrap_or_log();

    info!("Attempting to install with xdg-desktop-menu.");
    let mut cmd = std::process::Command::new("xdg-desktop-menu");
    cmd.arg("install");
    cmd.arg(&desktop_path);
    cmd.status().unwrap_or_log();
}
