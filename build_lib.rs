// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use std::process::Command;

use lalr1;

fn main() {
    println!("cargo:rerun-if-changed=src/parse.laps");
    let specification = lalr1::specification();
    match Command::new("lalr1_gen")
        .args(&["-f", "src/parse.laps"])
        .status()
    {
        Ok(status) => {
            if status.success() {
                Command::new("rustfmt")
                    .args(&["src/parse.rs"])
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
