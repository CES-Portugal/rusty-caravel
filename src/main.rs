use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use regex::Regex;
use tokio::sync::broadcast;
use tokio::task;

mod actors;
use actors::stdin::StdInLinesHandle;
use actors::sender_can::SenderCANHandle;

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
                        sender.send_can_message(0x69, [1,2,3]).await;
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

//clap rust -> cmd
//fn read_input(sender: SenderCANHandle) {
//    let mut line = String::new();
//    let stdin = io::stdin();
//    let re_send = Regex::new("^send").unwrap();
//    //let handle_send = tokio::spawn(async move {send_can()}.await );
//
//    loop {
//        stdin.lock().read_line(&mut line).expect("Could not read line");
//        let op = line.trim_right();
//        
//        if op == "EXIT" {
//            break;
//        } else if re_send.is_match(op) {
//            println!("HERE");
//            sender.send_can_message(1, [1,2,3]);
//        }
//        println!("=> {:?}", op);
//        line.clear();
//    }
//}

async fn send_can(cmd: String) {
    let re_canid = Regex::new(r".* -id (\d+)").unwrap();
    let re_candata = Regex::new(r".* -m [[:punct:]](.*)[[:punct:]]").unwrap();
    let mut canid = String::new();
    let mut msg = String::new();
    let mut stdin = io::stdin();
    // println!("match, {}", &cmd);
    if re_canid.is_match(&cmd) {
        let cap = re_canid.captures(&cmd).unwrap();
        canid = cap[1].to_string();
        println!("{} -> CAN id", canid);
    }
    if re_candata.is_match(&cmd) {
        let cap = re_candata.captures(&cmd).unwrap();
        msg = cap[1].to_string();
        println!("{} -> CAN data", msg);
    }
    // println!("CAN ID:");
    // stdin.lock().read_line(&mut canid).expect("Could not read line");
    // println!("Message:");
    // stdin.lock().read_line(&mut msg).expect("Could not read line");
    // send(canid, msg);
}


fn send(canid : String, msg : String) {
    println!("Message Sent \nid: {}message: {}", canid, msg);
}

#[tokio::main]
async fn main() {

    let (watch_sender, watch_receiver) = tokio::sync::watch::channel(false);

    let sender = SenderCANHandle::new();

    let stdin  = StdInLinesHandle::new(
        tokio::runtime::Handle::current(),
        watch_receiver.clone(),
        sender.clone()
    );

    //let (line_sender, line_receiver) = tokio::sync::mpsc::channel(1);
    //start_reading_stdin_lines(line_sender, );

    // this will send a shutdown signal at some point
    start_activity_until_shutdown(watch_sender);

    stdin.spawn_handle.await;

    //read_input(line_receiver, watch_receiver, sender).await;
}

