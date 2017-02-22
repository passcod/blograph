extern crate chrono;
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

use env_logger::LogBuilder;
use list::List;
use std::path::PathBuf;

mod all;
mod list;
mod logger;
mod post;

fn main() {
    logger::init();
    info!("Booting up");

    let posts = PathBuf::from("/home/write/blog");
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

    println!("\nShorts:");
    let preface = all.find_by_slug("2015/jan/25/300-shorts").unwrap();
    for item in all.children_of(preface).sort_by_date().iter() {
        println!("  - {}", item.post.slug());
    }
}
