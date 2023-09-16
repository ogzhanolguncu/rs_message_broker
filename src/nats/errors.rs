use std::{
    fmt::{self},
    io,
};

#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    CustomError(ErrMessages),
    PeerClosed,
    FailedToGetClientIP,
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::IoError(err)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(err) => write!(f, "IO error: {}", err),
            MyError::CustomError(err) => write!(f, "Custom error: {}", err),
            MyError::PeerClosed => write!(f, "Peer closed"),
            MyError::FailedToGetClientIP => write!(f, "Failed to get client IP address"),
        }
    }
}

#[derive(Debug, PartialEq)]

pub enum ErrMessages {
    NoHeadElement,
    UnknownProtocalOperation,
    UnknownCommand,
}

impl fmt::Display for ErrMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrMessages::NoHeadElement => write!(f, "No head element found"),
            ErrMessages::UnknownProtocalOperation => write!(f, "-ERR 'Unknown Protocol Operation'"),
            ErrMessages::UnknownCommand => write!(f, "-ERR 'Unknown Protocol Operation'"),
        }
    }
}

impl std::error::Error for ErrMessages {}
