use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use regex::Regex;
<<<<<<< HEAD
use tokio::sync::broadcast;
use tokio::task;
=======
>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207

async fn say_world() {
    println!("world");
}

async fn ctrlc_handler(tx: broadcast::Sender<()>) {
    signal::ctrl_c().await;
    println!();
    println!("Closing...");
    tx.send((())).expect("TODO");
}
//clap rust -> cmd
<<<<<<< HEAD
<<<<<<< HEAD
fn read_input() {
=======
=======

>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207
async fn read_input(mut rx: watch::Receiver<&str>) {
>>>>>>> 36c8c64 (Added send to can function)
    let mut line = String::new();
    let stdin = io::stdin();
    let re_send = Regex::new("^send").unwrap();
    //let handle_send = tokio::spawn(async move {send_can()}.await );

    loop {
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======

>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207
        // if rx.changed().await.is_ok() {
        //     println!("Stop processing user input");
        //     break;
        // }
>>>>>>> 36c8c64 (Added send to can function)
        stdin.lock().read_line(&mut line).expect("Could not read line");
        let op = line.trim_right();
        
        if op == "EXIT" {
            break;
<<<<<<< HEAD
<<<<<<< HEAD
        } else if re_send.is_match(op) {
            tokio::spawn(async move {send_can(op.to_string())}.await).await;
=======
        } else if op == "send" {
            send_can();
>>>>>>> 36c8c64 (Added send to can function)
=======
        } else if re_send.is_match(op) {
            tokio::spawn(async move {send_can(op.to_string())}.await).await;
>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207
        }
        line.clear();
    }
}

<<<<<<< HEAD
async fn send_can(cmd: String) {
    let re_canid = Regex::new(r".* -id (\d+)").unwrap();
    let re_candata = Regex::new(r".* -m [[:punct:]](.*)[[:punct:]]").unwrap();
    //wait for message
    //let mut canid = String::new();
    let mut msg = String::new();
    let mut stdin = io::stdin();

    let canid = match re_canid.captures(&cmd){
        Some (cap) => cap[1].to_string(),
        None => String::new()
    };
    println!("{} -> CAN id", canid);
    // if re_canid.is_match(&cmd) {
        
    //     let cap = re_canid.captures(&cmd).unwrap();
    //     canid = cap[1].to_string();
    //     println!("{} -> CAN id", canid);
    // }
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

<<<<<<< HEAD
=======
fn send_can() {
    let mut canid = String::new();
=======
async fn send_can(cmd: String) {
    let re_canid = Regex::new(r".* -id (\d+)").unwrap();
    let re_candata = Regex::new(r".* -m [[:punct:]](.*)[[:punct:]]").unwrap();
    //wait for message
    //let mut canid = String::new();
>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207
    let mut msg = String::new();
    let mut stdin = io::stdin();

    let canid = match re_canid.captures(&cmd){
        Some (cap) => cap[1].to_string(),
        None => String::new()
    };
    println!("{} -> CAN id", canid);
    // if re_canid.is_match(&cmd) {
        
    //     let cap = re_canid.captures(&cmd).unwrap();
    //     canid = cap[1].to_string();
    //     println!("{} -> CAN id", canid);
    // }
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
>>>>>>> 36c8c64 (Added send to can function)


fn send(canid : String, msg : String) {
    println!("Message Sent \nid: {}message: {}", canid, msg);
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(1);


    let res = task::spawn_blocking(move || {
        read_input()
    });

<<<<<<< HEAD
    ctrlc_handler(tx).await;
    std::process::exit(0);
=======
    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
    handle_input.await;
    handle_ctrl_c.await;

<<<<<<< HEAD
>>>>>>> 36c8c64 (Added send to can function)
=======

>>>>>>> 5dcc13e29b16e9e609c22e1637e8821efd7b1207
}
