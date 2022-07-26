// Copyright 2022 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

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
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;

    #[test]
    fn new_command() {
        println!("{:?}", Command::new("ls > aaa".to_string()))
    }
}
