use chrono::prelude::*;
use chrono::Duration;
use std::path::PathBuf;
use super::*;
use yaml_rust::{yaml, Yaml};

fn metadata_test(path: &str, meta: Yaml, output: bool) {
    let post = Post {
        path: PathBuf::from(path),
        raw: String::from(""),
        metadata: meta,
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
fn with_yesterday_file_date() {
    path_test(&(
        UTC::now() - Duration::days(1)
    ).format("%Y-%b-%d-hello-world.md").to_string(), false);
}

#[test]
fn with_tomorrow_file_date() {
    path_test(&(
        UTC::now() + Duration::days(1)
    ).format("%Y-%b-%d-hello-world.md").to_string(), true);
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
fn with_yesterday_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String((
            UTC::now() - Duration::days(1)
        ).format("%Y-%m-%dT%H:%M:%SZ").to_string())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_tomorrow_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String((
            UTC::now() + Duration::days(1)
        ).format("%Y-%m-%dT%H:%M:%SZ").to_string())
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), true);
}

