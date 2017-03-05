use list::List;
use post::Post;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::{DirEntry, Error as WalkDirError, WalkDir, WalkDirIterator};

fn is_hidden(entry: &DirEntry) -> bool {
    let hidden = entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false);

    if hidden { trace!("Rejecting hidden file: {:?}", entry); }
    hidden
}

fn filter_file(entry: Result<DirEntry, WalkDirError>, base: &PathBuf) -> Option<PathBuf> {
    if let Ok(file) = entry {
        if !file.file_type().is_file() {
            trace!("Rejecting non-file {:?}", file);
            return None
        }

        if let Some(name) = file.file_name().to_str() {
            if !name.ends_with(".md") {
                trace!("Rejecting non-markdown {}", name);
                return None
            }
        }

        if let Ok(path) = file.path().strip_prefix(base) {
            return Some(path.to_path_buf());
        }

        trace!("Rejecting {:?}", file);
    }

    None
}

pub fn load(base: PathBuf) -> List {
    let mut posts: Vec<Arc<Post>> = vec![];
    info!("Looking for posts in {:?}", base);

    for entry in WalkDir::new(&base)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| filter_file(e, &base)) {

        debug!("Loading {:?}/{:?}", base, entry);
        if let Ok(post) = Post::new(&base, entry) {
            trace!("Loaded post with slug: {}", post.slug());
            posts.push(Arc::new(post));
        }
    }

    debug!("Found {} posts", posts.len());
    List::new(posts)
}
