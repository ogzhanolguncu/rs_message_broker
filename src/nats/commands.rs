use super::{
    errors::ErrMessages,
    parse_utils::{split_data_by_crlf, split_data_by_space},
};

const PARSE_U16_ERROR: &str = "Something went wrong when parsing sid to u16";
const PARSE_USIZE_ERROR: &str = "Something went wrong when parsing byte to usize";

#[derive(PartialEq, Debug)]
pub enum Command {
    Sub { subject: String, sid: u16 },
    Pub { subject: String, payload: String },
    Connect(String),
    Ping(String),
    Unsub(u16),
}

pub fn handle_pub(tail: &str) -> Result<Command, ErrMessages> {
    let (subject, rest) = split_data_by_space(tail)?;
    let (unparsed_bytes, payload) = split_data_by_crlf(rest.trim())?;
    let bytes = unparsed_bytes
        .trim_end_matches("\r\n")
        .parse()
        .map_err(|err| {
            println!("{} {:?}", PARSE_USIZE_ERROR, err);
            ErrMessages::UnknownProtocalOperation
        })?;
    if payload.len() != bytes {
        return Err(ErrMessages::UnknownProtocalOperation);
    }
    Ok(Command::Pub { subject, payload })
}

pub fn handle_sub(tail: &str) -> Result<Command, ErrMessages> {
    let (subject, unparsed_sid) = split_data_by_space(tail)?;
    match unparsed_sid.trim_end_matches("\r\n").parse::<u16>() {
        Ok(sid) => Ok(Command::Sub { subject, sid }),
        Err(err) => {
            println!("{} {:?}", PARSE_U16_ERROR, err);
            Err(ErrMessages::UnknownProtocalOperation)
        }
    }
}

pub fn handle_unsub(tail: &str) -> Result<Command, ErrMessages> {
    match tail.trim_end_matches("\r\n").parse::<u16>() {
        Ok(sid) => Ok(Command::Unsub(sid)),
        Err(err) => {
            println!("{} {:?}", PARSE_U16_ERROR, err);
            Err(ErrMessages::UnknownProtocalOperation)
        }
    }
}

pub fn handle_ping() -> Command {
    Command::Ping("PONG\r\n".to_string())
}

pub fn handle_connect() -> Command {
    Command::Connect("+OK\r\n".to_string())
}
