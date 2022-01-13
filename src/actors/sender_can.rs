//! Sender CAN actor
//!
//! Actor responsible for sending can messages into a can socket

use tokio::sync::mpsc;

use crate::util::canutil;
use crate::util::canutil::{CANFrame, CANSocket};

use log::info;

/// Messages that the Actor Can Receive
enum SenderCANMessages {
    SendToID {
        id: u32,
        message: u64,
        _cycle_time: u64,
    },
}

struct SenderCAN {
    socket: CANSocket,
    receiver: mpsc::Receiver<SenderCANMessages>,
    _messages_sent: u32,
}

impl SenderCAN {
    fn new(receiver: mpsc::Receiver<SenderCANMessages>) -> Self {
        let socket = CANSocket::open("can0").expect("yikes");

        SenderCAN {
            socket,
            receiver,
            _messages_sent: 0,
        }
    }

    async fn handle_message(&mut self, msg: SenderCANMessages) {
        match msg {
            SenderCANMessages::SendToID {
                id,
                message,
                _cycle_time: _,
            } => {
                let frame = CANFrame::new(id, &message.to_be_bytes(), false, false).unwrap();
                canutil::send_can_frame(&self.socket, frame).await;
            }
        }
    }
}

async fn run(mut actor: SenderCAN) {
    info!("Running");

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

    pub async fn send_can_message(&self, id: u32, message: u64, _cycle_time: u64) {
        let msg = SenderCANMessages::SendToID {
            id,
            message,
            _cycle_time,
        };

        let _ = self.sender.send(msg).await;
    }
}
