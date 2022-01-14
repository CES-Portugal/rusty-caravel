use super::monitor::MonitorHandle;
use log::info;
use tokio::{signal, sync::mpsc};

#[derive(Debug)]
enum Messages {
    CleanExit,
}

struct CtrlCActor {
    inbox: mpsc::Receiver<Messages>,
    monitor_handle: MonitorHandle,
}

impl CtrlCActor {
    fn new(monitor_handle: MonitorHandle, inbox: mpsc::Receiver<Messages>) -> Self {
        CtrlCActor {
            inbox,
            monitor_handle,
        }
    }

    async fn tell_monitor(&self) {
        self.monitor_handle.ctrl_c_received().await.unwrap();
    }
}

async fn run(mut actor: CtrlCActor) {
    info!("Running");

    tokio::select! {
        _ = signal::ctrl_c() => {
            actor.tell_monitor().await
        },
        _ = actor.inbox.recv() => { },
    }

    info!("Shutting Down");
}

#[derive(Clone)]
pub struct CtrlCActorHandle {
    inbox: mpsc::Sender<Messages>,
}

impl CtrlCActorHandle {
    pub fn new(monitor: MonitorHandle) -> Self {
        let (sender, receiver) = mpsc::channel(1);

        let actor = CtrlCActor::new(monitor, receiver);

        tokio::spawn(run(actor));

        Self { inbox: sender }
    }

    pub async fn clean_shutdown(&self) {
        let msg = Messages::CleanExit;

        self.inbox.send(msg).await.expect("could not send message");
    }
}
