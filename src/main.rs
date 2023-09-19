#[macro_use]
extern crate log;

mod handle_stream;
mod nats;
mod store;

use handle_stream::handle_stream;
use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    sync::Arc,
    thread,
};
use store::message_broker::MessageBrokerStore;

const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 4222; //NATS PORT

fn main() {
    std::env::set_var("RUST_LOG", "info");
    info!("Starting up server!");

    let store = Arc::new(MessageBrokerStore::new());

    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap_or_else(|err| {
        println!("Failed to bind address: {}", err);
        std::process::exit(1);
    });
    for client in listener.incoming() {
        match client {
            Ok(stream) => {
                let store_clone = store.clone();
                thread::spawn(move || handle_stream(stream, &store_clone));
            }
            Err(err) => println!("Connection failed due to {:?}", err),
        }
    }
}
