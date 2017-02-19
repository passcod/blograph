use chrono::prelude::*;
use iso8601;
use num_traits::cast::FromPrimitive;
use super::metadata_parser;
use regex::Regex;
use yaml_rust::{yaml, Yaml, YamlLoader};

pub struct Metadata {
    yaml: Yaml
}

impl Metadata {
    pub fn parse(raw: &str) -> Metadata {
        Metadata { yaml: metadata_parser::parse(raw) }
    }

    pub fn from_yaml(yaml: Yaml) -> Metadata {
        Metadata { yaml: yaml }
    }

    pub fn page(&self) -> bool {
        match self.yaml["page"].as_bool() {
            None => false,
            Some(p) => p
        }
    }

    pub fn tags(&self) -> Vec<String> {
        match self.yaml["tags"].as_vec() {
            None => vec![],
            Some(v) => v.iter().filter_map(|ys: &Yaml| {
                match ys {
                    &Yaml::String(ref s) => Some(s.clone()),
                    _ => None
                }
            }).collect()
        }
    }

    pub fn date(&self) -> Option<DateTime<UTC>> {
        let iso = match self.yaml["date"] {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use yaml_rust::{yaml, Yaml, YamlLoader};

    fn meta(y: &str) -> Metadata {
        Metadata::from_yaml(YamlLoader::load_from_str(y).unwrap()[0].clone())
    }

    #[test]
    fn path_true() {
        assert_eq!(meta("page: true").page(), true);
    }

    #[test]
    fn path_false() {
        assert_eq!(meta("page: false").page(), false);
    }

    #[test]
    fn path_else() {
        assert_eq!(meta("page: a string").page(), false);
    }

    #[test]
    fn path_missing() {
        assert_eq!(meta("not-page: true").page(), false);
    }

    #[test]
    fn tags_one() {
        assert_eq!(meta("tags:\n  - one").tags(), vec!["one"]);
    }

    #[test]
    fn tags_several() {
        assert_eq!(meta("tags:
                        - one
                        - two
                        - three").tags(), vec!["one", "two", "three"]);
    }

    #[test]
    fn tags_none() {
        let nop: Vec<String> = vec![];
        assert_eq!(meta("tags:").tags(), nop);
    }

    #[test]
    fn tags_else() {
        let nop: Vec<String> = vec![];
        assert_eq!(meta("tags: a string").tags(), nop);
    }

    #[test]
    fn tags_missing() {
        let nop: Vec<String> = vec![];
        assert_eq!(meta("not-tags:\n- shhh").tags(), nop);
    }
}
