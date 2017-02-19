use chrono::prelude::*;
use iso8601;
use num_traits::cast::FromPrimitive;
use regex::Regex;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use yaml_rust::Yaml;

mod date;
mod metadata;

#[cfg(test)] mod test_date;
#[cfg(test)] mod test_is_future;
#[cfg(test)] mod test_is_page;
#[cfg(test)] mod test_slug;

pub struct Post {
    path: PathBuf,
    metadata: Yaml,
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
            metadata: metadata::parse(&raw),
            content: metadata::strip(&raw)
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
        if self.date().is_none() {
            return true;
        }

        match self.metadata["page"] {
            Yaml::Boolean(b) => b,
            _ => false
        }
    }

    #[inline]
    fn meta_date(&self) -> Option<DateTime<UTC>> {
        let iso = match self.metadata["date"] {
            Yaml::String(ref d) => match iso8601::datetime(d) {
                Ok(d) => d,
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
                }
            },
            _ => return None
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

