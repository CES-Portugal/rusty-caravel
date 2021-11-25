use tokio::signal;
use super::simulation::monitor;
use log::{info};

struct CtrlCActor {
    notification_channel: tokio::sync::watch::Sender<bool>,
}

impl CtrlCActor {

    fn new(notification_channel: tokio::sync::watch::Sender<bool>) -> Self { 
        CtrlCActor { notification_channel } 
    }

    fn ctrlc_announce(self) {
        self.notification_channel.send(true).expect("CtrlC failed");
    }
}

async fn run(actor: CtrlCActor) {
    info!("Running");

    signal::ctrl_c().await.expect("Problem with Ctrl C");

    println!("Exiting after Ctr+C");

    actor.ctrlc_announce();
}


#[derive(Clone)]
pub struct CtrlCActorHandle {
    notification_channel: tokio::sync::watch::Receiver<bool>,
}

impl CtrlCActorHandle {

    pub fn new(mut simulation : &mut monitor) -> Self {
        simulation.ctrlc += 1;
        let (sender, receiver) = tokio::sync::watch::channel(false);

        let actor = CtrlCActor::new(sender);

        tokio::spawn(run(actor));

        Self { notification_channel: receiver }
    }

    pub async fn wait_for_shutdown(&mut self) -> anyhow::Result<()> {

        self.notification_channel.changed().await?;

        Ok(())
    }

}
