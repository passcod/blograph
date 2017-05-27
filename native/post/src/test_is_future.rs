use chrono::prelude::*;
use std::path::PathBuf;
use super::*;
use super::metadata::Metadata;
use yaml_rust::{yaml, Yaml};

fn metadata_test(path: &str, meta: Yaml, output: bool) {
    let post = Post {
        path: PathBuf::from(path),
        metadata: Metadata::from_yaml(meta),
        content: String::from("")
    };
    assert_eq!(post.is_future(), output);
}

fn path_test(path: &str, output: bool) {
    metadata_test(path, Yaml::Hash(yaml::Hash::new()), output)
}

#[test]
fn with_no_date() {
    path_test("hello-world.md", false);
}

#[test]
fn with_today_file_date() {
    path_test(&UTC::now().format("%Y-%b-%d-hello-world.md").to_string(), false);
}

#[test]
fn with_past_file_date() {
    path_test("2010-jan-01-hello-world.md", false);
}

#[test]
fn with_future_file_date() {
    path_test("3010-jan-01-hello-world.md", true);
}

#[test]
fn with_today_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(UTC::now().format("%Y-%m-%dT%H:%M:%SZ").to_string())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_past_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("2010-01-01".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_past_metadata_datetime() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("2010-01-01T12:34:56Z".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_past_metadata_datetimezone() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("2010-01-01T12:34:56+13:00".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_future_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("3010-01-01".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), true);
}

#[test]
fn with_future_metadata_datetime() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("3010-01-01T12:34:56Z".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), true);
}

#[test]
fn with_future_metadata_datetimezone() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String("3010-01-01T12:34:56+13:00".into())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), true);
}

