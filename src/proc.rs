
use std::process::exit;

use fork::Fork;
use fork::fork;
use tracing::trace;
use tracing_unwrap::ResultExt;

pub fn daemonize() {
    trace!("Daemonizing with PID {}...", std::process::id());
    if matches!(fork().unwrap_or_log(), Fork::Child) {
        fork::close_fd().unwrap_or_log();
        fork::setsid().unwrap_or_log();
        if matches!(fork().unwrap_or_log(), Fork::Child) {
            trace!("Continuing as daemon with PID {}...", std::process::id());
        } else {
            exit(0)
        }
    } else {
        exit(0)
    }
}
