use chrono::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

mod date;

#[cfg(test)]
mod test_slug;

pub struct Post {
    path: PathBuf,
    raw: String,
    metadata: Yaml,
    content: String,
}

static POSTS: &'static str = "/home/write/blog";

impl Post {
    fn from_path(path: PathBuf) -> Result<Post> {
        let mut abspath = PathBuf::from(POSTS);
        abspath.push(path.clone());

        let mut file = try!(File::open(abspath));
        let mut raw = String::new();
        try!(file.read_to_string(&mut raw));

        Ok(Post {
            path: path,
            raw: raw,
            metadata: Yaml::from_str("123"),
            content: String::from("")
        })
    }

    pub fn slug(&self) -> String {
        lazy_static! {
            static ref EXT: Regex = Regex::new(r"\.[\w.]+$").unwrap();
        }

        let file_date = date::from_path(&self.path);
        let without_date = date::strip(&self.path);
        let without_ext = EXT.replace_all(&without_date, "");

        let slug_date = match file_date {
            Some(d) => format!("{}/", d.format("%Y/%b/%d")).to_lowercase(),
            None => String::from("")
        };

        format!("{}{}", slug_date, without_ext)
    }
}

