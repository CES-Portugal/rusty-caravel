#[derive(Debug)]
pub struct monitor {
    pub ctrlc : u8,
    pub receiver : u8,
    pub sender : u8,
    pub stdin : u8,
    pub simulation : u8,
}

pub fn new_simulation() -> monitor {
    monitor {
        ctrlc : 0,
        receiver : 0,
        sender : 0,
        stdin : 0,
        simulation : 1,
    } 
}