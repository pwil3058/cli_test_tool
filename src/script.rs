// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use crate::command;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

use crate::command::{Command, Outcome};
use crate::failure::Failure;

#[derive(Debug)]
struct CommandAndExpectedOutcome {
    command: Command,
    expected_outcome: Outcome,
    range: Range<usize>,
}

#[derive(Debug, Default)]
pub struct Script {
    _script: String,
    commands: Vec<CommandAndExpectedOutcome>,
}

#[derive(Debug)]
pub enum PassOrFail {
    Pass,
    Fail(String, Outcome, Outcome),
}

fn read_script<R: Read>(mut reader: R) -> Result<String, Failure> {
    let mut script = String::new();
    match reader.read_to_string(&mut script) {
        Ok(size) => {
            log::trace!("Read {} bytes", size);
            Ok(script)
        }
        Err(err) => {
            log::error!("Error reading script file: {}", err);
            Err(Failure::IOError(err))
        }
    }
}

impl Script {
    pub fn read<R: Read>(reader: R) -> Result<Self, Failure> {
        let script = read_script(reader)?;
        let lines: Vec<&str> = script.split_inclusive('\n').collect();
        let mut commands = Vec::new();
        let mut i = 0;
        while let Some(line) = lines.get(i) {
            if let Some(stripped) = line.strip_prefix('$') {
                let command = command::Command::new(stripped)?;
                let mut expected_outcome = command::Outcome::default();
                println!("{:?}", command);
                let start = i;
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
                                    log::error!(
                                        "Line: {i}: {err} badly formed error code: {trimmed}"
                                    );
                                    return Err(Failure::from("Badly formed error code"));
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
                let range = Range { start, end: i };
                commands.push(CommandAndExpectedOutcome {
                    command,
                    expected_outcome,
                    range,
                })
            } else {
                i += 1
            }
        }
        Ok(Self {
            _script: script,
            commands,
        })
    }

    pub fn read_from(path: &Path) -> Result<Self, Failure> {
        match File::open(path) {
            Ok(file) => Self::read(file),
            Err(err) => {
                log::error!("Error opening script file: {:?}: {}. Aborting.", path, err);
                Err(Failure::IOError(err))
            }
        }
    }

    pub fn run(&self) -> Result<PassOrFail, Failure> {
        for caeo in self.commands.iter() {
            println!("Run: {}", caeo.command.cmd_line_string);
            println!("Lines: {:?}", caeo.range);
            let outcome = caeo.command.run()?;
            println!("Outcome: {outcome:?}");
            if outcome != caeo.expected_outcome {
                return Ok(PassOrFail::Fail(
                    caeo.command.cmd_line_string.clone(),
                    caeo.expected_outcome.clone(),
                    outcome,
                ));
            }
        }
        Ok(PassOrFail::Pass)
    }
}
