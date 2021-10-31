use tokio::sync::{oneshot, mpsc};

pub async fn receive_can(id: Option<String>, nr_of_messages: Option<String>) {
    println!("receive id {:?} {:?} times.", id, nr_of_messages)
}