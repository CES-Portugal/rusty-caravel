use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error, CANFrame};


pub async fn recv_can(frame: CANFrame) -> Result<(), Error> {

    let socket_tx = CANSocket::open("vcan0")?;
    println!("writing on vcan0");
    socket_tx.write_frame(frame)?.await?;
    println!("msg received {:?}",frame);

    Ok(())
}
