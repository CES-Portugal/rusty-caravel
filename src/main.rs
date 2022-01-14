mod actors;
mod util;

use actors::monitor::MonitorHandle;
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    info!("Starting runtime");

    let mut monitor = MonitorHandle::new();

    monitor
        .spawn_ctrlc_watcher()
        .await
        .expect("could not spawn ctrl c watcher");

    monitor
        .spawn_console()
        .await
        .expect("could not spawn console actor");

    monitor.wait_to_die_like_in_life().await;
}
