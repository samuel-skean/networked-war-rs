mod format;
mod server;

use std::{net::IpAddr, process::ExitCode};

use clap::Parser;
use server::*;
use tokio::net::TcpListener;

#[derive(clap::Parser)]
struct Args {
    host: IpAddr,
    /// Can be set to 0 to request the OS to pick a port.
    port: u16,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    // STRETCH: what would it mean to let the user bind to a string (e.g., a DNS
    // name)? Should I support that?
    let listener = TcpListener::bind((args.host, args.port)).await.unwrap();
    println!("Listening on {addr}", addr = listener.local_addr().unwrap());
    // TODO: When might accept fail? Also, consider `TcpListenerStream`.
    while let Ok(player_one) = listener.accept().await {
        println!("Got client {0:?}", player_one.1);
        let Ok(player_two) = listener.accept().await else {
            break;
        };
        tokio::spawn(serve_game(Game {
            player_one,
            player_two,
        }));
    }
    eprintln!("How did I get here? `accept` failed, I think!");
    return ExitCode::from(1);
}
