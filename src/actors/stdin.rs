use super::sender_can::SenderCANHandle;
use super::ctrlc::CtrlCActorHandle;
use super::commands;
use super::receiver_can::ReceiverCANHandle;
use std::io;
use std::io::Write; 

struct StdInLines {
    line_receiver: tokio::sync::mpsc::Receiver<String>,
    watch_receiver: CtrlCActorHandle,
    sender: SenderCANHandle,
    receiver: ReceiverCANHandle
}


impl StdInLines {
    fn new (
        line_receiver: tokio::sync::mpsc::Receiver<String>,
        watch_receiver: CtrlCActorHandle,
        sender: SenderCANHandle,
        receiver: ReceiverCANHandle
    ) -> StdInLines {
        StdInLines { line_receiver, watch_receiver, sender, receiver }
    }

    async fn handle_command(&mut self, msg: String) -> bool {
        println!("msg recebida -> {:?}", msg);
        
        let parse_result = commands::parse(&msg);

        match parse_result {
            Ok(commands::ParsedCommand::Boss(cmd)) => {
                let _cmd_output = self.execute_command(cmd).await;
                true
            }
            Ok(commands::ParsedCommand::Exit) => {
                println!("exiting manually..."); 
                false
            },
            Err(e) => {
                println!("{}",e);
                true
            }
        }
        // match msg.as_str() {
        //     "exit" => { 
        //         println!("exiting manually..."); 
        //         false 
        //     },
        //     "send" => {
        //         self.sender.send_can_message(0x69, [1,2,3]).await;
        //         true
        //     },
        //     unexpected_line => {
        //         println!("unexpected command: {}", unexpected_line);
        //         true
        //     }
        // }
        //true
    }

    async fn execute_command(&mut self, cmd: BossCommand) -> impl std::fmt::Display {

        match cmd {
            BossCommand::SendCan { id: _ , message: _, cycletime: _ } => self.sender.send_can_message(123, 123, 0).await,
            BossCommand::ReceiveCan { id, nrofmessages } => self.receiver.receive_can_msg(id, nrofmessages).await,
        };
        //format!("ran command: {:?}", test)
        format!("ran command: ")
    }
}


#[derive(Debug)]
pub enum BossCommand {
    SendCan {
        id: Option<String>,
        message: Option<String>,
        cycletime: Option<String>,
    },
    ReceiveCan {
        id: Option<String>,
        nrofmessages: Option<String>
    },
}


async fn run(mut actor: StdInLines) {
    println!("Processing INPUTS");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        tokio::select! {
            Some(line) = actor.line_receiver.recv() => {
                if !actor.handle_command(line).await {
                    break;
                }
            }
            Ok(_) = actor.watch_receiver.wait_for_shutdown() => {
                println!("shutdown");
                break;
            }
        }
    }
}


fn reading_stdin_lines(
    runtime: tokio::runtime::Handle,
    sender: tokio::sync::mpsc::Sender<String>
) {
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut line_buf = String::new();
        while let Ok(_) = stdin.read_line(&mut line_buf) {
            let line = line_buf.trim_end().to_string();
            line_buf.clear();
            let sender2 = sender.clone();

            runtime.spawn(async move {
                let result = sender2.send(line).await;
                if let Err(error) = result {
                    println!("start_reading_stdin_lines send error: {:?}", error);
                }
            });
        }
    });
}

pub struct StdInLinesHandle {
    pub spawn_handle: tokio::task::JoinHandle<()>
}

impl StdInLinesHandle {

    pub fn new(
        runtime: tokio::runtime::Handle,
        watch_receiver: CtrlCActorHandle,
        sender: SenderCANHandle,
        receiver: ReceiverCANHandle
    ) -> StdInLinesHandle {

        let (line_sender, line_receiver) = tokio::sync::mpsc::channel(1);

        reading_stdin_lines(runtime, line_sender);

        let actor = StdInLines::new(line_receiver, watch_receiver, sender, receiver);

        let spawn_handle = tokio::spawn(run(actor));

        Self {spawn_handle}
    }
}
