use super::{
    errors::ErrMessages,
    parse_utils::{split_data_by_crlf, split_data_by_space},
};

const PARSE_U16_ERROR: &str = "Something went wrong when parsing sid to u16";
const PARSE_USIZE_ERROR: &str = "Something went wrong when parsing byte to usize";

#[derive(PartialEq, Debug)]
pub enum Command {
    SUB {
        subject: String,
        sid: u16,
    },
    PUB {
        subject: String,
        bytes: u8,
        payload: String,
    },
    CONNECT(String),
    PING(String),
}

pub fn handle_pub(tail: &String) -> Result<Command, ErrMessages> {
    let (subject, rest) = split_data_by_space(&tail)?;
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
    bytes
        .try_into()
        .map(|bytes_u8| Command::PUB {
            subject,
            bytes: bytes_u8,
            payload,
        })
        .map_err(|_| ErrMessages::UnknownProtocalOperation)
}

pub fn handle_sub(tail: &String) -> Result<Command, ErrMessages> {
    let (subject, unparsed_sid) = split_data_by_space(tail)?;
    match unparsed_sid.trim_end_matches("\r\n").parse::<u16>() {
        Ok(sid) => Ok(Command::SUB { subject, sid }),
        Err(err) => {
            println!("{} {:?}", PARSE_U16_ERROR, err);
            Err(ErrMessages::UnknownProtocalOperation)
        }
    }
}

pub fn handle_ping() -> Command {
    Command::PING("PONG\r\n".to_string())
}

pub fn handle_connect() -> Command {
    Command::CONNECT("+OK\r\n".to_string())
}
