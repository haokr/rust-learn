use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    println!("addr: {:?}, port: {:?}", addr.ip(), addr.port());
}