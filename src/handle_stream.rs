use log::{info, warn};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use crate::{
    nats::{
        commands::Command,
        errors::{ErrMessages, MyError},
        parser::parse_nats,
    },
    store::message_broker::{MessageBrokerStore, Subject, SubscriptionId},
    ADDR, PORT,
};

#[derive(Debug)]
enum ReadState {
    Command,
    Payload {
        subject: String,
        expected_len: usize,
    },
}

pub fn handle_stream(mut stream: TcpStream, store: &Arc<MessageBrokerStore>) {
    initialize_stream(&mut stream);
    event_loop(&mut stream, store);
}

fn initialize_stream(stream: &mut TcpStream) {
    if let Err(err) = respond_with_info(stream, ADDR.to_string().as_str(), PORT) {
        error!("Failed to get client IP address: {:?}", err);
    }
}

fn event_loop(stream: &mut TcpStream, store: &Arc<MessageBrokerStore>) {
    let mut buffer = Vec::new();
    let mut data = [0_u8; 128];
    let mut state = ReadState::Command;

    loop {
        match read_from_stream(stream, &mut data) {
            Ok(size) => {
                buffer.extend_from_slice(&data[0..size]);
                state = match state {
                    ReadState::Command => handle_command_state(stream, store, &mut buffer),
                    ReadState::Payload {
                        subject,
                        expected_len,
                    } => handle_payload_state(stream, store, &mut buffer, subject, expected_len),
                };
            }
            Err(e) => {
                println!("An error occurred while reading from the stream: {}", e);
                break;
            }
        }
    }
}

fn read_from_stream(stream: &mut TcpStream, data: &mut [u8]) -> Result<usize, MyError> {
    match stream.read(data) {
        Ok(0) => {
            info!("Connection closed by client!");
            Err(MyError::PeerClosed)
        }
        Ok(size) => Ok(size),
        Err(_) => Err(MyError::CustomError(ErrMessages::InternalError)),
    }
}

fn handle_command_state(
    stream: &mut TcpStream,
    store: &Arc<MessageBrokerStore>,
    buffer: &mut Vec<u8>,
) -> ReadState {
    if let Some(end_of_command) = buffer.iter().position(|&b| b == b'\n') {
        let command_bytes = buffer.drain(..=end_of_command).collect::<Vec<u8>>();
        let command_str = String::from_utf8_lossy(&command_bytes);

        if command_str.starts_with("PUB") {
            let parts: Vec<&str> = command_str.split_whitespace().collect();
            if parts.len() >= 3 {
                let subject = parts[1].to_string();
                let expected_len: usize = parts[2].parse().unwrap_or(0);
                return ReadState::Payload {
                    subject,
                    expected_len,
                };
            }
        }
        if let Err(err) = handle_event(stream, store, &command_str) {
            handle_error(err, stream);
        }
    }
    ReadState::Command
}

fn handle_payload_state(
    stream: &mut TcpStream,
    store: &Arc<MessageBrokerStore>,
    buffer: &mut Vec<u8>,
    subject: String,
    expected_len: usize,
) -> ReadState {
    if buffer.len() >= expected_len {
        let payload = buffer.drain(..expected_len).collect::<Vec<u8>>();
        let payload_str = String::from_utf8_lossy(&payload);

        let pub_payload = format!("PUB {} {}\r\n{}\r\n", subject, expected_len, payload_str);
        if let Err(err) = handle_event(stream, store, &pub_payload) {
            handle_error(err, stream);
        }
    }
    ReadState::Command
}
fn handle_event(
    stream: &mut TcpStream,
    store: &Arc<MessageBrokerStore>,
    human_readable: &str,
) -> Result<(), MyError> {
    let command = parse_nats(&human_readable.to_uppercase()).map_err(MyError::CustomError)?;
    handle_command(stream, command, store)
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

fn write_back_to_client(stream: &mut TcpStream, message: String) -> Result<(), MyError> {
    stream.write_all(message.as_bytes()).map_err(|e| {
        let peer_info = stream
            .peer_addr()
            .map_or_else(|_| "Unknown peer".to_string(), |addr| addr.to_string());

        error!("An error occurred while writing to {}: {}", peer_info, e);
        MyError::IoError(e)
    })
}

fn handle_command(
    stream: &mut TcpStream,
    command: Command,
    store: &Arc<MessageBrokerStore>,
) -> Result<(), MyError> {
    match command {
        Command::Sub { sid, subject } => {
            let stream_clone = stream.try_clone().unwrap();
            match store.add_subscription(Subject(subject), SubscriptionId(sid), stream_clone) {
                Ok(_) => write_back_to_client(stream, "+OK\r\n".to_string()),
                Err(_) => Err(MyError::CustomError(ErrMessages::InternalError)),
            }
        }
        Command::Unsub(sid) => match store.remove_subscription(SubscriptionId(sid)) {
            Ok(_) => write_back_to_client(stream, "+OK\r\n".to_string()),
            Err(_) => Err(MyError::CustomError(ErrMessages::InternalError)),
        },
        Command::Pub { payload, subject } => {
            write_back_to_client(stream, "+OK\r\n".to_string())?;
            store.publish_to_sub(Subject(subject), payload);
            Ok(())
        }
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
