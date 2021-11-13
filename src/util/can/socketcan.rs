use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};

pub use tokio_socketcan::CANFrame;

pub async fn send_can_frame(socket: CANSocket, frame: CANFrame) -> Result<(), Error> {

    let socket_tx = CANSocket::open("vcan0")?;
    println!("writing on vcan0");
    socket_tx.write_frame(frame)?.await?;
    println!("msg received {:?}",frame);

    Ok(())
}
