use tokio::sync::mpsc;
//use futures_timer::Delay;
//use std::time::Duration;
use tokio_socketcan::CANFrame;
use super::can_handler;

enum SenderCANMessages {
    SendToID {
        id: Option<String>,
        message: Option<String>,
        cycle_time: Option<String>,
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
            SenderCANMessages::SendToID {id, message, cycle_time : _}=> {
                println!("teste");
                let frame = CANFrame::new(id.unwrap().parse::<u32>().unwrap(), message.unwrap().as_bytes(), false, false).unwrap();
                let send = tokio::spawn(can_handler::send_can(frame));
                
                println!("calling can handler");
                let result = send.await;
                let _result = match result {
                    Ok(res) => res,
                    Err(error) => panic!("Problem with sending can message, {:?}", error),
                };

                println!("Msg Sent!");
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


    pub async fn send_can_message(&self, id: Option<String>, message: Option<String>, cycle_time: Option<String>) {
        let msg = SenderCANMessages::SendToID {
            id, message, cycle_time,
        };
        let _ = self.sender.send(msg).await;
    }
}
