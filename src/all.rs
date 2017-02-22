use std::path::PathBuf;
use std::rc::Rc;
use super::list::List;
use super::post::Post;
use walkdir::{DirEntry, Error as WalkDirError, WalkDir, WalkDirIterator};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn filter_file(entry: Result<DirEntry, WalkDirError>, base: &PathBuf) -> Option<PathBuf> {
    if let Ok(file) = entry {
        if !file.file_type().is_file() { return None }

        if let Some(name) = file.file_name().to_str() {
            if !name.ends_with(".md") { return None }
        }

        if let Ok(path) = file.path().strip_prefix(base) {
            return Some(path.to_path_buf());
        }
    }

    None
}

pub fn load(base: PathBuf) -> List {
    let mut posts: Vec<Rc<Post>> = vec![];

    for entry in WalkDir::new(&base)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| filter_file(e, &base)) {
        if let Ok(post) = Post::new(&base, entry) {
            posts.push(Rc::new(post));
        }
    }

    List::new(posts)
}
