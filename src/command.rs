// Copyright 2022 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::convert::From;
use std::env;

#[derive(Debug, Default)]
pub struct Outcome {
    pub e_code: i32,
    pub std_out: Vec<u8>,
    pub std_err: Vec<u8>,
}

impl From<std::process::Output> for Outcome {
    fn from(output: std::process::Output) -> Self {
        Outcome {
            e_code: output.status.code().expect("process terminated by signal"),
            std_out: output.stdout,
            std_err: output.stderr,
        }
    }
}

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

#[derive(Debug)]
pub struct Command {
    pub cmd_line_string: String,
    cmd_line: Vec<String>,
    input_path: Option<String>,
    redirection_path: Option<String>,
}

impl Command {
    pub fn new(cmd_line_string: &str) -> Result<Self, String> {
        match shlex::split(cmd_line_string) {
            Some(mut cmd_line) => {
                if cmd_line.is_empty() {
                    Err("Empty command line.".to_string())
                } else {
                    let input_path = match cmd_line.iter().position(|x| *x == "<") {
                        Some(i_index) => match cmd_line.get(i_index + 1) {
                            Some(path) => {
                                let path = path.clone();
                                cmd_line.remove(i_index);
                                cmd_line.remove(i_index);
                                Some(path)
                            }
                            None => return Err("expected input file path".to_string()),
                        },
                        None => None,
                    };
                    let redirection_path = match cmd_line.iter().position(|x| *x == ">") {
                        Some(red_index) => match cmd_line.get(red_index + 1) {
                            Some(path) => {
                                let path = path.clone();
                                cmd_line.remove(red_index);
                                cmd_line.remove(red_index);
                                Some(path)
                            }
                            None => return Err("expected input file path".to_string()),
                        },
                        None => None,
                    };
                    Ok(Command {
                        cmd_line_string: String::from(cmd_line_string),
                        cmd_line,
                        input_path,
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
                2 => {
                    env::set_current_dir(&self.cmd_line[1])?;
                    env::set_var("PWD", env::current_dir()?);
                    Ok(Outcome::default())
                }
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
            program_name => {
                let input_file = match self.input_path {
                    Some(ref path) => std::process::Stdio::from(std::fs::File::open(path)?),
                    None => std::process::Stdio::null(),
                };
                let output_file = match self.redirection_path {
                    Some(ref path) => std::process::Stdio::from(std::fs::File::open(path)?),
                    None => std::process::Stdio::piped(),
                };
                Ok(Outcome::from(
                    std::process::Command::new(program_name)
                        .args(&self.cmd_line[1..])
                        .stdin(input_file)
                        .stdout(output_file)
                        .output()?,
                ))
            }
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;

    #[test]
    fn new_command() {
        println!("{:?}", Command::new("whatever x y < bbb > aaa"))
    }
}
