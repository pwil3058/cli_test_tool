// Copyright 2022 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use crate::error::Error;
use std::convert::From;
use std::env;

use crate::script::EnvVars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Outcome {
    pub e_code: Option<i32>,
    pub std_out: String,
    pub std_err: String,
}

impl Default for Outcome {
    fn default() -> Self {
        Self {
            e_code: Some(0),
            std_out: String::new(),
            std_err: String::new(),
        }
    }
}

impl From<std::process::Output> for Outcome {
    fn from(output: std::process::Output) -> Self {
        Outcome {
            e_code: output.status.code(),
            std_out: String::from_utf8(output.stdout).unwrap(),
            std_err: String::from_utf8(output.stderr).unwrap(),
        }
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
    pub fn new(cmd_line_string: &str) -> Result<Self, &'static str> {
        match shlex::split(cmd_line_string) {
            Some(mut cmd_line) => {
                if cmd_line.is_empty() {
                    Err("Empty command line.")
                } else {
                    let input_path = match cmd_line.iter().position(|x| *x == "<") {
                        Some(i_index) => match cmd_line.get(i_index + 1) {
                            Some(path) => {
                                let path = path.clone();
                                cmd_line.remove(i_index);
                                cmd_line.remove(i_index);
                                Some(path)
                            }
                            None => return Err("expected input file path"),
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
                            None => return Err("expected input file path"),
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
            None => Err("Poorly formed command line"),
        }
    }

    pub fn run(&self, env_vars: &mut EnvVars) -> Result<Outcome, Error> {
        if self.cmd_line[0].find("=").is_some() {
            let pair: Vec<&str> = self.cmd_line[0].as_str().split('=').collect();
            if pair.len() == 2 {
                // env::set_var(pair[0], pair[1]);
                let _ = env_vars.set_var(pair[0].into(), pair[1].into());
            } else {
                return Err(Error::Why("expected \"ARG=VALUE\""));
            }
            return Ok(Outcome::default());
        }
        match self.cmd_line[0].as_str() {
            "umask" => Err(Error::Why("\"umask\" is not available")),
            "cd" => match self.cmd_line.len() {
                2 => {
                    env::set_current_dir(&self.cmd_line[1])?;
                    // env::set_var("PWD", env::current_dir()?);
                    let _ = env_vars.set_var("PWD", &env::current_dir()?.to_string_lossy());
                    Ok(Outcome::default())
                }
                _ => Err(Error::Why("expected exactly one argument")),
            },
            "unset" => {
                for var in &self.cmd_line[1..] {
                    // env::remove_var(var);
                    // let os_var = OsString::from(var);
                    let _ = env_vars.remove_var(var);
                }
                Ok(Outcome::default())
            }
            program_name => {
                let input_file = match self.input_path {
                    Some(ref path) => std::process::Stdio::from(std::fs::File::open(path)?),
                    None => std::process::Stdio::null(),
                };
                let output_file = match self.redirection_path {
                    Some(ref path) => std::process::Stdio::from(std::fs::File::create(path)?),
                    None => std::process::Stdio::piped(),
                };
                Ok(Outcome::from(
                    std::process::Command::new(program_name)
                        .args(&self.cmd_line[1..])
                        .stdin(input_file)
                        .stdout(output_file)
                        .envs(&env_vars.0)
                        .output()?,
                ))
            }
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::{Command, Outcome};
    use crate::script::EnvVars;

    #[test]
    fn new_command() {
        let cmd = Command::new("whatever x y < bbb > aaa").unwrap();
        println!("{:?}", cmd);
        assert_eq!(cmd.cmd_line[0], "whatever");
        assert_eq!(cmd.cmd_line[1..], ["x", "y"]);
        assert_eq!(cmd.input_path, Some("bbb".to_string()));
        assert_eq!(cmd.redirection_path, Some("aaa".to_string()));
        let env_vars = &mut EnvVars::new();
        let result = cmd.run(env_vars).unwrap_err().to_string();
        assert_eq!(result, "IOError: No such file or directory (os error 2)");
    }

    #[test]
    fn set_var_test() {
        let cmd = Command::new("MYNAME=Peter").unwrap();
        let env_vars = &mut EnvVars::new();
        let result = cmd.run(env_vars);
        println!("{:?}", result);
        assert_eq!(
            result.unwrap(),
            Outcome {
                e_code: Some(0),
                std_out: "".to_string(),
                std_err: "".to_string(),
            }
        );
        assert_eq!(env_vars.var("MYNAME").unwrap(), "Peter");
    }
}
