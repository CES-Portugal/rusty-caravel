use futures_util::stream::StreamExt;
use tokio_socketcan::{CANSocket, Error};


pub async fn recv_can() -> Result<(), Error> {
    
    let mut socket_rx = CANSocket::open("can0")?;
    let socket_tx = CANSocket::open("can0")?;
    println!("Waiting for can messages...");
    while let Some(Ok(frame)) = socket_rx.next().await {
        //socket_tx.write_frame(frame)?.await;
        println!("msg received {:?}",frame);
    }
    Ok(())
}