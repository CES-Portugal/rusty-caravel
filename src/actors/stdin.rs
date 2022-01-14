use super::monitor::MonitorHandle;
use shell_words;
use tokio::sync::mpsc;

use clap::{AppSettings, Parser};

use log::info;

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
    Exit(Exit),
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

#[derive(Debug)]
enum Messages {
    Line { line: String },
    Shutdown,
}

struct StdInLines {
    inbox: mpsc::Receiver<Messages>,
    monitor: MonitorHandle,
}

impl StdInLines {
    fn new(inbox: mpsc::Receiver<Messages>, monitor: MonitorHandle) -> Self {
        StdInLines { inbox, monitor }
    }

    async fn tell_monitor(&self) {
        self.monitor.exit_received().await.unwrap();
    }

    async fn handle_message(&mut self, msg: Messages) -> bool {
        match msg {
            Messages::Shutdown => false,
            Messages::Line { line } => self.handle_command(line).await,
        }
    }

    async fn handle_command(&mut self, msg: String) -> bool {
        let words = shell_words::split(&msg).expect("cmd split went bust");

        let cmd: Opts = match Opts::try_parse_from(words) {
            Ok(opts) => opts,
            Err(error) => {
                println!("{}", error);
                return true;
            }
        };

        match cmd.subcmd {
            SubCommand::Send(t) => {
                //let id: u32 = t.id.parse().expect("TODO handle errors"); // Parse into number

                //let message: u64 = t.message.parse().expect("TODO handle errors");

                //let cycletime: u64 = t.cycletime.parse().expect("TODO handle errors");

                //if cycletime == 0 {
                //    self.sender.send_can_message(id, message, cycletime).await;
                //    true
                //} else {
                //    tokio::spawn(cyclic_sender(self.sender.clone(), id, message, cycletime));
                //    true
                //}
                true
            }
            SubCommand::Receive(t) => {
                //self.receiver.receive_can_msg(t.id, t.nr_of_messages).await;
                true
            }
            SubCommand::Exit(_t) => {
                self.tell_monitor().await;
                false
            }
        }
    }
}

//async fn cyclic_sender(sender: SenderCANHandle, id: u32, message: u64, cycletime: u64) {
//    loop {
//        sleep(Duration::from_millis(cycletime)).await;
//        sender.send_can_message(id, message, cycletime).await
//    }
//}

async fn run(mut actor: StdInLines) {
    info!("Running");

    while let Some(msg) = actor.inbox.recv().await {
        if !actor.handle_message(msg).await {
            break;
        }
    }

    info!("Shutting Down");
}

fn reading_stdin_lines(sender: mpsc::Sender<Messages>) {
    let runtime = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        let sender = sender.clone();
        let stdin = std::io::stdin();
        let mut line_buf = String::new();
        while let Ok(_) = stdin.read_line(&mut line_buf) {
            let sender = sender.clone();
            let line = line_buf.trim_end().to_string();
            line_buf.clear();

            runtime.spawn(async move {
                let message = Messages::Line { line };
                let result = sender.send(message).await;
                if let Err(error) = result {
                    println!("start_reading_stdin_lines send error: {:?}", error);
                }
            });
        }
    });
}

pub struct StdInLinesHandle {
    inbox: mpsc::Sender<Messages>,
}

impl StdInLinesHandle {
    pub fn new(
        // runtime: tokio::runtime::Handle,
        //watch_receiver: CtrlCActorHandle,
        //sender: SenderCANHandle,
        //receiver: ReceiverCANHandle
        monitor: MonitorHandle,
    ) -> StdInLinesHandle {
        let (tx, inbox) = tokio::sync::mpsc::channel(5);

        reading_stdin_lines(tx.clone());

        let actor = StdInLines::new(inbox, monitor);

        tokio::spawn(run(actor));

        Self { inbox: tx }
    }

    pub async fn shutdown(&self) {
        let msg = Messages::Shutdown;

        self.inbox.try_send(msg).expect("What ?");
    }
}
