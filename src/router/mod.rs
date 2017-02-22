use hyper::server;
use hyper::server::request::Request;
use hyper::server::response::Response;
use super::*;

pub struct Handler {
    posts: List,
}

impl Handler {
    pub fn new(posts: List) -> Self {
        debug!("Preparing router");
        Handler { posts: posts }
    }
}

impl server::Handler for Handler {
    fn handle(&self, req: Request, res: Response) {
        //
    }
}
