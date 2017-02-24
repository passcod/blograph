use colored::*;
use hyper::header::ContentLength;
use hyper::server;
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use regex::Regex;
use self::bytes::bytes;
use self::seconds::seconds;
use std::io::Write;
use super::*;
use time::PreciseTime;
use url::Url;

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

    pub fn route(&self, mut req: Request, mut res: Response) -> (StatusCode, usize) {
        lazy_static! {
            static ref URL: Url = Url::parse("http://localhost/").unwrap();
            static ref TAGGED: Regex = Regex::new(r"^/tag/(?P<tag>.+)$").unwrap();
            static ref DATED: Regex = Regex::new(r"(?ix)
                ^(?P<date>
                    /\d{4} # year
                    /(?:\d{1,2}|[a-z]{3}) # month
                    /\d{1,2} # day
                )(?P<path>/.+)$
            ").unwrap();
        }

        // Assume Hyper gives us a valid URI, and ignore otherwise
        let url = URL.join(&match req.uri {
            RequestUri::AbsolutePath(u) => u,
            _ => {
                *res.status_mut() = StatusCode::MethodNotAllowed;
                let mut res = res.start().unwrap();
                let written = res.write(&[]).unwrap();
                return (res.status(), written);
            }
        }).unwrap();

        // We only want the path, not the query or whatever may be appended
        let path = url.path();

        // Static routes
        let mut routed = match path {
            "/" => Some(self.index()),
            _ => None
        }.or_else(|| {
            if DATED.is_match(path) {
                Some(self.dated(&url))
            } else if TAGGED.is_match(path) {
                Some(self.tagged(&url))
            } else {
                None
            }
        }).unwrap_or(self.notfound());

        *res.status_mut() = routed.0;
        let mut res = res.start().unwrap();
        let written = res.write(&routed.1.into_bytes()).unwrap();

        (res.status(), written)
    }

    fn notfound(&self) -> (StatusCode, String) {
        (StatusCode::NotFound, "Not found".into())
    }

    fn index(&self) -> (StatusCode, String) {
        (StatusCode::Ok,
         self.posts
            .iter()
            .filter(|p|
                !p.post.is_future() &&
                !p.post.is_page() &&
                !p.post.metadata.bool("sideband").unwrap_or(false)
            )
            .collect::<List>()
            .sort_by_date()
            .iter()
            .fold("Front page:".into(), |acc, i|
                acc + "\n  - " + &i.post.slug()
            )
        )
    }

    fn dated(&self, url: &Url) -> (StatusCode, String) {
        (StatusCode::Ok, "".into())
    }

    fn tagged(&self, url: &Url) -> (StatusCode, String) {
        (StatusCode::Ok, "".into())
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
