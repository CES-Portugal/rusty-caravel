use tokio::sync::{mpsc};
//use futures_timer::Delay;
//use std::time::Duration;

use crate::util::canutil;
use crate::util::canutil::CANFrame;

enum SenderCANMessages {
    SendToID {
        id: u16,
        message: u64,
        cycle_time: u64,
    }
}

struct SenderCAN {
    receiver: mpsc::Receiver<SenderCANMessages>,
    messages_sent: u32,
}

impl SenderCAN {
    fn new(receiver: mpsc::Receiver<SenderCANMessages>) -> Self {
        SenderCAN {
            receiver,
            messages_sent: 0,
        }
    }

    async fn handle_message(&mut self, msg: SenderCANMessages) {
        println!("can_handle");
        match msg {
            SenderCANMessages::SendToID {id, message, cycle_time}=> {
                println!("teste");
                let frame = CANFrame::new(12, &[1,2,3], false, false).unwrap();
                //let can_send_rcv = tokio::spawn(can_handler::recv_can(frame));
                
                println!("calling can handler");
                //can_send_rcv.await;
                //socket_tx.write_frame(frame).await;
                println!("Msg Sent!");
                canutil::send_can_frame(frame);
                // println!("Waiting 3 seconds");
                // Delay::new(Duration::from_secs(3)).await;
                //println!("Received {:?}, sending to {:?} with cycle time {:?} ms", message, id, cycle_time);
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
