use crate::Args;
use anyhow::Result;
use log::LevelFilter;
use simplelog::{CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::{fmt::Display, fs::File};

pub struct DisplayBytes<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> Display for DisplayBytes<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = self.0.as_ref().iter().peekable();
        write!(f, "[")?;
        while let Some(x) = i.next() {
            write!(f, "{:02x}", x)?;
            if i.peek().is_some() {
                write!(f, " ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub fn init_logging(args: &Args) -> Result<()> {
    let logger_config = simplelog::Config::default();
    let term_logger = TermLogger::new(
        if args.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        logger_config.clone(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
    if let Some(ref log) = args.log {
        let write_logger = WriteLogger::new(
            LevelFilter::Debug,
            logger_config,
            File::options().append(true).create(true).open(log).unwrap(),
        );
        CombinedLogger::init(vec![term_logger, write_logger]).unwrap();
    } else {
        CombinedLogger::init(vec![term_logger]).unwrap();
    }

    Ok(())
}
