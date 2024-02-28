// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use crate::command;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use crate::command::{Command, Outcome};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    Simple(String),
}

impl From<&str> for Error {
    fn from(str: &str) -> Self {
        Self::Simple(str.to_string())
    }
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
    expected_outcome: Outcome,
}

#[derive(Debug, Default)]
pub struct Script {
    script: String,
    commands: Vec<CommandAndExpectedOutcome>,
}

fn read_script<R: Read>(mut reader: R) -> Result<String, Error> {
    let mut script = String::new();
    match reader.read_to_string(&mut script) {
        Ok(size) => {
            log::trace!("Read {} bytes", size);
            Ok(script)
        }
        Err(err) => {
            log::error!("Error reading script file: {}", err);
            Err(Error::IOError(err))
        }
    }
}

fn read_script_from(path: &Path) -> Result<String, Error> {
    match File::open(path) {
        Ok(mut file) => read_script(file),
        Err(err) => {
            log::error!("Error opening script file: {:?}: {}. Aborting.", path, err);
            Err(Error::IOError(err))
        }
    }
}

impl Script {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        let script = read_script(reader)?;
        let lines: Vec<&str> = script.lines().collect();
        let mut commands = Vec::new();
        let mut i = 0;
        while let Some(line) = lines.get(i) {
            if let Some(stripped) = line.strip_prefix('$') {
                let command = command::Command::new(stripped)?;
                let mut expected_outcome = command::Outcome::default();
                println!("{:?}", command);
                i += 1;
                while let Some(line) = lines.get(i) {
                    if line.starts_with('$') {
                        break;
                    } else if let Some(stripped) = line.strip_prefix('?') {
                        let trimmed = stripped.trim();
                        if trimmed.is_empty() {
                            expected_outcome.e_code = None;
                        } else {
                            match i32::from_str(trimmed) {
                                Ok(e_code) => expected_outcome.e_code = Some(e_code),
                                Err(err) => {
                                    log::error!("Line: {i}: badly formed error code: {trimmed}");
                                    return Err(Error::from("Badly formed error code"));
                                }
                            }
                        }
                        println!("expected e_code: {:?}", expected_outcome.e_code);
                    } else if let Some(trimmed) = line.strip_prefix('!') {
                        expected_outcome.std_err.push_str(trimmed.trim_start());
                    } else if let Some(trimmed) = line.strip_prefix('>') {
                        expected_outcome.std_out.push_str(trimmed.trim_start());
                    }
                    i += 1;
                }
                commands.push(CommandAndExpectedOutcome {
                    command,
                    expected_outcome,
                })
            } else {
                i += 1
            }
        }
        Ok(Self { script, commands })
    }

    pub fn read_from(path: &Path) -> Result<Self, Error> {
        match File::open(path) {
            Ok(mut file) => Self::read(file),
            Err(err) => {
                log::error!("Error opening script file: {:?}: {}. Aborting.", path, err);
                Err(Error::IOError(err))
            }
        }
    }
}
