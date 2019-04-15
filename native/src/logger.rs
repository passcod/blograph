use colored::*;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::{env, io::Write};

pub fn init() {
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            let level = format!("{: <5}", record.level());
            let levelc = match record.level() {
                Level::Trace => level.dimmed().yellow(),
                Level::Debug => level.black().on_yellow(),
                Level::Info => level.green().bold(),
                Level::Warn => level.magenta().bold(),
                Level::Error => level.red().bold(),
            };

            write!(
                buf,
                "{} {} {}",
                levelc,
                record.target().cyan(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Warn);

    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    }

    builder.init();
}
