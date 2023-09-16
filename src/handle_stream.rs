use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::{
    nats::{commands::Command, parser::parse_nats},
    ADDR, PORT,
};

pub fn handle_stream(mut stream: TcpStream) {
    let mut data = [0_u8; 128];
    loop {
        respond_with_info(&mut stream);
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    match stream.peer_addr() {
                        Ok(addr) => println!("Connection closed by {}", addr),
                        Err(_) => println!("Connection closed but could not get peer address."),
                    }
                    break;
                } else {
                    let human_readable = String::from_utf8_lossy(&data);
                    parse_nats(&human_readable)
                        .map(|res| match res {
                            Command::SUB { .. } => todo!(),
                            Command::PUB { .. } => todo!(),
                            Command::CONNECT(res) => write_back_to_client(&mut stream, res),
                            Command::PING(res) => write_back_to_client(&mut stream, res),
                        })
                        .map_err(|err| write_back_to_client(&mut stream, err.to_string()))
                        .ok(); // Consumes the Result, does nothing regardless of its value
                }
            }
            Err(err) => {
                match stream.peer_addr() {
                    Ok(addr) => println!(
                        "An error occurred, terminating connection with {}: {}",
                        addr, err
                    ),
                    Err(_) => println!("An error occurred and could not get peer address: {}", err),
                }
                if let Err(err) = stream.shutdown(Shutdown::Both) {
                    println!("An error occurred while shutting down the stream: {}", err);
                }
                break;
            }
        }
    }
}

fn write_back_to_client(stream: &mut TcpStream, message: String) {
    match stream.write_all(message.as_bytes()) {
        Ok(_) => {}
        Err(err) => match stream.peer_addr() {
            Ok(addr) => println!("An error occurred while writing to {}: {}", addr, err),
            Err(_) => println!(
                "An error occurred while writing and could not get peer address: {}",
                err
            ),
        },
    }
}

fn respond_with_info(stream: &mut TcpStream) {
    match stream.peer_addr() {
        Ok(client_ip) => {
            let json_string = format!(
                "INFO {{\"host\":\"{}\",\"port\":{},\"client_ip\":\"{}\"}}",
                ADDR, PORT, client_ip
            );
            if let Err(err) = stream.write_all(json_string.as_bytes()) {
                match stream.peer_addr() {
                    Ok(addr) => println!("An error occurred while writing to {}: {}", addr, err),
                    Err(_) => println!(
                        "An error occurred while writing and could not get peer address: {}",
                        err
                    ),
                }
            }
        }
        Err(_) => println!("Couldn't access clients ip!"),
    };
}
