use chrono::prelude::*;
use regex::Regex;
use self::metadata::Metadata;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use yaml_rust::Yaml;

mod date;
mod metadata;
mod metadata_parser;

#[cfg(test)] mod test_date;
#[cfg(test)] mod test_is_future;
#[cfg(test)] mod test_is_page;
#[cfg(test)] mod test_slug;

pub struct Post {
    path: PathBuf,
    metadata: Metadata,
    content: String,
}

impl Post {
    pub fn new(base: PathBuf, path: PathBuf) -> Result<Post> {
        let mut abspath = base.clone();
        abspath.push(path.clone());

        let mut file = try!(File::open(abspath));
        let mut raw = String::new();
        try!(file.read_to_string(&mut raw));

        Ok(Post {
            path: path,
            metadata: Metadata::parse(&raw),
            content: metadata_parser::strip(&raw)
        })
    }

    pub fn is_future(&self) -> bool {
        match self.date() {
            None => false,
            Some(d) => {
                d.timestamp() > UTC::now().timestamp()
            }
        }
    }

    pub fn is_page(&self) -> bool {
        match self.date() {
            None => true,
            Some(_) => self.metadata.page()
        }
    }

    pub fn date(&self) -> Option<DateTime<UTC>> {
        match self.metadata.date() {
            Some(d) => Some(d),
            None => date::from_path(&self.path)
        }
    }

    pub fn slug(&self) -> String {
        lazy_static! {
            static ref EXT: Regex = Regex::new(r"\.[\w.]+$").unwrap();
        }

        let without_date = date::strip(&self.path);
        let without_ext = EXT.replace_all(&without_date, "");

        let slug_date = match self.is_page() {
            true => None,
            false => match self.date() {
                Some(d) => Some(d),
                None => unreachable!()
                // if self.date() is None, then self.is_page() is true
            }
        };

        match slug_date {
            Some(d) => format!(
                "{}/{}",
                d.format("%Y/%b/%d"),
                without_ext
            ).to_lowercase(),
            None => String::from(without_ext)
        }
    }
}

