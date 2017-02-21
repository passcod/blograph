use futures::future::Future;
use super::post::Post;
use super::realised::RealisedPost;

pub struct Bodies {
    pub plain: Box<[u8]>,
    pub zopfli: Box<[u8]>,
    pub brotli: Box<[u8]>,
}

pub struct Render {
    pub post: RealisedPost,
    pub was_future: bool,
    pub bodies: Future<Item=Bodies, Error=()>,
}

impl Render {
    //
}
