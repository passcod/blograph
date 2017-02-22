use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
use super::post::Post;

#[derive(Debug, Eq, PartialEq)]
pub struct List {
    posts: Vec<Rc<Post>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Iter<'a> {
    list: &'a List,
    cursor: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Item {
    pub post: Rc<Post>,
    pub previous: Option<Rc<Post>>,
    pub next: Option<Rc<Post>>,
}

impl List {
    pub fn new(posts: Vec<Rc<Post>>) -> List {
        List { posts: posts }
    }

    pub fn iter(&self) -> Iter {
        Iter { list: self, cursor: 0 }
    }

    pub fn len(&self) -> usize {
        self.posts.len()
    }

    pub fn contains(&self, post: &Rc<Post>) -> bool {
        self.posts.iter().any(|p| p == post)
    }

    pub fn find_by_slug(&self, slug: &str) -> Option<&Rc<Post>> {
        self.posts.iter().find(|p| p.slug() == slug)
    }

    pub fn parents_of(&self, post: &Rc<Post>) -> List {
        let mut parents = vec![];
        for slug in post.metadata.parents() {
            if let Some(parent) = self.find_by_slug(&slug) {
                parents.push(parent.clone())
            }
        }

        List::new(parents)
    }

    pub fn children_of(&self, post: &Rc<Post>) -> List {
        let mut children = vec![];
        for item in self.posts.iter() {
            for parent in item.metadata.parents() {
                if parent == post.slug() {
                    children.push(item.clone())
                }
            }
        }

        List::new(children)
    }

    pub fn sort_by_date(&self) -> List {
        let mut sorted = self.posts.clone();
        sorted.sort_by(|a, b| {
            a.date().cmp(&b.date())
        });
        List::new(sorted)
    }

    pub fn tags(&self) -> HashSet<String> {
        let mut tags = HashSet::new();
        for post in self.posts.iter() {
            for tag in post.metadata.tags() {
                tags.insert(tag);
            }
        }

        tags
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        if let Some(current) = self.list.posts.get(self.cursor) {
            let next = self.list.posts.get(self.cursor + 1);
            let previous = match self.cursor {
                0 => None,
                _ => self.list.posts.get(self.cursor - 1)
            };

            self.cursor += 1;

            Some(Item {
                post: current.clone(),
                previous: match previous {
                    None => None,
                    Some(rc) => Some(rc.clone())
                },
                next: match next {
                    None => None,
                    Some(rc) => Some(rc.clone())
                },
            })
        } else {
            None
        }
    }
}

impl FromIterator<Post> for List {
    fn from_iter<I: IntoIterator<Item=Post>>(iter: I) -> Self {
        let mut posts = vec![];

        for post in iter {
            posts.push(Rc::new(post));
        }

        Self::new(posts)
    }
}

impl FromIterator<Rc<Post>> for List {
    fn from_iter<I: IntoIterator<Item=Rc<Post>>>(iter: I) -> Self {
        let mut posts = vec![];

        for post in iter {
            posts.push(post);
        }

        Self::new(posts)
    }
}

impl<'a> FromIterator<&'a Rc<Post>> for List {
    fn from_iter<I: IntoIterator<Item=&'a Rc<Post>>>(iter: I) -> Self {
        let mut posts = vec![];

        for post in iter {
            posts.push(post.clone());
        }

        Self::new(posts)
    }
}

impl<'a> FromIterator<&'a Item> for List {
    fn from_iter<I: IntoIterator<Item=&'a Item>>(iter: I) -> Self {
        let mut posts = vec![];

        for item in iter {
            posts.push(item.post.clone());
        }

        Self::new(posts)
    }
}

impl FromIterator<Item> for List {
    fn from_iter<I: IntoIterator<Item=Item>>(iter: I) -> Self {
        let mut posts = vec![];

        for item in iter {
            posts.push(item.post);
        }

        Self::new(posts)
    }
}
