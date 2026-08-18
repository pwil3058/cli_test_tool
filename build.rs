// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/command_action.laps");
    match Command::new("lalr1_gen")
        .args(&["-f", "src/command_action.laps"])
        .status()
    {
        Ok(status) => {
            if status.success() {
                Command::new("rustfmt")
                    .args(&["src/command_action.rs"])
                    .status()
                    .unwrap();
            } else {
                panic!("failed prebuild: {}", status);
            };
        }
        Err(err) => panic!("Build error: {}", err),
    }
    println!("cargo:rerun-if-changed=build.rs");
}
