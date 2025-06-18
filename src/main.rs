pub mod auth;
pub mod configparse;
pub mod daemon;
pub mod debug;
pub mod keepalive;

#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use anyhow::Result;
use auth::Doggercom;
use clap::{ArgGroup, Parser, ValueEnum};
use configparse::config_parse;
use daemon::daemonize;
use debug::init_logging;
use std::path::PathBuf;

use crate::configparse::{preconfig_variant_jlu, Config};

#[derive(Clone, PartialEq, ValueEnum)]
pub enum ArgMode {
    #[value(name = "dhcp")]
    DHCP,
    #[value(name = "pppoe")]
    PPPoE,
}

#[derive(Clone, PartialEq, ValueEnum)]
pub enum ArgVariant {
    #[value(name = "jlu")]
    JLU,
}

#[derive(Parser)]
#[command(about, version)]
#[clap(group(
    ArgGroup::new("exclusive_args")
        .required(true)
        .multiple(true)
        .args(&["MODE", "VARIANT"])
))]
pub struct Args {
    /// Set your dogcom mode
    #[arg(value_enum, short = 'm', long = "mode", name = "MODE")]
    pub arg_mode: Option<ArgMode>,

    /// Import configuration file
    #[arg(short = 'c', long = "conf", name = "FILEPATH")]
    pub conf: PathBuf,

    /// Bind your ip address
    #[arg(
        short = 'b',
        long = "bindip",
        name = "IPADDR",
        default_value_t = String::from("0.0.0.0")
    )]
    pub bindip: String,

    /// Specify log file
    #[arg(short = 'l', long = "log", name = "LOGPATH")]
    pub log: Option<PathBuf>,

    /// Set daemon flag
    #[arg(short = 'd', long = "daemon")]
    pub daemon: bool,

    /// Enable 802.1x (unimplemented)
    #[arg(short = 'x', long = "802.1x")]
    pub enable_802_1x: bool,

    /// Set eternal flag
    #[arg(short = 'e', long = "eternal")]
    pub eternal: bool,

    /// Set verbose flag
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Relogin interval
    ///
    /// in minutes; 0 = never; default = never
    #[arg(short = 'r', long = "relogin", name = "MINS")]
    pub relogin: Option<u64>,

    /// Preconfigured variant
    #[arg(short = 't', long = "variant", name = "VARIANT")]
    pub variant: Option<ArgVariant>,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    if let Err(err) = init_logging(&args) {
        println!("Logging initialization error: {err}");
    }

    if args.daemon {
        daemonize();
    }

    let config = match args.variant {
        Some(ArgVariant::JLU) => {
            args.arg_mode = args.arg_mode.or(Some(ArgMode::DHCP));
            args.relogin = args.relogin.or(Some(60));
            preconfig_variant_jlu()
        }
        None => Config::default(),
    };
    args.relogin = match args.relogin {
        Some(x) if x > 0 => Some(x),
        _ => None,
    };
    let config = config_parse(config, &args.conf)?;
    if args.enable_802_1x {
        try_smart_eaplogin()?;
    }

    Doggercom::new(args, config)?.run()
}

fn try_smart_eaplogin() -> Result<()> {
    unimplemented!("802.1x login not implemented");
}
