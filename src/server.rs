use hyper::server::Server;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::process;

pub fn init(port: u16) -> Server {
    trace!("Preparing server");

    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let bind = SocketAddrV4::new(ip, port);
    debug!("IP: {}", ip);
    debug!("Socket: {}", bind);

    let server = Server::http(bind);
    if let Err(err) = server {
        error!("Could't bind server to {}", bind);
        error!("{}", err);
        process::exit(1);
    }

    trace!("Prepared server");
    server.unwrap()
}
