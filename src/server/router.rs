use colored::*;
use futures::future::{self, Shared, Future, FutureResult};
use hyper::{self, Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Service, Request, Response};
use list::List;
use std::sync::Arc;
use super::{bytes, seconds};
use time::PreciseTime;

#[derive(Clone, Debug)]
pub struct Router {
    posts: Arc<List>
}

impl Router {
    pub fn new(posts: Arc<List>) -> Self {
        Router { posts: posts }
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Shared<BoxFuture<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        info!("→ {} {}", format!("{}", req.method()).magenta(), req.path());
        future::ok(PreciseTime::now())
        .map(|now| match (req.method(), req.path()) {
            (&Get, "/") | (&Get, "/echo") => {
                (now, Response::new().with_body("echo"))
            },
            _ => {
                (now, Response::new().with_status(StatusCode::NotFound))
            }
        }).map(|(now, res)| {
            let elapsed = now.to(PreciseTime::now());
            info!("← {} ({}) ({})",
                format!("{}", /*status*/ "000 ???").magenta(),
                bytes(/*written*/ 0),
                seconds(elapsed)
            );

            res
        }).boxed().shared()
    }
}
