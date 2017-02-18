use chrono::prelude::*;
use std::path::PathBuf;
use super::*;
use yaml_rust::{yaml, Yaml};

fn metadata_test(path: &str, meta: Yaml, output: DateTime<UTC>) {
    let post = Post {
        path: PathBuf::from(path),
        raw: String::from(""),
        metadata: meta,
        content: String::from("")
    };
    assert_eq!(post.date(), output);
}

fn path_test(path: &str, output: Date<UTC>) {
    metadata_test(path, Yaml::Hash(yaml::Hash::new()), output.and_hms(0, 0, 0))
}

#[test]
fn with_file_date() {
    date_test("2017-feb-18-hello-world.md", "2017/feb/18/hello-world");
}

#[test]
fn with_double_ext() {
    slug_test("2017-feb-18-hello-world.pre.md", "2017/feb/18/hello-world");
}

#[test]
fn with_longer_prefix() {
    slug_test("2017-feb-18-hello-world.md", "2017/feb/18/hello-world");
}

#[test]
fn with_file_date_hidden_by_metadata() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("page")),
        Yaml::Boolean(true)
    );

    metadata_test("2017-feb-18-hello-world.md", Yaml::Hash(meta), "hello-world");
}

#[test]
fn with_metadata_date_hidden_by_metadata() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18"))
    );
    meta.insert(
        Yaml::String(String::from("page")),
        Yaml::Boolean(true)
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), "hello-world");
}

#[test]
fn with_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18"))
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), "2017/feb/18/hello-world");
}

#[test]
fn with_metadata_and_file_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-04"))
    );

    metadata_test("2017-feb-18-hello-world.md", Yaml::Hash(meta), "2016/feb/04/hello-world")
}

#[test]
fn with_two_dates() {
    slug_test("2017-feb-18/2016-jul-03-hello-world.md", "2016/jul/03/hello-world");
}

#[test]
fn with_no_date() {
    slug_test("hello-world.md", "hello-world");
}

#[test]
fn with_date_subpath() {
    slug_test("2017-feb-18/master-kylo/chapter-01.md", "2017/feb/18/master-kylo/chapter-01");
}

#[test]
fn with_dated_subpath() {
    slug_test("2017-feb-18-master-kylo/chapter-01.md", "2017/feb/18/master-kylo/chapter-01");
}

#[test]
fn with_subpath() {
    slug_test("master-kylo/2017-feb-18-chapter-01.md", "2017/feb/18/master-kylo/chapter-01");
}

#[test]
fn with_subpath_and_no_date() {
    slug_test("master-kylo/chapter-01.md", "master-kylo/chapter-01");
}

