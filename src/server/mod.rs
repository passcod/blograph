use hyper::server::Http;
use list::List;
use self::router::Router;
pub use self::bytes::bytes;
pub use self::seconds::seconds;
use std::process;
use std::sync::Arc;

mod bytes;
mod router;
mod seconds;

pub fn start(addr: &str, all: List) {
    trace!("Preparing server");
    let bind = addr.parse();
    if let Err(err) = bind {
        error!("Bad address definition {}", addr);
        error!("{}", err);
        process::exit(1);
    }

    let bind = bind.unwrap();
    debug!("Bind: {}", bind);

    let all = Arc::new(all);
    let server = Http::new().bind(&bind, move || Ok(Router::new(all.clone())));
    if let Err(err) = server {
        error!("Could't bind server to {}", bind);
        error!("{}", err);
        process::exit(1);
    }

    trace!("Unwrapping server");
    let server = server.unwrap();

    info!("Listening on http://{}", server.local_addr().unwrap());
    if let Err(err) = server.run() {
        error!("Failed to start server!");
        error!("{}", err);
        process::exit(1);
    } else {
        info!("Server shutdown");
    }
}
