#[cfg_attr(not(windows), path = "can/socketcan.rs")]
#[cfg_attr(windows, path = "can/windowscan.rs")]
pub mod canutil;

pub mod Actor {
    pub trait ActorBehaviour {
        fn new() -> Self;

        fn demo() {
            println!("Demo From trait")
        }
    }
}
