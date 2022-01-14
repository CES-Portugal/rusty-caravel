use anyhow::Result;
use log::info;
use tokio::sync::{mpsc, watch};

use super::ctrlc::CtrlCActorHandle;
use super::sender_can::SenderCANHandle;
use super::stdin::StdInLinesHandle;

#[derive(Debug)]
enum MonitorMessages {
    SpawnCtrlC,
    SpawnConsole,
    SpawnSender,
    AddToMonitor,
    RemoveFromMonitor,
    UglyExit,
    CleanExit,
    Exit,
}

pub struct Monitor {
    handler: MonitorHandle,
    shutdown: watch::Sender<bool>,
    ctrlc_actor: Option<CtrlCActorHandle>,
    console_actor: Option<StdInLinesHandle>,
    can_senders: Vec<SenderCANHandle>,
    inbox: mpsc::Receiver<MonitorMessages>,
}

impl Monitor {
    fn new(
        inbox: mpsc::Receiver<MonitorMessages>,
        shutdown: watch::Sender<bool>,
        handler: MonitorHandle,
    ) -> Self {
        let can_senders = Vec::new();

        Monitor {
            handler,
            shutdown,
            ctrlc_actor: None,
            console_actor: None,
            can_senders,
            inbox,
        }
    }

    fn handle_message(&mut self, msg: MonitorMessages) -> bool {
        match msg {
            MonitorMessages::Exit => {
                self.shutdown.send(true);
                false
            }

            MonitorMessages::UglyExit => {
                self.ctrlc_actor = None;
                false
            }

            MonitorMessages::CleanExit => {
                self.console_actor = None;
                false
            }

            MonitorMessages::SpawnCtrlC => {
                let ctrlc = CtrlCActorHandle::new(self.handler.clone());
                self.ctrlc_actor = Some(ctrlc);
                true
            }

            MonitorMessages::SpawnSender => {
                let sender = SenderCANHandle::new();
                self.can_senders.push(sender);
                true
            }

            MonitorMessages::SpawnConsole => {
                let sender = StdInLinesHandle::new(self.handler.clone());
                self.console_actor = Some(sender);
                true
            }

            _ => true,
        }
    }
}

pub async fn run(mut actor: Monitor) {
    info!("Running");

    while let Some(msg) = actor.inbox.recv().await {
        if !actor.handle_message(msg) {
            break;
        }
    }

    // tell CtrlC actor to shutdown
    if let Some(ctrlc) = actor.ctrlc_actor {
        info!("telling ctrlc watcher to shutdown");
        ctrlc.clean_shutdown().await;
    }

    // tell Console actor to shutdown
    if let Some(console) = actor.console_actor {
        info!("telling console to shutdown");
        console.shutdown().await;
    }

    actor.shutdown.send(true).unwrap();

    info!("Shuting Down");
}

#[derive(Clone)]
pub struct MonitorHandle {
    inbox: mpsc::Sender<MonitorMessages>,
    shutdown: watch::Receiver<bool>,
}

impl MonitorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(3);
        let (tx, shutdown) = watch::channel(false);

        let handle = Self {
            inbox: sender,
            shutdown,
        };

        let actor = Monitor::new(receiver, tx, handle.clone());

        tokio::spawn(run(actor));

        handle
    }

    pub async fn wait_to_die_like_in_life(&mut self) {
        self.shutdown.changed().await.unwrap();
    }

    /*
    pub async fn shutdown(&self) -> Result<()> {
        let msg = MonitorMessages::Exit;

        self.inbox.send(msg).await.expect("failed to send message");

        Ok(())
    }
    */

    pub async fn ctrl_c_received(&self) -> Result<()> {
        let msg = MonitorMessages::UglyExit;

        self.inbox.send(msg).await.expect("failed to send message");

        Ok(())
    }

    pub async fn exit_received(&self) -> Result<()> {
        let msg = MonitorMessages::CleanExit;

        self.inbox.send(msg).await.expect("failed to send message");

        Ok(())
    }

    pub async fn spawn_ctrlc_watcher(&self) -> Result<()> {
        let msg = MonitorMessages::SpawnCtrlC;

        self.inbox.send(msg).await.expect("failed to send message");

        Ok(())
    }

    pub async fn spawn_console(&self) -> Result<()> {
        let msg = MonitorMessages::SpawnConsole;

        self.inbox.send(msg).await.expect("failed to send message");

        Ok(())
    }

    /*

       pub async fn spawn_sender(&self) -> Result<()> {
           let msg = MonitorMessages::SpawnSender;

           self.inbox.send(msg).await.expect("failed to send message");

           Ok(())
       }
    */
}
