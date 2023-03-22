use std::fs;
use std::os::unix::prelude::PermissionsExt;

use itertools::Itertools;
use steamlocate::SteamDir;
use tracing::info;
use tracing_unwrap::OptionExt;
use tracing_unwrap::ResultExt;

use crate::dirs::BIN_DIR;
use crate::dirs::ETC_DIR;
use crate::dirs::STEAM_USER_DATA_DIR;
use crate::NAME;

use steamlocate;

pub fn install() {
    let argv = std::env::args().collect_vec();
    let own_binary = fs::read(&argv[0]).unwrap_or_log();

    let mut steam = SteamDir::locate().unwrap_or_log();
    let shortcuts = steam.shortcuts();

    dbg!(shortcuts);
    return;

    info!("Installing.");

    let bin_path = BIN_DIR.join(NAME);
    fs::create_dir_all(&*BIN_DIR).unwrap_or_log();
    fs::write(&bin_path, &own_binary).unwrap_or_log();
    let mut perms = fs::metadata(&bin_path).unwrap_or_log().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin_path, perms).unwrap_or_log();
    let bin_path = bin_path.to_str().unwrap_or_log();

    let icon_path = ETC_DIR.join(format!("0_icon.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&icon_path, include_bytes!("../assets/0_icon.png")).unwrap_or_log();
    let icon_path = icon_path.to_str().unwrap_or_log();

    let hero_path = ETC_DIR.join(format!("0_hero.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&hero_path, include_bytes!("../assets/0_hero.png")).unwrap_or_log();

    let capsule_path = ETC_DIR.join(format!("0.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&capsule_path, include_bytes!("../assets/0.png")).unwrap_or_log();

    let logo_path = ETC_DIR.join(format!("0_logo.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&logo_path, include_bytes!("../assets/0_logo.png")).unwrap_or_log();

    let logo_path = ETC_DIR.join(format!("0_logo.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&logo_path, include_bytes!("../assets/0_logo.png")).unwrap_or_log();

    let logo_config_path = ETC_DIR.join(format!("0.json"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&logo_config_path, include_bytes!("../assets/0.json")).unwrap_or_log();

    let poster_path = ETC_DIR.join(format!("0p.png"));
    fs::create_dir_all(&*ETC_DIR).unwrap_or_log();
    fs::write(&poster_path, include_bytes!("../assets/0p.png")).unwrap_or_log();

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
