use super::*;
use post::{Metadata, Post};
use std::sync::Arc;

pub fn make_post(path: &str, meta: &str) -> Arc<Post> {
    Arc::new(Post::from(path, Metadata::parse(meta), ""))
}

pub fn make_list() -> List {
    let mut posts = vec![];

    posts.push(make_post(
        "2015-jan-25-300-shorts.md",
        "---
tags:
  - preface
  - 300-shorts
    ",
    ));

    posts.push(make_post(
        "2017-feb-05-some-predictions.md",
        "---
tags:
  - post
  - predictions
    ",
    ));

    posts.push(make_post(
        "2017-jan-03-mountain.md",
        "---
tags:
  - short
  - 300-shorts
parents:
  - \"2015/jan/25/300-shorts\"
    ",
    ));

    posts.push(make_post(
        "2017-jan-04-there-is-no-such-thing-as-writing-for-adults.md",
        "---
tags:
  - thought
  - education
  - writing
    ",
    ));

    posts.push(make_post(
        "2017-feb-10-monthly-update.md",
        "---
tags:
  - monthly-update
    ",
    ));

    posts.push(make_post(
        "2017-mar-10-monthly-update.md",
        "---
tags:
  - monthly-update
parents:
  - \"2017/feb/10/monthly-update\"
    ",
    ));

    List::new(posts)
}
