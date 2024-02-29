// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::{error, fmt};

#[derive(Debug)]
pub enum Failure {
    IOError(std::io::Error),
    Why(&'static str),
}

impl From<std::io::Error> for Failure {
    fn from(error: std::io::Error) -> Self {
        Failure::IOError(error)
    }
}

impl From<&'static str> for Failure {
    fn from(str: &'static str) -> Self {
        Self::Why(str)
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "IOError: {err}"),
            Self::Why(reason) => write!(f, "Error: {reason}"),
        }
    }
}

impl error::Error for Failure {}
