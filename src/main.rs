use tokio::signal;

mod actors;
mod util;
use actors::stdin::StdInLinesHandle;
use actors::sender_can::SenderCANHandle;
use actors::receiver_can::ReceiverCANHandle;

fn start_activity_until_shutdown(watch_sender: tokio::sync::watch::Sender<bool>) {
    tokio::spawn(async move {
        let result = signal::ctrl_c().await;
        let _result = match result {
            Ok(res) => res,
            Err(error) => panic!("Problem with ctrl + C thread, {:?}", error),
        };
        println!("Exiting after Ctr+C");
        let result = watch_sender.send(true);
        if let Err(error) = result {
            println!("watch_sender send error: {:?}", error);
        }
    });
}

#[tokio::main]
async fn main() {

    let (watch_sender, watch_receiver) = tokio::sync::watch::channel(false);

    let sender = SenderCANHandle::new();

    let receiver = ReceiverCANHandle::new();

    let stdin  = StdInLinesHandle::new(
        tokio::runtime::Handle::current(),
        watch_receiver.clone(),
        sender.clone(),
        receiver.clone(),
    );

    // this will send a shutdown signal at some point
    start_activity_until_shutdown(watch_sender);

    stdin.spawn_handle.await;
}
