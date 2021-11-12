use tokio::sync::mpsc;


enum ReceiverCANMessages {
    ReceiveFromId {
        id: Option<String>,
        message: Option<String>,
    }
}

struct ReceiverCan {
    receiver: mpsc::Receiver<ReceiverCANMessages>,
    messages_received: u32,
}

impl ReceiverCan {
    fn new(receiver: mpsc::Receiver<ReceiverCANMessages>) -> Self {
        ReceiverCan {
            receiver,
            messages_received: 0,
        }
    }

    fn handle_receiver(&mut self, msg: ReceiverCANMessages) {
        match msg {
            ReceiverCANMessages::ReceiveFromId {id, message}=> {
                println!("Received {:?}, from canid {:?}.", message, id);
            },
        }
    }
}

async fn run(mut actor: ReceiverCan) {
    println!("Running Receiver Can...");

    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_receiver(msg);
    }
}


#[derive(Clone)]
pub struct ReceiverCANHandle {
    sender: mpsc::Sender<ReceiverCANMessages>,
}

impl ReceiverCANHandle {

    pub fn new() -> Self  {
        let (sender, receiver) = mpsc::channel(9);
        let actor = ReceiverCan::new(receiver);

        tokio::spawn(run(actor));

        Self { sender }
    }

    pub async fn receive_can_msg(&self, id: Option<String>, message: Option<String>) {
        let msg = ReceiverCANMessages::ReceiveFromId {
            id, message,
        };
        println!("receive_can_msg CALL on receiver_can.rs");//{:?} -> {:?} .", id, message);
        let _ = self.sender.send(msg).await;
    } 
}