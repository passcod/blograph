use chrono::prelude::*;
use regex::Regex;
use std::path::PathBuf;

static DATE_RX: &'static str = r"(?x)
    -?
    (?P<year>\d{4})               # 4-digit year
    [-/]
    (?P<month>\d{1,2}             # 1- or 2-digit month
        | jan | feb | mar | apr
        | may | jun | jul | aug   # or 3-letter named
        | sep | oct | nov | dec
    )
    [-/]
    (?P<day>\d{1,2})              # 1- or 2-digit day
    -?
";

pub fn from_path(path: &PathBuf) -> Option<DateTime<Utc>> {
    lazy_static! {
        static ref DATE: Regex = Regex::new(DATE_RX).unwrap();
    }

    match DATE
        .find_iter(&String::from(path.to_str().unwrap()).to_lowercase())
        .last()
    {
        None => None,
        Some(date) => match DATE.captures(date.as_str()) {
            None => None,
            Some(caps) => Some(
                Utc.ymd(
                    // The unwraps are safe given the regex matches digits.
                    (&caps["year"]).parse::<i32>().unwrap(),
                    match &caps["month"] {
                        "jan" => 1,
                        "feb" => 2,
                        "mar" => 3,
                        "apr" => 4,
                        "may" => 5,
                        "jun" => 6,
                        "jul" => 7,
                        "aug" => 8,
                        "sep" => 9,
                        "oct" => 10,
                        "nov" => 11,
                        "dec" => 12,
                        m @ _ => m.parse::<u32>().unwrap(),
                    },
                    (&caps["day"]).parse::<u32>().unwrap(),
                )
                .and_hms(0, 0, 0),
            ),
        },
    }
}

pub fn strip(path: &PathBuf) -> String {
    lazy_static! {
        static ref DATE: Regex = Regex::new(DATE_RX).unwrap();
        static ref SLASHED: Regex = Regex::new(r"-*/-*").unwrap();
        static ref DASHED: Regex = Regex::new(r"(^-|-$|-{2,}|^/)").unwrap();
    }

    let lowered = String::from(path.to_str().unwrap()).to_lowercase();

    DASHED
        .replace_all(
            &String::from(SLASHED.replace_all(&String::from(DATE.replace_all(&lowered, "-")), "/")),
            "",
        )
        .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn with_alpha_month() {
        assert_eq!(
            from_path(&PathBuf::from("2017-feb-18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_upper_month() {
        assert_eq!(
            from_path(&PathBuf::from("2017-FEB-18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_mixed_month() {
        assert_eq!(
            from_path(&PathBuf::from("2017-Feb-18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_number_month() {
        assert_eq!(
            from_path(&PathBuf::from("2017-02-18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_slashes() {
        assert_eq!(
            from_path(&PathBuf::from("2017/02/18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn as_infix() {
        assert_eq!(
            from_path(&PathBuf::from("posts/2017-feb-18-hello-world.md")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_prefix() {
        assert_eq!(
            from_path(&PathBuf::from("posts/2017-feb-18")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_suffix() {
        assert_eq!(
            from_path(&PathBuf::from("2017-feb-18-hello-world.pre.md")),
            Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_two_dates() {
        assert_eq!(
            from_path(&PathBuf::from("2017-feb-18/2016-jul-03-hello-world.md")),
            Some(Utc.ymd(2016, 07, 03).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn with_no_date() {
        assert_eq!(from_path(&PathBuf::from("hello-world.md")), None);
    }

    #[test]
    fn strip_infix() {
        assert_eq!(
            strip(&PathBuf::from("posts/2017-feb-18-hello-world.md")),
            String::from("posts/hello-world.md")
        );
    }

    #[test]
    fn strip_suffix() {
        assert_eq!(
            strip(&PathBuf::from("posts/2017-feb-18")),
            String::from("posts/")
        );
    }

    #[test]
    fn strip_front_slash() {
        assert_eq!(
            strip(&PathBuf::from("/2017-feb-18-hello")),
            String::from("hello")
        );
    }

    #[test]
    fn strip_prefix() {
        assert_eq!(
            strip(&PathBuf::from("2017-feb-18-hello-world.pre.md")),
            String::from("hello-world.pre.md")
        );
    }

    #[test]
    fn strip_two_dates() {
        assert_eq!(
            strip(&PathBuf::from("2017-feb-18-and-2016-jul-03-hello-world.md")),
            String::from("and-hello-world.md")
        );
    }

    #[test]
    fn strip_two_dates_subpath() {
        assert_eq!(
            strip(&PathBuf::from("2017-feb-18-hello/2016-jul-03-world.md")),
            String::from("hello/world.md")
        );
    }

    #[test]
    fn strip_two_dates_subpath_bare() {
        assert_eq!(
            strip(&PathBuf::from("2017-feb-18/2016-jul-03-hello-world.md")),
            String::from("hello-world.md")
        );
    }

    #[test]
    fn strip_no_date() {
        assert_eq!(
            strip(&PathBuf::from("hello-world.md")),
            String::from("hello-world.md")
        );
    }

    #[test]
    fn strip_not_a_date() {
        assert_eq!(
            strip(&PathBuf::from("not-2017-and-18")),
            String::from("not-2017-and-18")
        );
    }

    #[test]
    fn strip_just_date() {
        assert_eq!(strip(&PathBuf::from("2017-feb-18")), String::from(""));
    }
}
