use super::metadata::Metadata;
use super::*;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

fn meta(y: &str) -> Metadata {
    Metadata::from_yaml(YamlLoader::load_from_str(y).unwrap()[0].clone())
}

fn metadata_test(path: &str, metastr: &str, output: &str) {
    let post = Post {
        path: PathBuf::from(path),
        metadata: meta(metastr),
        content: String::from(""),
    };
    assert_eq!(post.title(), output);
}

fn path_test(input: &str, output: &str) {
    metadata_test(input, "foo: bar", output)
}

#[test]
fn from_path() {
    path_test("2017-feb-18-hello-world.md", "Hello world");
}

#[test]
fn from_mixedcase_path() {
    path_test("2017-feb-18-hello-BRAVE-World.md", "Hello brave world");
}

#[test]
fn from_metadata() {
    metadata_test(
        "hello-world.md",
        "title: Hello Mr. Universe",
        "Hello Mr. Universe",
    );
}
#[test]
fn from_bad_metadata() {
    metadata_test("hello-world.md", "title: false", "Hello world");
}
