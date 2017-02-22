use colored::*;
use hyper::header::ContentLength;
use hyper::server;
use hyper::server::request::Request;
use hyper::server::response::Response;
use self::bytes::bytes;
use self::seconds::seconds;
use super::*;
use time::Duration;

mod bytes;
mod seconds;

pub struct Handler {
    posts: List,
}

impl Handler {
    pub fn new(posts: List) -> Self {
        debug!("Preparing router");
        Handler { posts: posts }
    }

    pub fn route(&self, req: &mut Request, res: &mut Response) {

    }
}

impl server::Handler for Handler {
    fn handle(&self, mut req: Request, mut res: Response) {
        info!("→ {} {}", format!("{}", req.method).magenta(), req.uri);
        let elapsed = Duration::span(|| self.route(&mut req, &mut res));
        info!("← {} ({}) ({})",
            format!("{}", res.status()).magenta(),
            bytes(res.headers()
                .get::<ContentLength>()
                .unwrap_or(&ContentLength(0))
                .0 as usize
            ),
            seconds(elapsed)
        );
    }
}
