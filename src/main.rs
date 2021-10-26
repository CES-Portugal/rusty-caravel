use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use regex::Regex;

async fn say_world() {
    println!("world");
}

async fn ctrlc_handler(tx: watch::Sender<&str>) {
    signal::ctrl_c().await;
    println!("closing...");
    tx.send("world").expect("TODO");
}
//clap rust -> cmd

async fn read_input(mut rx: watch::Receiver<&str>) {
    let mut line = String::new();
    let stdin = io::stdin();
    let re_send = Regex::new("^send").unwrap();
    //let handle_send = tokio::spawn(async move {send_can()}.await );

    loop {

        // if rx.changed().await.is_ok() {
        //     println!("Stop processing user input");
        //     break;
        // }
        stdin.lock().read_line(&mut line).expect("Could not read line");
        let op = line.trim_right();
        
        if op == "EXIT" {
            break;
        } else if re_send.is_match(op) {
            tokio::spawn(async move {send_can(op.to_string())}.await).await;
        }
        line.clear()
    }
}

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


fn send(canid : String, msg : String) {
    println!("Message Sent \nid: {}message: {}", canid, msg);
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("ctrl_c");

    let handle_input = tokio::spawn(async move {ctrlc_handler(tx)}.await );
    let handle_ctrl_c = tokio::spawn(async move {read_input(rx)}.await );

    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
    handle_input.await;
    handle_ctrl_c.await;


}
