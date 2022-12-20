// Copyright 2022 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::env;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Outcome {
    e_code: i32,
    std_out: Vec<u8>,
    std_err: Vec<u8>,
}

#[derive(Debug)]
pub enum Failure {
    IOError(std::io::Error),
    Why(&'static str),
}

#[derive(Debug)]
pub struct Command {
    cmd_line_string: String,
    cmd_line: Vec<String>,
    input_text: Option<String>,
    redirection_path: Option<String>,
}

impl Command {
    pub fn new(cmd_line_string: String) -> Result<Self, String> {
        match shlex::split(&cmd_line_string) {
            Some(mut cmd_line) => {
                if cmd_line.is_empty() {
                    Err("Empty command line.".to_string())
                } else {
                    let redirection_path = match cmd_line.iter().position(|x| *x == ">") {
                        Some(red_index) => match cmd_line.len() - red_index {
                            1 => return Err("expected redirection path".to_string()),
                            2 => {
                                let path = cmd_line.pop().unwrap();
                                cmd_line.truncate(red_index);
                                Some(path)
                            }
                            _ => return Err("unexpected arguments".to_string()),
                        },
                        None => None,
                    };
                    Ok(Command {
                        cmd_line_string,
                        cmd_line,
                        input_text: None,
                        redirection_path,
                    })
                }
            }
            None => Err("Poorly formed command line".to_string()),
        }
    }

    pub fn run(&self) -> Result<Outcome, Failure> {
        match self.cmd_line[0].as_str() {
            "umask" => Err(Failure::Why("\"umask\" is not available")),
            "cd" => match self.cmd_line.len() {
                2 => match env::set_current_dir(&self.cmd_line[1]) {
                    Ok(_) => {
                        env::set_var("PWD", env::current_dir().unwrap());
                        Ok(Outcome::default())
                    }
                    Err(err) => Err(Failure::IOError(err)),
                },
                _ => Err(Failure::Why("expected exactly one argument")),
            },
            "export" => {
                for cmd in &self.cmd_line[1..] {
                    let pair: Vec<&str> = cmd.as_str().split("=").collect();
                    if pair.len() == 2 {
                        env::set_var(pair[0], pair[1]);
                    } else {
                        return Err(Failure::Why("expected \"ARG=VALUE\""));
                    }
                }
                Ok(Outcome::default())
            }
            "unset" => {
                for var in &self.cmd_line[1..] {
                    env::remove_var(var);
                }
                Ok(Outcome::default())
            }
            _ => Err(Failure::Why("placeholder")),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;

    #[test]
    fn new_command() {
        println!("{:?}", Command::new("ls > aaa".to_string()))
    }
}
