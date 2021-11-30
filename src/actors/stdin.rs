use super::sender_can::SenderCANHandle;
use super::ctrlc::CtrlCActorHandle;
use super::receiver_can::ReceiverCANHandle;
use super::simulation::monitor;
use std::io;
use std::io::Write; 
use tokio::time::{sleep, Duration};
use shell_words;

use clap::{ Parser, AppSettings };

use log::{info};

#[derive(Parser)]
#[clap(version = "0.2.0", author = "marujos", setting=AppSettings::NoBinaryName)]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    config: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Send(Send),
    Receive(Receive),
    Exit(Exit)
}

#[derive(Parser)]
struct Send {
    id: String, 
    message: String,
    cycletime: String,
}

#[derive(Parser)]
struct Exit {}

#[derive(Parser)]
struct Receive {
    id: Option<String>,
    nr_of_messages: Option<String>,
}

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
        let words = shell_words::split(&msg).expect("cmd split went bust");
        
        let cmd : Opts = match Opts::try_parse_from(words) {
            Ok(opts) => opts,
            Err(error) => { println!("{}", error); return true }
        };

        match cmd.subcmd {
            SubCommand::Send(t) => {
                let id : u32 = t.id.parse().expect("TODO handle errors"); // Parse into number
        
                let message : u64 = t.message.parse().expect("TODO handle errors");

                let cycletime : u64 = t.cycletime.parse().expect("TODO handle errors");
                
                if cycletime == 0 {
                    self.sender.send_can_message(id, message, cycletime).await;
                    true
                }
                else {
                    tokio::spawn(cyclic_sender(self.sender.clone(), id, message, cycletime));
                    true
                }

            },
            SubCommand::Receive(t) => {
                self.receiver.receive_can_msg(t.id, t.nr_of_messages).await;
                true
            },
            SubCommand::Exit(_t) => { false }
        }
    }
}

async fn cyclic_sender(sender: SenderCANHandle, id: u32, message : u64, cycletime: u64) {
    loop {
        sleep(Duration::from_millis(cycletime)).await;
        sender.send_can_message(id, message, cycletime).await
    }
}

async fn run(mut actor: StdInLines) {
    info!("Running");
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
        mut simulation : &mut monitor,
        runtime: tokio::runtime::Handle,
        watch_receiver: CtrlCActorHandle,
        sender: SenderCANHandle,
        receiver: ReceiverCANHandle
    ) -> StdInLinesHandle {
        
        simulation.stdin +=1;
        let (line_sender, line_receiver) = tokio::sync::mpsc::channel(1);

        reading_stdin_lines(runtime, line_sender);

        let actor = StdInLines::new(line_receiver, watch_receiver, sender, receiver);

        let spawn_handle = tokio::spawn(run(actor));

        Self {spawn_handle}
    }
}
