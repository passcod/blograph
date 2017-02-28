#[macro_use] extern crate clap;
extern crate colored;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use] extern crate lazy_static;
extern crate list;
#[macro_use] extern crate log;
extern crate post;
extern crate time;
extern crate url;
extern crate regex;
extern crate walkdir;

use std::path::PathBuf;
use std::process;

mod all;
mod logger;
mod server;

fn main() {
    logger::init(args.occurrences_of("verbose") as usize);
    trace!("Set verbose level {}", args.occurrences_of("verbose"));
    info!("Booting up");

    let posts = PathBuf::from(args.value_of("posts").unwrap());
    let theme = PathBuf::from(args.value_of("theme").unwrap());
    debug!("Posts: {:?}", posts);
    debug!("Theme: {:?}", theme);

    if !posts.exists() {
        error!("{:?} does not exist, aborting", posts);
        process::exit(1);
    } else {
        trace!("Checked {:?} exists", posts);
    }

    if !theme.exists() {
        error!("{:?} does not exist, aborting", theme);
        process::exit(1);
    } else {
        trace!("Checked {:?} exists", theme);
    }

    trace!("Loading posts from {:?}", posts);
    let all = all::load(posts);
    info!("Loaded {} posts", all.len());
}
