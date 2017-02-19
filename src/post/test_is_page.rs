use std::path::PathBuf;
use super::*;
use yaml_rust::{yaml, Yaml};

fn metadata_test(path: &str, meta: Yaml, output: bool) {
    let post = Post {
        path: PathBuf::from(path),
        metadata: meta,
        content: String::from("")
    };
    assert_eq!(post.is_page(), output);
}

fn path_test(path: &str, output: bool) {
    metadata_test(path, Yaml::Hash(yaml::Hash::new()), output)
}

#[test]
fn with_file_date() {
    path_test("2017-feb-18-hello-world.md", false);
}

#[test]
fn with_no_date() {
    path_test("hello-world.md", true);
}

#[test]
fn with_metadata_page() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("page")),
        Yaml::Boolean(true)
    );

    metadata_test("2017-feb-18-hello-world.md", Yaml::Hash(meta), true);
}

#[test]
fn with_metadata_date_and_page() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18"))
    );
    meta.insert(
        Yaml::String(String::from("page")),
        Yaml::Boolean(true)
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), true);
}

#[test]
fn with_metadata_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-18"))
    );

    metadata_test("hello-world.md", Yaml::Hash(meta), false);
}

#[test]
fn with_metadata_and_file_date() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-04"))
    );
    meta.insert(
        Yaml::String(String::from("page")),
        Yaml::Boolean(true)
    );

    metadata_test("2017-feb-18-hello-world.md", Yaml::Hash(meta), true)
}

#[test]
fn with_metadata_and_file_date_and_page() {
    let mut meta = yaml::Hash::new();
    meta.insert(
        Yaml::String(String::from("date")),
        Yaml::String(String::from("2017-02-04"))
    );

    metadata_test("2017-feb-18-hello-world.md", Yaml::Hash(meta), false)
}

