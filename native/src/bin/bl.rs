#[path = "../all.rs"]
mod all;

fn main() {
    let posts = all::load(std::env::current_dir().unwrap());
    for item in posts.sort_by_date().iter() {
        let post = item.post;
        println!(
            "{}{}{}",
            if post.is_future() { "[" } else { " " },
            post.slug(),
            if post.is_future() { "]" } else { "" }
        );
    }
}
