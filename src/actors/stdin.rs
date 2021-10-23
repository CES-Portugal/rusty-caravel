use tokio::sync::{oneshot, mpsc};
use futures::future::{Abortable, AbortHandle, Aborted};
use regex::Regex;
use tokio::task;
use std::io::{self, BufRead};

struct InteractiveParser {
    idRegex: Regex,
    dataRegex:Regex 
}

impl InteractiveParser {

    fn new() -> Self {
        InteractiveParser {
            idRegex: Regex::new(r".* -id (\d+)").unwrap(),
            dataRegex: Regex::new(r".* -id (\d+)").unwrap()
        }
    }
}

pub struct InteractiveParserHandle { }

impl InteractiveParserHandle {

    pub fn new() -> Self {
        task::spawn_blocking(move || {
            let mut line = String::new();
            let stdin = io::stdin();
            let actor = InteractiveParser::new();

            loop {
                stdin.lock().read_line(&mut line).expect("Could not read line");
                let op = line.trim_right();
        
                if op == "EXIT" {
                    break;
                }
                line.clear();
            }
        });

        Self { }
    }
}
