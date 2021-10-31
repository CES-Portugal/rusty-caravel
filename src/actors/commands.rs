use super::stdin::BossCommand;
use std::str::FromStr;
use structopt::StructOpt;
use anyhow::anyhow;


// #[derive(Debug)]
// #[derive(StructOpt)]
// struct Receive {
//     Can {
//         id: u32,
//         message: String,
//         cycle_time: Option<String>,
//     },
// }

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
                clap::Arg::with_name("cyclic")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("c")
                    .long("cyclic")
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
                clap::Arg::with_name("number of messages")
                    .takes_value(true)
                    .multiple(false)
                    .required(false)
                    .short("n")
                    .long("messages")
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
    Send {
        id: Option<String>, 
        message: Option<String>, 
        cycle_time: Option<String>
    },
    #[structopt(name = "receive")]
    Receive {
        #[structopt(short = "i")]
        id: Option<String>,
        #[structopt(short = "n")]
        nr_of_messages: Option<String>
    },
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

    println!("{:?} se feliz", cmd);
    macro_rules! c {
        // have $($args)* in order to handle Command::Foo(foo) or Command::Bar { bar: baz }
        ($cmd:ident$($args:tt)*) => {
            ParsedCommand::Boss(BossCommand::$cmd$($args)*)
        };
    }
    let cmd = match cmd {
        CanCommand::Exit => ParsedCommand::Exit,
        CanCommand::Send { id, cycle_time, message} => c!(SendCan{ id, cycle_time, message }),
        CanCommand::Receive { id, nr_of_messages  } => c!(ReceiveCan{ id, nr_of_messages }),
        // about 15 more commands in the real version...
    };

    Ok(cmd)
}