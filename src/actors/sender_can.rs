//! Sender CAN actor
//!
//! Actor responsible for sending can messages into a can socket

use log::{debug, error, info, Level};
use tokio::sync::{mpsc};

use crate::util::canutil;
use crate::util::canutil::{CANFrame, CANSocket};

/// Messages that the Actor Can Receive
enum SenderCANMessages {
    SendToID {
        id: u16,
        message: u64,
        cycle_time: u64,
    }
}

struct SenderCAN {
    socket  : CANSocket,
    receiver: mpsc::Receiver<SenderCANMessages>,
    messages_sent: u32,
}

impl SenderCAN {
    fn new(receiver: mpsc::Receiver<SenderCANMessages>) -> Self {

        let socket = CANSocket::open("vcan0").expect("yikes");

        SenderCAN {
            socket,
            receiver,
            messages_sent: 0,
        }
    }

    async fn handle_message(&mut self, msg: SenderCANMessages) {
        match msg {
            SenderCANMessages::SendToID {id, message, cycle_time}=> {
                let frame = CANFrame::new(12, &[1,2,3], false, false).unwrap();
                canutil::send_can_frame(&self.socket, frame).await;
            },
        }
    }
}

async fn run(mut actor: SenderCAN) {
    println!("Running...");

    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}


#[derive(Clone)]
pub struct SenderCANHandle {
    sender: mpsc::Sender<SenderCANMessages>,
}

impl SenderCANHandle {

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = SenderCAN::new(receiver);

        tokio::spawn(run(actor));

        Self { sender }
    }


    pub async fn send_can_message(&self, id: u16, message: u64, cycle_time: u64 ) {
        let msg = SenderCANMessages::SendToID {
            id, message, cycle_time,
        };

        let _ = self.sender.send(msg).await;
    }
}
