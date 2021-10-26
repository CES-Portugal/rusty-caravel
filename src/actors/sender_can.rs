use tokio::sync::{oneshot, mpsc};

struct SenderCAN {
    receiver: mpsc::Receiver<SenderCANMessages>,
    messages_sent: u32,
}

enum SenderCANMessages {
    SendToID {
        id: u32,
        message: [i32; 3]
    }
}

impl SenderCAN {
    fn new(receiver: mpsc::Receiver<SenderCANMessages>) -> Self {
        SenderCAN {
            receiver,
            messages_sent: 0,
        }
    }

    fn handle_message(&mut self, msg: SenderCANMessages) {
        match msg {
            SenderCANMessages::SendToID {id, message}=> {
                println!("Received {:?}, sending to {:?}...", message, id);
            },
        }
    }
}

async fn run(mut actor: SenderCAN) {
    println!("Running...");

    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
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


    pub async fn send_can_message(&self, id: u32, message: [i32; 3]) {
        let msg = SenderCANMessages::SendToID {
            id, message
        };

        let _ = self.sender.send(msg).await;
    }
}