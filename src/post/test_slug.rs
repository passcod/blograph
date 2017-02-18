use std::path::PathBuf;
use super::*;
use yaml_rust::Yaml;

fn slug_test(input: &str, output: &str) {
    let post = Post {
        path: PathBuf::from(input),
        raw: String::from(""),
        metadata: Yaml::from_str("123"),
        content: String::from("")
    };
    assert_eq!(post.slug(), output);
}

#[test]
fn with_single_ext() {
    slug_test("2017-feb-18-hello-world.md", "2017/feb/18/hello-world");
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
    slug_test("2017-feb-18-hello-world.md", "hello-world");
}

#[test]
fn with_metadata_date_hidden_by_metadata() {
    slug_test("hello-world.md", "hello-world");
}

#[test]
fn with_metadata_date() {
    slug_test("hello-world.md", "2017/feb/18/hello-world");
}

#[test]
fn with_metadata_and_file_date() {
    slug_test("2017-feb-18-hello-world.md", "hello-world");
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

