use std::path::{Path, PathBuf};
use std::rc::Rc;
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

pub fn posts(base: PathBuf) -> Vec<Rc<RealisedPost>> {
	let mut posts: Vec<Rc<RealisedPost>> = vec![];

	for entry in WalkDir::new(&base)
		.follow_links(true)
		.into_iter()
		.filter_entry(|e| !is_hidden(e))
		.filter_map(|e| filter_file(e, &base)) {
		if let Ok(post) = Post::new(&base, entry) {
			let mut realised = RealisedPost::new(post);
			realised.set_previous(posts.last());
			println!("{:?}, prev? {}", realised.slug, realised.has_previous());
			posts.push(Rc::new(realised));
		}
	}

	posts
}

#[derive(Debug)]
pub struct RealisedPost {
    pub post: Post,
	slug: String,
    previous: Option<Rc<RealisedPost>>,
    next: Option<Rc<RealisedPost>>,
    parents: Vec<Rc<RealisedPost>>,
    children: Vec<Rc<RealisedPost>>,
}

impl RealisedPost {
    fn new(post: Post) -> RealisedPost {
		RealisedPost {
			slug: post.slug(),
			post: post,
			previous: None,
			next: None,
			parents: vec![],
			children: vec![]
		}
	}

	fn set_previous(&mut self, prev: Option<&Rc<RealisedPost>>) {
		if let Some(post) = prev {
			self.previous = Some(post.clone());
		}
	}

	fn set_next(&mut self, next: Option<&Rc<RealisedPost>>) {
		if let Some(post) = next {
			self.next = Some(post.clone());
		}
	}

	fn has_previous(&self) -> bool {
		self.previous.is_some()
	}

	fn has_next(&self) -> bool {
		self.next.is_some()
	}
}
