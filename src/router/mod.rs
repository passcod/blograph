use colored::*;
use hyper::header::ContentLength;
use hyper::server;
use hyper::server::request::Request;
use hyper::server::response::Response;
use hyper::status::StatusCode;
use self::bytes::bytes;
use self::seconds::seconds;
use std::io::Write;
use super::*;
use time::PreciseTime;

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

    pub fn route(&self, req: Request, res: Response) -> (StatusCode, usize) {
        let frontpage = self.posts.iter().filter(|p|
            !p.post.is_future() &&
            !p.post.is_page() &&
            !p.post.metadata.bool("sideband").unwrap_or(false)
        ).collect::<List>().sort_by_date();

        let slugs = frontpage
            .iter()
            .map(|i| i.post.slug())
            .collect::<Vec<String>>();

        let mut res = res.start().unwrap();
        let mut written = 0;

        for slug in slugs {
            written += res.write(&format!("- {}\n", slug).into_bytes()).unwrap();
        }

        (res.status(), written)
    }
}

impl server::Handler for Handler {
    fn handle(&self, req: Request, res: Response) {
        info!("→ {} {}", format!("{}", req.method).magenta(), req.uri);
        let now = PreciseTime::now();
        let (status, written) = self.route(req, res);
        let elapsed = now.to(PreciseTime::now());
        info!("← {} ({}) ({})",
            format!("{}", status).magenta(),
            bytes(written),
            seconds(elapsed)
        );
    }
}
