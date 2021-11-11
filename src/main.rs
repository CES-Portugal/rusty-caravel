use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use tokio::sync::broadcast;
use tokio::task;

mod actors;
use actors::stdin::StdInLinesHandle;
use actors::sender_can::SenderCANHandle;
use actors::receiver_can::ReceiverCANHandle;
//use actors::can_handler;

fn start_reading_stdin_lines(
    sender: tokio::sync::mpsc::Sender<String>,
    runtime: tokio::runtime::Handle
) {
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut line_buf = String::new();
        while let Ok(_) = stdin.read_line(&mut line_buf) {
            let line = line_buf.trim_end().to_string();
            line_buf.clear();
            let sender2 = sender.clone();

            runtime.spawn(async move {
                let result = sender2.send(line).await;
                if let Err(error) = result {
                    println!("start_reading_stdin_lines send error: {:?}", error);
                }
            });
        }
    });
}

fn start_activity_until_shutdown(watch_sender: tokio::sync::watch::Sender<bool>) {
    tokio::spawn(async move {
        signal::ctrl_c().await;
        println!("Exiting after Ctr+C");
        let result = watch_sender.send(true);
        if let Err(error) = result {
            println!("watch_sender send error: {:?}", error);
        }
    });
}

async fn read_input(
    mut line_receiver: tokio::sync::mpsc::Receiver<String>,
    mut watch_receiver: tokio::sync::watch::Receiver<bool>,
    mut sender: SenderCANHandle
) {
    loop {
        tokio::select! {
            Some(line) = line_receiver.recv() => {
                // process the input
                match line.as_str() {
                    "exit" => {
                        println!("exiting manually...");
                        break;
                    },
                    "send" => {
                        //sender.send_can_message(0x69, [1,2,3]).await;
                    }
                    unexpected_line => {
                        println!("unexpected command: {}", unexpected_line);
                    }
                }
            }
            Ok(_) = watch_receiver.changed() => {
                println!("shutdown");
                break;
            }
        }
    }
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

    // let can_send_rcv = tokio::spawn(can_handler::recv_can());
    //let (line_sender, line_receiver) = tokio::sync::mpsc::channel(1);
    //start_reading_stdin_lines(line_sender, );

    // this will send a shutdown signal at some point
    start_activity_until_shutdown(watch_sender);

    stdin.spawn_handle.await;
    //can_send_rcv.await;
    //read_input(line_receiver, watch_receiver, sender).await;
}
