use log::{info};
use env_logger;


mod actors;
mod util;

use actors::stdin::StdInLinesHandle;
use actors::sender_can::SenderCANHandle;
use actors::receiver_can::ReceiverCANHandle;
use actors::ctrlc::CtrlCActorHandle;


#[tokio::main]
async fn main() {

    env_logger::init();

    info!("Starting runtime");

    let receiver = ReceiverCANHandle::new();

    let sender = SenderCANHandle::new();

    let ctrlc = CtrlCActorHandle::new();

    let stdin  = StdInLinesHandle::new(
        tokio::runtime::Handle::current(),
        ctrlc.clone(),
        sender.clone(),
        receiver.clone(),
    );



    stdin.spawn_handle.await.expect("TODO remove expects");
}
