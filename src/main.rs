use std::io::{self, BufRead};
use tokio::signal;
use tokio::sync::watch;


async fn say_world() {
    println!("world");
}

async fn ctrlc_handler(tx: watch::Sender<&str>) {
    signal::ctrl_c().await;
    println!("closing...");
    tx.send("world").expect("TODO");
}

async fn read_input(mut rx: watch::Receiver<&str>) {
    let mut line = String::new();
    let stdin = io::stdin();

    loop {
        if rx.changed().await.is_ok() {
            println!("Stop processing user input");
            break;
        }
        stdin.lock().read_line(&mut line).expect("Could not read line");
        let op = line.trim_right();
        if op == "EXIT" {
            break;
        }
        line.clear()
    }
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
