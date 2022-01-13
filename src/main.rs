use env_logger;
use log::info;

mod actors;
mod util;

use actors::ctrlc::CtrlCActorHandle;
use actors::receiver_can::ReceiverCANHandle;
use actors::sender_can::SenderCANHandle;
use actors::stdin::StdInLinesHandle;

use actors::monitor::MonitorHandle;
use tokio::signal::ctrl_c;

use crate::actors::monitor;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting runtime");

    let mut monitor = MonitorHandle::new();

    monitor
        .spawn_ctrlc_watcher()
        .await
        .expect("could not spawn ctrl c watcher");

    //let mut ctrlc = CtrlCActorHandle::new(monitor.clone());

    monitor.wait_to_die_like_in_life().await;

    //let stdin = StdInLinesHandle::new(tokio::runtime::Handle::current(), monitor.clone());
    //let receiver = ReceiverCANHandle::new();

    //let sender = SenderCANHandle::new();

    //monitor.shutdown().await.expect("demo");

    //stdin.spawn_handle.await.expect("TODO remove expects");
}
