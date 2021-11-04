use super::stdin::BossCommand;
use std::str::FromStr;
use structopt::StructOpt;
use anyhow::anyhow;


#[derive(Debug)]
#[derive(StructOpt)]
struct Send {
    id: Option<String>, 
    message: Option<String>,
    cycletime: Option<String>,
}


#[derive(Debug)]
#[derive(StructOpt)]
struct Receive {
    id: Option<String>,
    nrofmessages: Option<String>,
}


#[derive(StructOpt)]
#[structopt(
    name = "rusty-can",
    version = "0.1.0",
    // NoBinaryName means that clap won't expect the first argument in the
    // list to be the cli binary's path
    setting(clap::AppSettings::NoBinaryName),
    global_setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::ArgRequiredElseHelp),
    subcommand(
        clap::SubCommand::with_name("send")
            .about("Used to send can messages")
            .arg(
                clap::Arg::with_name("id")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("i")
                    .long("canid")
                    .help("Id of can message")
                    .default_value("0x40A"),
            )
            .arg(
                clap::Arg::with_name("message")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("m")
                    .long("message")
                    .help("message to be send")
                    .default_value("deadbeef"),
            )
            .arg(
                clap::Arg::with_name("cycletime")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("c")
                    .long("cycletime")
                    .help("cycle time in ms. 0 if not cyclic.")
                    .default_value("0"),
            )
        ),
    subcommand(
        clap::SubCommand::with_name("receive")
            .about("Used to receive can messages")
            .arg(
                clap::Arg::with_name("id")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("i")
                    .long("canid")
                    .help("listen to this can ID")
                    .default_value("0x40A"),
            )
            .arg(
                clap::Arg::with_name("nrofmessages")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("n")
                    .long("nrofmessages")
                    .help("number of messages to receive")
                    .default_value("1"),
            )
        ),
    subcommand(
        clap::SubCommand::with_name("exit")
            .about("exit the cli")
            .help("exit the cli")
        ),
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
    // but also just `--help` invocations; 
    // and the fmt::Display impl takes care of it all
    let cmd = match CanCommand::from_iter_safe(words) {
        Ok(cmd) => cmd,
        Err(error) => return Err(anyhow!(error.message)),
    };


    //println!("{:?} se feliz", cmd);
    macro_rules! c {
        // have $($args)* in order to handle Command::Foo(foo) or Command::Bar { bar: baz }
        ($cmd:ident$($args:tt)*) => {
            ParsedCommand::Boss(BossCommand::$cmd$($args)*)
        };
    }

    let cmd = match cmd {
        CanCommand::Exit => ParsedCommand::Exit,
        CanCommand::Send(sendcmd) => match sendcmd {
            Send { id, message, cycletime} => c!(SendCan{ id, message, cycletime}),
        },
        CanCommand::Receive(receivecmd) => match receivecmd {
            Receive { id, nrofmessages} => c!(ReceiveCan{ id, nrofmessages}),
        },
        // about 15 more commands in the real version...
    };

    Ok(cmd)
}