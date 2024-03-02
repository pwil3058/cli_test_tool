// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::num::ParseIntError;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    MalformedErrorCode(ParseIntError),
    Why(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::MalformedErrorCode(error)
    }
}

impl From<&'static str> for Error {
    fn from(str: &'static str) -> Self {
        Self::Why(str)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "IOError: {err}"),
            Self::MalformedErrorCode(err) => write!(f, "Malformed Error Code: {err}"),
            Self::Why(reason) => write!(f, "Error: {reason}"),
        }
    }
}

impl error::Error for Error {}
