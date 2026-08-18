// Copyright 2024 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

use crate::command::{Command, Outcome};
use crate::error::Error;

#[derive(Debug)]
pub struct EnvVars(pub HashMap<String, String>);

impl EnvVars {
    pub fn new() -> Self {
        let env_vars: HashMap<String, String> = std::env::vars()
            .filter(|(k, _)| k == "TZ" || k == "LANG" || k == "PATH" || k == "PWD" || k == "HOME")
            .map(|(ref k, ref v)| (k.into(), v.into()))
            .collect();
        EnvVars(env_vars)
    }

    #[allow(dead_code)]
    pub fn var(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn set_var(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    pub fn remove_var(&mut self, key: &str) {
        self.0.remove(key);
    }
}

#[derive(Debug)]
struct CommandAndExpectedOutcome {
    command: Command,
    expected_outcome: Outcome,
    range: Range<usize>,
}

impl CommandAndExpectedOutcome {
    pub fn evaluate(&self, env_vars: &mut EnvVars) -> Result<Evaluation, Error> {
        let outcome = self.command.run(env_vars)?;
        if outcome == self.expected_outcome {
            Ok(Evaluation::Pass)
        } else {
            Ok(Evaluation::Fail(
                self.range.clone(),
                self.command.cmd_line_string.clone(),
                self.expected_outcome.clone(),
                outcome,
            ))
        }
    }
}

#[derive(Debug, Default)]
pub struct Script {
    commands: Vec<CommandAndExpectedOutcome>,
}

#[derive(Debug)]
pub enum Evaluation {
    Pass,
    Fail(Range<usize>, String, Outcome, Outcome),
}

impl Evaluation {
    pub fn failed(&self) -> bool {
        match self {
            Self::Pass => false,
            Self::Fail(_, _, _, _) => true,
        }
    }
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "PASS"),
            Self::Fail(range, cl_string, expected, actual) => {
                write!(f, "FAIL: {range:?}: {cl_string}")?;
                if expected.e_code != actual.e_code {
                    if let Some(e_e_code) = expected.e_code {
                        if let Some(a_e_code) = actual.e_code {
                            write!(f, "\tExpected Error Code {e_e_code} got {a_e_code}")?;
                        } else {
                            write!(f, "\tExpected Error Code {e_e_code} got \"killed\"")?;
                        }
                    } else {
                        let a_e_code = actual.e_code.expect("Should NOT be None");
                        write!(f, "\tExpected Error Code \"killed\" got {a_e_code}")?;
                    }
                }
                if expected.std_out != actual.std_out {
                    write!(
                        f,
                        "\tExpected Stdout: {}\t  Actual Stdout: {}",
                        expected.std_out, actual.std_out
                    )?;
                }
                if expected.std_err != actual.std_err {
                    write!(
                        f,
                        "\tExpected Stderr: {}\t  Actual Stderr: {}",
                        expected.std_err, actual.std_err
                    )?;
                }
                Ok(())
            }
        }
    }
}

fn read_script<R: Read>(mut reader: R) -> Result<String, Error> {
    let mut script = String::new();
    reader.read_to_string(&mut script)?;
    Ok(script)
}

impl Script {
    pub fn read<R: Read>(reader: R) -> Result<Self, Error> {
        let script = read_script(reader)?;
        let lines: Vec<&str> = script.split_inclusive('\n').collect();
        let mut commands = Vec::new();
        let mut i = 0;
        while let Some(line) = lines.get(i) {
            if let Some(stripped) = line.strip_prefix('$') {
                let command = Command::new(stripped)?;
                let mut expected_outcome = Outcome::default();
                // line numbers start at 1
                let start = i + 1;
                i += 1;
                while let Some(line) = lines.get(i) {
                    if line.starts_with('$') {
                        break;
                    } else if let Some(stripped) = line.strip_prefix('?') {
                        let trimmed = stripped.trim();
                        if trimmed.is_empty() {
                            expected_outcome.e_code = None;
                        } else {
                            expected_outcome.e_code = Some(i32::from_str(trimmed)?);
                        }
                    } else if let Some(trimmed) = line.strip_prefix('!') {
                        expected_outcome.std_err.push_str(trimmed.trim_start());
                    } else if let Some(trimmed) = line.strip_prefix('>') {
                        expected_outcome.std_out.push_str(trimmed.trim_start());
                    }
                    i += 1;
                }
                // line numbers start at 1
                let range = Range { start, end: i + 1 };
                commands.push(CommandAndExpectedOutcome {
                    command,
                    expected_outcome,
                    range,
                })
            } else {
                i += 1
            }
        }
        Ok(Self { commands })
    }

    pub fn read_from(path: &Path) -> Result<Self, Error> {
        Self::read(File::open(path)?)
    }

    pub fn evaluate(&self) -> Result<Evaluation, Error> {
        let mut env_vars = EnvVars::new();
        for command in self.commands.iter() {
            let evaluation = command.evaluate(&mut env_vars)?;
            if evaluation.failed() {
                return Ok(evaluation);
            }
        }
        Ok(Evaluation::Pass)
    }
}
