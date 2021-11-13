use tokio::signal;
use tokio::sync::{mpsc};
use anyhow::anyhow;

struct CtrlCActor {
    notification_channel: tokio::sync::watch::Sender<bool>,
}

impl CtrlCActor {

    fn new(notification_channel: tokio::sync::watch::Sender<bool>) -> Self { 
        CtrlCActor { notification_channel } 
    }

    fn ctrlc_announce(self) {
        self.notification_channel.send(true);
    }
}

async fn run(actor: CtrlCActor) {
    println!("Running... CtrlCActor");

    signal::ctrl_c().await.expect("Problem with Ctrl C");

    println!("Exiting after Ctr+C");

    actor.notification_channel.send(true).expect("Problem sending signal");
}


#[derive(Clone)]
pub struct CtrlCActorHandle {
    notification_channel: tokio::sync::watch::Receiver<bool>,
}

impl CtrlCActorHandle {

    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::watch::channel(false);

        let actor = CtrlCActor::new(sender);

        tokio::spawn(run(actor));

        Self { notification_channel: receiver }
    }

    pub async fn wait_for_shutdown(&mut self) -> anyhow::Result<()> {

        self.notification_channel.changed().await;

        Ok(())
    }

}
