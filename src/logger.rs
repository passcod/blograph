use colored::*;
use env_logger::LogBuilder;
use log::{LogRecord, LogLevel, LogLevelFilter};
use std::env;

pub fn init(verbose: usize) {
    let format = |record: &LogRecord| {
        let level = format!("{: <5}", record.level());
        let levelc = match record.level() {
            LogLevel::Trace => level.dimmed().yellow(),
            LogLevel::Debug => level.black().on_yellow(),
            LogLevel::Info => level.green().bold(),
            LogLevel::Warn => level.magenta().bold(),
            LogLevel::Error => level.red().bold()
        };

        format!("{} {} {}", levelc, record.target().cyan(), record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, match verbose {
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    });

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
