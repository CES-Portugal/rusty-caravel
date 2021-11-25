use log::{info};
use env_logger;

mod actors;
mod util;

use actors::stdin::StdInLinesHandle;
use actors::sender_can::SenderCANHandle;
use actors::receiver_can::ReceiverCANHandle;
use actors::ctrlc::CtrlCActorHandle;
use actors::simulation::{monitor,new_simulation};

// enum monitor {
//     ctrlc(u8),
//     receiver(u8),
//     sender(u8),
//     stdin(u8),
//     simulation(u8),
// }

// impl monitor {
//     fn new(&mut self) {
//         match 
//     }
//     fn start_sim(&mut self) {

//         let send = SenderCANHandle::new(&mut self::sender);

//     }
// }

#[tokio::main]
async fn main() {

    env_logger::init();

    info!("Starting runtime");
    let mut mon = new_simulation() ;
    let sender = SenderCANHandle::new(&mut mon);
    let receiver = ReceiverCANHandle::new(&mut mon);
    let ctrlc = CtrlCActorHandle::new(&mut mon);
    let stdin  = StdInLinesHandle::new(
        &mut mon,
        tokio::runtime::Handle::current(),
        ctrlc.clone(),
        sender.clone(),
        receiver.clone(),
    );
    println!("Current Actors {:?}", mon);
    stdin.spawn_handle.await.expect("TODO remove expects");

}
