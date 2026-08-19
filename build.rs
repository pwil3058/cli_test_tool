// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use std::process::Command;

use lalr1_lib::ParserGenerator;

fn main() {
    println!("cargo:rerun-if-changed=src/command_action.laps");
    match ParserGenerator::new("src/command_action.laps") {
        Ok(generator) => match generator.write_parser_code_to_file("src/command_action.rs") {
            Ok(_) => {
                Command::new("rustfmt")
                    .args(&["src/command_action.rs"])
                    .status()
                    .expect("prebuild: cargo run rustfmt failed");
            }
            Err(e) => panic!("failed prebuild: {}", e),
        },
        Err(e) => panic!("Build error{}", e),
    }
    println!("cargo:rerun-if-changed=build.rs");
}
