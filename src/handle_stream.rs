use log::{error, info, warn};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    nats::{commands::Command, errors::MyError, parser::parse_nats},
    ADDR, PORT,
};

pub fn handle_stream(mut stream: TcpStream) {
    initialize_stream(&mut stream);
    event_loop(&mut stream);
}

fn initialize_stream(stream: &mut TcpStream) {
    if let Err(err) = respond_with_info(stream, ADDR.to_string().as_str(), PORT) {
        error!("Failed to get client IP address: {:?}", err);
    }
}

fn event_loop(stream: &mut TcpStream) {
    let mut buffer = [0_u8; 128];
    loop {
        match handle_event(stream, &mut buffer) {
            Ok(_) => continue,
            Err(err) => {
                handle_error(err, stream);
                break;
            }
        }
    }
}

fn handle_event(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), MyError> {
    let human_readable = read_buffer(stream, buffer)?.ok_or(MyError::PeerClosed)?;
    let command = parse_nats(&human_readable.to_uppercase()).map_err(MyError::CustomError)?;
    handle_command(stream, command)
}

fn handle_error(err: MyError, stream: &mut TcpStream) {
    match err {
        MyError::IoError(io_err) => {
            error!("IO error: {}", io_err);
        }
        MyError::CustomError(custom_err) => {
            let crlf_added_custom_err = custom_err.to_string() + "\r\n";
            if let Err(e) = write_back_to_client(stream, crlf_added_custom_err) {
                error!("Failed to write back to client: {}", e);
            }
            error!("Custom error: {}", custom_err);
        }
        MyError::PeerClosed => {
            info!("Peer closed!");
        }
        MyError::FailedToGetClientIP => {
            warn!("Failed to get client IP address!");
        }
    }
}

fn read_buffer(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<Option<String>, MyError> {
    let size = stream.read(buffer)?;
    if size == 0 {
        Ok(None)
    } else {
        Ok(Some(String::from_utf8_lossy(&buffer[..size]).into_owned()))
    }
}

fn write_back_to_client(stream: &mut TcpStream, message: String) -> Result<(), MyError> {
    stream.write_all(message.as_bytes()).map_err(|e| {
        let peer_info = stream
            .peer_addr()
            .map_or_else(|_| "Unknown peer".to_string(), |addr| addr.to_string());

        error!("An error occurred while writing to {}: {}", peer_info, e);
        MyError::IoError(e)
    })
}

fn handle_command(stream: &mut TcpStream, command: Command) -> Result<(), MyError> {
    match command {
        Command::Sub { .. } => Ok(()),
        Command::Pub { .. } => Ok(()),
        Command::Connect(message) | Command::Ping(message) => write_back_to_client(stream, message),
    }
}

fn respond_with_info(stream: &mut TcpStream, addr: &str, port: u16) -> Result<(), MyError> {
    let client_ip = stream
        .peer_addr()
        .map_err(|_| MyError::FailedToGetClientIP)?;
    let json_string = format!(
        "INFO {{\"host\":\"{}\",\"port\":{},\"client_ip\":\"{}\"}}\r\n",
        addr, port, client_ip
    );
    write_back_to_client(stream, json_string)
}
