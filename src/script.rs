// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use crate::command::{Command, Outcome};

#[derive(Debug)]
enum Error {
    IOError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An Error has occurred")
    }
}

impl error::Error for Error {}

#[derive(Debug)]
struct CommandAndExpectedOutcome {
    command: Command,
    expected: Outcome,
}

#[derive(Debug, Default)]
pub struct Script {
    script: String,
    commands: Vec<CommandAndExpectedOutcome>,
}

fn read_script(path: &Path) -> Result<String, Error> {
    let mut script = String::new();
    match File::open(path) {
        Ok(mut file) => match file.read_to_string(&mut script) {
            Ok(size) => {
                log::trace!("Read {} bytes", size);
                Ok(script)
            }
            Err(err) => {
                log::error!("Error reading script file: {}", err);
                Err(Error::IOError(err))
            }
        },
        Err(err) => {
            log::error!("Error opening script file: {:?}: {}. Aborting.", path, err);
            Err(Error::IOError(err))
        }
    }
}

impl Script {
    pub fn read_from(path: &Path) -> Result<Self, Error> {
        let script = read_script(path)?;
        Ok(Self::default())
    }
}
