use chrono::prelude::*;
use iso8601;
use num_traits::cast::FromPrimitive;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

mod date;

#[cfg(test)] mod test_date;
#[cfg(test)] mod test_is_page;
#[cfg(test)] mod test_slug;

pub struct Post {
    path: PathBuf,
    raw: String,
    metadata: Yaml,
    content: String,
}

static POSTS: &'static str = "/home/write/blog";

impl Post {
    pub fn from_path(path: PathBuf) -> Result<Post> {
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

    pub fn is_page(&self) -> bool {
        if self.date().is_none() {
            return true;
        }

        match self.metadata["page"] {
            Yaml::BadValue => false,
            ref page @ _ => match page.as_bool() {
                None => false,
                Some(b) => b
            }
        }
    }

    #[inline]
    fn meta_date(&self) -> Option<DateTime<UTC>> {
        let iso = match self.metadata["date"] {
            Yaml::BadValue => return None,
            ref date @ _ => match date.as_str() {
                None => return None,
                Some(d) => match iso8601::datetime(d) {
                    Err(_) => match iso8601::date(d) {
                        Err(_) => return None,
                        Ok(d) => iso8601::DateTime {
                            date: d,
                            time: iso8601::Time {
                                hour: 0,
                                minute: 0,
                                second: 0,
                                millisecond: 0,
                                tz_offset_hours: 0,
                                tz_offset_minutes: 0
                            }
                        }
                    },
                    Ok(d) => d
                }
            }
        };

        let tz = match FixedOffset::east_opt(
            iso.time.tz_offset_hours * 3600 +
            iso.time.tz_offset_minutes * 60
        ) {
            None => return None,
            Some(t) => t
        };

        let tzed = match iso.date {
            iso8601::Date::YMD { year, month, day }
                => tz.ymd_opt(year, month, day),
            iso8601::Date::Week { year, ww, d }
                => tz.isoywd_opt(year, ww, match Weekday::from_u32(d) {
                    None => return None,
                    Some(d) => d
                }),
            iso8601::Date::Ordinal { year, ddd }
                => tz.yo_opt(year, ddd)
        }.and_hms_milli_opt(
            iso.time.hour,
            iso.time.minute,
            iso.time.second,
            iso.time.millisecond
        ).earliest();

        match tzed {
            None => None,
            Some(d) => Some(d.with_timezone(&UTC))
        }
    }

    pub fn date(&self) -> Option<DateTime<UTC>> {
        match self.meta_date() {
            Some(d) => Some(d),
            None => date::from_path(&self.path)
        }
    }

    pub fn slug(&self) -> String {
        lazy_static! {
            static ref EXT: Regex = Regex::new(r"\.[\w.]+$").unwrap();
        }

        let file_date = date::from_path(&self.path);
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

