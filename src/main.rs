extern crate brotli2;
extern crate chrono;
extern crate crowbook_text_processing;
extern crate futures;
extern crate iso8601;
#[macro_use] extern crate lazy_static;
extern crate num_traits;
extern crate pulldown_cmark;
extern crate regex;
extern crate walkdir;
extern crate yaml_rust;
extern crate zopfli;

use std::path::PathBuf;

mod post;
mod realised;
mod render;

fn main() {
	realised::posts(PathBuf::from("/home/write/blog"));
    println!("Hello, world!");
}
