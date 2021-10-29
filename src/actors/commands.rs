use super::stdin::BossCommand;
use std::str::FromStr;
use structopt::StructOpt;
use anyhow::anyhow;

#[derive(Debug)]
#[derive(StructOpt)]
enum Send {
    Can {
        id: u32,
        message: String,
    },
}

#[derive(Debug)]
#[derive(StructOpt)]
enum Receive {
    Can {
        id: u32,
        message: String,
    },
}

#[derive(StructOpt)]
#[structopt(
    name = "rusty-can",
    // NoBinaryName means that clap won't expect the first argument in the
    // list to be the cli binary's path
    setting(clap::AppSettings::NoBinaryName),
    global_setting(clap::AppSettings::ColoredHelp)
)]

#[derive(Debug)]
enum CanCommand {
    Send(Send),
    Receive(Receive),
    Exit,
}


pub enum ParsedCommand {
    Boss(BossCommand),
    Exit,
}


pub fn parse(command: &str) -> anyhow::Result<ParsedCommand> {
    let words = shell_words::split(command)?;
    // StructOpt has a "safe" version as well; errors from this include invalid commands
    // but also just `--help` invocations; it's all fine since we just write!(tcp, "{}", err)
    // and the fmt::Display impl takes care of it all
    //let cmd = CanCommand::from_iter_safe(words);
    let cmd = match CanCommand::from_iter_safe(words) {
        Ok(cmd) => cmd,
        Err(error) => return Err(anyhow!(error.message)),
    };
    //Err(anyhow!("Error qualquer"))

    // println!("{:?} se feliz", cmd);
    macro_rules! c {
        // have $($args)* in order to handle Command::Foo(foo) or Command::Bar { bar: baz }
        ($cmd:ident$($args:tt)*) => {
            ParsedCommand::Boss(BossCommand::$cmd$($args)*)
        };
    }
    let cmd = match cmd {
        CanCommand::Exit => ParsedCommand::Exit,
        CanCommand::Send(send) => match send {
            Send::Can { id, message } => c!(SendCan{ id, message }),
        },
        CanCommand::Receive(receive) => match receive {
            Receive::Can { id, message } => c!(ReceiveCan{ id, message }),
        },
        // about 15 more commands in the real version...
    };

    Ok(cmd)
}