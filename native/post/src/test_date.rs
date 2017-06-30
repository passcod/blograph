use chrono::prelude::*;
use std::path::PathBuf;
use super::*;
use super::metadata::Metadata;
use yaml_rust::{yaml, Yaml};

fn metadata_test(path: &str, meta: Yaml, output: Option<DateTime<Utc>>) {
    let post = Post {
        path: PathBuf::from(path),
        metadata: Metadata::from_yaml(meta),
        content: String::from("")
    };
    assert_eq!(post.date(), output);
}

fn path_test(path: &str, output: Option<Date<Utc>>) {
    metadata_test(path, Yaml::Hash(yaml::Hash::new()), match output {
        None => None,
        Some(d) => Some(Utc.ymd(d.year(), d.month(), d.day()).and_hms(0, 0, 0))
    })
}

#[test]
fn with_no_date() {
    path_test("hello-world.md", None);
}

#[test]
fn with_file_alpha_date() {
    path_test("2017-feb-18-hello-world.md", Some(Utc.ymd(2017, 02, 18)));
}

#[test]
fn with_file_number_date() {
    path_test("2017-02-18-hello-world.md", Some(Utc.ymd(2017, 02, 18)));
}

#[test]
fn with_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 02, 18).and_hms(0, 0, 0))
    );
}

#[test]
fn with_metadata_datetime() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18T12:34:56Z"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 02, 18).and_hms(12, 34, 56))
    );
}

#[test]
fn with_metadata_isoweek() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017W321"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 08, 08).and_hms(0, 0, 0))
    );
}

#[test]
fn with_metadata_ordinal() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017321"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 11, 17).and_hms(0, 0, 0))
    );
}

#[test]
fn with_metadata_invalid() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("invalid"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        None
    );
}

#[test]
fn with_metadata_datetime_not_utc() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18T12:34:56+13:30"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 02, 17).and_hms(23, 4, 56))
    );
}

#[test]
fn with_metadata_datetime_milli() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18T12:34:56.789Z"))
    );

    metadata_test(
        "hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 02, 18).and_hms_milli(12, 34, 56, 789))
    );
}

#[test]
fn with_metadata_and_file_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-04"))
    );

    metadata_test(
        "2017-feb-18-hello-world.md",
        Yaml::Hash(meta),
        Some(Utc.ymd(2017, 02, 04).and_hms(0, 0, 0))
    );
}
