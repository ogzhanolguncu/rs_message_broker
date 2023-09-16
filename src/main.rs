mod nats;

use crate::nats::parser::parse_nats;

// const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
// const PORT: u16 = 4222; //NATS PORT

fn main() {
    // let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap_or_else(|err| {
    //     println!("Failed to bind address: {}", err);
    //     std::process::exit(1);
    // });
    parse_nats("").unwrap();
    todo!()
}

//If anything goes wrong do return this and terminate std::exit(1)
//Connection closed by foreign host.
