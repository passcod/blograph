use chrono::prelude::*;
use regex::Regex;
use self::capitalise::capitalise;
use self::metadata::Metadata;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use yaml_rust::Yaml;

mod capitalise;
mod date;
mod metadata;
mod metadata_parser;

#[cfg(test)] mod test_date;
#[cfg(test)] mod test_is_future;
#[cfg(test)] mod test_is_page;
#[cfg(test)] mod test_slug;
#[cfg(test)] mod test_title;

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

    fn extless_path(&self) -> String {
        lazy_static! {
            static ref EXT: Regex = Regex::new(r"\.[\w.]+$").unwrap();
        }

        let dateless = date::strip(&self.path);
        String::from(EXT.replace_all(&dateless, ""))
    }

    pub fn slug(&self) -> String {
        let slug_date = match self.is_page() {
            true => None,
            false => match self.date() {
                Some(d) => Some(d),
                None => unreachable!()
                // if self.date() is None, then self.is_page() is true
            }
        };

        let extless = self.extless_path();
        match slug_date {
            Some(d) => format!(
                "{}/{}",
                d.format("%Y/%b/%d"),
                extless
            ).to_lowercase(),
            None => String::from(extless)
        }
    }

    pub fn title(&self) -> String {
        lazy_static! {
            static ref SPACED: Regex = Regex::new("-+").unwrap();
        }

        match self.metadata.title() {
            Some(s) => s,
            None => capitalise(
                &String::from(SPACED.replace_all(&self.extless_path(), " "))
            )
        }
    }
}