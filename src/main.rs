#[macro_use]
extern crate log;

mod handle_stream;
mod nats;

use handle_stream::handle_stream;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 4222; //NATS PORT

fn main() {
    std::env::set_var("RUST_LOG", "info"); // Optional, if you haven't set it in the shell
    env_logger::init();
    info!("Starting up server!");

    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap_or_else(|err| {
        println!("Failed to bind address: {}", err);
        std::process::exit(1);
    });
    for client in listener.incoming() {
        match client {
            Ok(stream) => handle_stream(stream),
            Err(err) => println!("Connection failed due to {:?}", err),
        }
    }
    todo!()
}
