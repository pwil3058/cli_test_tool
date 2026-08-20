// Copyright 2022 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use crate::command_action::CommandAction;
use crate::error::Error;
use crate::script::EnvVars;
use lalr1::Parser;
use std::convert::From;
use std::env;

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
    pub cmd_action: CommandAction,
}

impl Command {
    pub fn new(cmd_line_string: &str) -> Result<Self, &'static str> {
        let mut cmd_action: CommandAction = Default::default();
        if let Err(_) = cmd_action.parse_text(cmd_line_string, "command") {
            return Err("Command not parseable");
        };
        Ok(Self {
            cmd_line_string: cmd_line_string.to_string(),
            cmd_action,
        })
    }

    pub fn run(&self, env_vars: &mut EnvVars) -> Result<Outcome, Error> {
        use CommandAction::*;
        match &self.cmd_action {
            SetEnvVar(var, value) => {
                env_vars.set_var(&var, &value);
                Ok(Outcome::default())
            }
            UnsetEnvVar(var) => {
                env_vars.remove_var(&var);
                Ok(Outcome::default())
            }
            ChangeDir(dir) => {
                env::set_current_dir(&dir)?;
                let _ = env_vars.set_var("PWD", &env::current_dir()?.to_string_lossy());
                Ok(Outcome::default())
            }
            RunProgram(program_name, args, input_path, output_path, err_output_path) => {
                let input_file = match input_path {
                    Some(path) => std::process::Stdio::from(std::fs::File::open(path)?),
                    None => std::process::Stdio::null(),
                };
                let output_file = match output_path {
                    Some((path, overwrite)) => {
                        if *overwrite {
                            std::process::Stdio::from(std::fs::File::create(path)?)
                        } else {
                            let file = std::fs::OpenOptions::new()
                                .append(true)
                                .write(true)
                                .create(true)
                                .open(path)?;
                            std::process::Stdio::from(file)
                        }
                    }
                    None => std::process::Stdio::piped(),
                };
                let err_output_file = match err_output_path {
                    Some((path, overwrite)) => {
                        if *overwrite {
                            std::process::Stdio::from(std::fs::File::create(path)?)
                        } else {
                            let file = std::fs::OpenOptions::new()
                                .append(true)
                                .write(true)
                                .create(true)
                                .open(path)?;
                            std::process::Stdio::from(file)
                        }
                    }
                    None => std::process::Stdio::piped(),
                };
                Ok(Outcome::from(
                    std::process::Command::new(program_name)
                        .args(args.iter())
                        .stdin(input_file)
                        .stdout(output_file)
                        .stderr(err_output_file)
                        .envs(&env_vars.0)
                        .output()?,
                ))
            }
            Default => Err(Error::Why("Uninitialized CommandAction")),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::{Command, Outcome};
    use crate::command_action::CommandAction;
    use crate::script::EnvVars;

    #[test]
    fn new_command() {
        use CommandAction::*;
        let cmd = Command::new("whatever x y < bbb > aaa").unwrap();
        match &cmd.cmd_action {
            RunProgram(program_name, args, input_path, output_path, err_output_path) => {
                assert_eq!(program_name, "whatever");
                assert_eq!(*args, ["x", "y"]);
                assert_eq!(*input_path, Some("bbb".to_string()));
                assert_eq!(*output_path, Some(("aaa".to_string(), true)));
                assert_eq!(*err_output_path, None);
            }
            _ => assert!(false),
        }
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
