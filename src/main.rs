#[macro_use] extern crate clap;
extern crate colored;
extern crate env_logger;
extern crate hyper;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate time;
extern crate url;
extern crate regex;
extern crate walkdir;
extern crate yaml_rust;
extern crate list;
extern crate post;

use list::List;
use std::path::PathBuf;
use std::process;

mod all;
mod logger;
mod router;
mod server;

fn main() {
    let args = clap_app!(myapp =>
        (@arg posts: --posts +takes_value +required "Path to posts directory")
        (@arg theme: --theme +takes_value +required "Path to theme directory")
        (@arg port: -p --port +takes_value "Port to listen on [6920]")
        (@arg verbose: -v ... "Sets the level of verbosity")
    ).get_matches();

    logger::init(args.occurrences_of("verbose") as usize);
    trace!("Set verbose level {}", args.occurrences_of("verbose"));
    info!("Booting up");

    let port = args.value_of("port")
        .unwrap_or("6920")
        .parse::<u16>()
        .ok()
        .unwrap_or(0);

    if port == 0 {
        error!("Bad format for --port, aborting");
        process::exit(1);
    } else {
        debug!("Port: {}", port);
    }

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

    let server = server::init(port.clone());
    let handler = router::Handler::new(all);

    trace!("Starting server");
    if let Err(err) = server.handle(handler) {
        error!("Failed to start server!");
        error!("{}", err);
        process::exit(1);
    } else {
        info!("Listening on port {}", port);
    }
}
