use log::{debug, error, info, Level};
use anyhow::{Result, anyhow};
use futures_util::stream::StreamExt;

pub use tokio_socketcan::{CANFrame, CANSocket};

pub async fn send_can_frame(socket: &CANSocket, frame: CANFrame) {

    match socket.write_frame(frame).expect("Writing is busted").await {
        Ok(_) => {
            debug!("Wrote {:?} # {:?}", socket, frame);
        }
        Err(_) => {
            debug!("Failed writing {:?} # {:?}", socket, frame);
        }
    }

}
