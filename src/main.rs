extern crate chrono;
#[macro_use] extern crate clap;
extern crate colored;
extern crate crowbook_text_processing;
extern crate env_logger;
extern crate iso8601;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate num_traits;
extern crate pulldown_cmark;
extern crate regex;
extern crate walkdir;
extern crate yaml_rust;

use list::List;
use std::path::PathBuf;
use std::process;

mod all;
mod list;
mod logger;
mod post;

fn main() {
    let args = clap_app!(myapp =>
        (@arg posts: -p --posts +takes_value +required "Path to posts directory")
        (@arg theme: -t --theme +takes_value +required "Path to theme directory")
        (@arg verbose: -v ... "Sets the level of verbosity")
    ).get_matches();

    logger::init(args.occurrences_of("verbose") as usize);
    debug!("Set verbose level {}", args.occurrences_of("verbose"));
    info!("Booting up");

    let posts = PathBuf::from(args.value_of("posts").unwrap());
    let theme = PathBuf::from(args.value_of("theme").unwrap());
    debug!("Posts: {:?}", posts);
    debug!("Theme: {:?}", theme);

    if !posts.exists() {
        error!("{:?} does not exist, aborting", posts);
        process::exit(1);
    } else {
        debug!("Checked {:?} exists", posts);
    }

    if !theme.exists() {
        error!("{:?} does not exist, aborting", theme);
        process::exit(1);
    } else {
        debug!("Checked {:?} exists", theme);
    }

    info!("Loading posts from {:?}", posts);
    let all = all::load(posts);
    info!("Loaded {} posts", all.len());

    let frontpage = all.iter().filter(|item| {
        !item.post.is_future() &&
        !item.post.is_page()
    }).collect::<List>().sort_by_date();

    let mut taglists: Vec<(String, List)> = vec![];
    for tag in all.tags() {
        taglists.push((tag.clone(), all.iter().filter(|item| {
            item.post.metadata.tags().iter().any(|t| t == &tag)
        }).collect::<List>().sort_by_date()));
    }

    // println!("Front page:");
    // for item in frontpage.iter() {
    //     println!("  - {}", item.post.slug());
    // }

    // for (tag, list) in taglists {
    //     println!("\nTag {}:", tag);
    //     for item in list.iter() {
    //         println!("  - {}", item.post.slug());
    //     }
    // }

    debug!("Shorts:");
    let preface = all.find_by_slug("2015/jan/25/300-shorts").unwrap();
    for item in all.children_of(preface).sort_by_date().iter() {
        debug!("  - {}", item.post.slug());
    }
}
