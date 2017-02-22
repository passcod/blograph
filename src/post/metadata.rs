use chrono::prelude::*;
use iso8601;
use num_traits::cast::FromPrimitive;
use super::metadata_parser;
use yaml_rust::Yaml;

#[derive(Debug, Eq, PartialEq)]
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

    pub fn author(&self) -> Option<String> {
        match self.yaml["author"] {
            Yaml::String(ref s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn title(&self) -> Option<String> {
        match self.yaml["title"] {
            Yaml::String(ref s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn parents(&self) -> Vec<String> {
        match self.yaml["parent"] {
            Yaml::String(ref s) => vec![s.clone()],
            _ => match self.yaml["parents"] {
                Yaml::Array(ref v) => v.iter().filter_map(|s: &Yaml| {
                    match s {
                        &Yaml::String(ref s) => Some(s.clone()),
                        _ => None
                    }
                }).collect(),
                _ => vec![]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::prelude::*;
    use super::*;
    use yaml_rust::{yaml, Yaml, YamlLoader};

    fn meta(y: &str) -> Metadata {
        Metadata::from_yaml(YamlLoader::load_from_str(y).unwrap()[0].clone())
    }

    #[test]
    fn page_true() {
        assert_eq!(meta("page: true").page(), true);
    }

    #[test]
    fn page_false() {
        assert_eq!(meta("page: false").page(), false);
    }

    #[test]
    fn page_else() {
        assert_eq!(meta("page: a string").page(), false);
    }

    #[test]
    fn page_missing() {
        assert_eq!(meta("not-page: true").page(), false);
    }

    #[test]
    fn author_there() {
        assert_eq!(meta("author: Me").author(), Some(String::from("Me")));
    }

    #[test]
    fn author_else() {
        assert_eq!(meta("author: true").author(), None);
    }

    #[test]
    fn author_missing() {
        assert_eq!(meta("not-author: true").author(), None);
    }

    #[test]
    fn title_there() {
        assert_eq!(meta("title: A good post").title(), Some(String::from("A good post")));
    }

    #[test]
    fn title_else() {
        assert_eq!(meta("title: true").title(), None);
    }

    #[test]
    fn title_missing() {
        assert_eq!(meta("not-title: true").title(), None);
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

    #[test]
    fn date_missing() {
        assert_eq!(meta("not-date: boo").date(), None);
    }

    #[test]
    fn date_invalid() {
        assert_eq!(meta("date: boo").date(), None);
    }

    #[test]
    fn date_wrong_type() {
        assert_eq!(meta("not-date: false").date(), None);
    }

    #[test]
    fn date_ymd() {
        assert_eq!(meta("date: 2017-02-18").date(),
            Some(UTC.ymd(2017, 2, 18).and_hms(0, 0, 0)));
    }

    #[test]
    fn date_ymd_hms() {
        assert_eq!(meta("date: 2017-02-18T12:34:56Z").date(),
            Some(UTC.ymd(2017, 2, 18).and_hms(12, 34, 56)));
    }

    #[test]
    fn date_ymd_hms_offset() {
        assert_eq!(meta("date: 2017-02-18T12:34:56-13:30").date(),
            Some(UTC.ymd(2017, 2, 19).and_hms(2, 4, 56)));
    }

    #[test]
    fn date_ywd() {
        assert_eq!(meta("date: 2017W123").date(),
            Some(UTC.ymd(2017, 3, 23).and_hms(0, 0, 0)));
    }

    #[test]
    fn date_ywd_hms() {
        assert_eq!(meta("date: 2017W123T12:34:56Z").date(),
            Some(UTC.ymd(2017, 3, 23).and_hms(12, 34, 56)));
    }

    #[test]
    fn date_ywd_hms_offset() {
        assert_eq!(meta("date: 2017W123T12:34:56+13:30").date(),
            Some(UTC.ymd(2017, 3, 22).and_hms(23, 4, 56)));
    }

    #[test]
    fn date_yo() {
        assert_eq!(meta("date: 2017123T").date(),
            Some(UTC.ymd(2017, 5, 3).and_hms(0, 0, 0)));
    }

    #[test]
    fn date_yo_without_trailing_t() {
        assert_eq!(meta("date: 2017123").date(), None);
    }

    #[test]
    fn date_yo_hms() {
        assert_eq!(meta("date: 2017123T12:34:56Z").date(),
            Some(UTC.ymd(2017, 5, 3).and_hms(12, 34, 56)));
    }

    #[test]
    fn date_yo_hms_offset() {
        assert_eq!(meta("date: 2017123T12:34:56-13:30").date(),
            Some(UTC.ymd(2017, 5, 4).and_hms(2, 4, 56)));
    }

    #[test]
    fn parents_single() {
        assert_eq!(meta("parent: hello-world").parents(), vec!["hello-world"]);
    }

    #[test]
    fn parents_single_array() {
        assert_eq!(meta("parent:\n  - help").parents(), vec![] as Vec<String>);
    }

    #[test]
    fn parents_single_bad_type() {
        assert_eq!(meta("parent: true").parents(), vec![] as Vec<String>);
    }

    #[test]
    fn parents_multi() {
        assert_eq!(meta("parents:\n  - hello\n  - world").parents(),
            vec!["hello", "world"]);
    }

    #[test]
    fn parents_multi_one() {
        assert_eq!(meta("parents:\n  - hello").parents(), vec!["hello"]);
    }

    #[test]
    fn parents_multi_bad_type() {
        assert_eq!(meta("parents:\n  - hello\n  - 123").parents(), vec!["hello"]);
    }

    #[test]
    fn parents_shadowed_by_parent() {
        assert_eq!(meta("parent: hello\nparents:\n  - world").parents(), vec!["hello"]);
    }

    #[test]
    fn parents_missing() {
        assert_eq!(meta("not-parent: hello-world").parents(), vec![] as Vec<String>);
    }
}
