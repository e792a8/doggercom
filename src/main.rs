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
use auth::doggercom;
use clap::{Parser, ValueEnum};
use configparse::config_parse;
use daemon::daemonize;
use debug::init_logging;
use std::path::PathBuf;

#[derive(Clone, PartialEq, ValueEnum)]
pub enum ArgMode {
    #[value(name = "dhcp")]
    DHCP,
    #[value(name = "pppoe")]
    PPPoE,
}

#[derive(Parser)]
#[command(about, version)]
pub struct Args {
    /// Set your dogcom mode
    #[arg(value_enum, short = 'm', long = "mode", name = "MODE")]
    pub arg_mode: ArgMode,

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
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(err) = init_logging(&args) {
        println!("Logging initialization error: {err}");
    }

    if args.daemon {
        daemonize();
    }
    let config = config_parse(&args.conf)?;
    if args.enable_802_1x {
        try_smart_eaplogin()?;
    }
    doggercom(&args, &config)?;

    Ok(())
}

fn try_smart_eaplogin() -> Result<()> {
    unimplemented!("802.1x login not implemented");
}
