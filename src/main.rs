#[allow(warnings, unused)]
mod logs;
mod daemon;
mod database;

#[tokio::main]
async fn main() {
    // Start Logger
    logs::init_logger();

    // Start Daemon
    log::info!("Starting Class-Watcher Daemon...");
    daemon::start_daemon().await;
}

