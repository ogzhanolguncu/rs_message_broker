use std::fmt::{self};

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
