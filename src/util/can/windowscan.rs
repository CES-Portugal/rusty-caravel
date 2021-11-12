use anyhow::Result;

pub struct CANFrame {}

impl CANFrame {

    pub fn new(id: u32, data: &[u8], rtr: bool, err: bool) -> Result<Self> {
        Ok(CANFrame{})
    }

}

pub async fn send_can_frame(frame: CANFrame) -> Result<()> {

    println!("called send_frame");

    Ok(())
}
