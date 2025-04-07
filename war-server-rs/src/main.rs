mod format;

use std::net::IpAddr;

use clap::Parser;
use tokio::net::TcpListener;

#[derive(clap::Parser)]
struct Args {
    host: IpAddr,
    /// Can be set to 0 to request the OS to pick a port.
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // STRETCH: what would it mean to let the user bind to a string (e.g., a DNS
    // name)? Should I support that?
    let listener = TcpListener::bind((args.host, args.port)).await.unwrap();
    println!("Listening on {addr}", addr = listener.local_addr().unwrap());
    // TODO: When might accept fail? Also, consider
    while let Ok((conn, peer_addr)) = listener.accept().await {}
}
