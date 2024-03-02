// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

mod command;
mod error;
mod script;

use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tempdir::TempDir;

/// CLI Test Tool
#[derive(Debug, StructOpt)]
#[structopt(about = "Run CLI test scripts")]
struct CLIOptions {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    _quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    /// Run test in a clean temporary directory
    #[structopt(short, long)]
    use_temp_dir: bool,
    /// The name/path of the file containing the test script
    #[structopt(required = true)]
    script: PathBuf,
}

fn main() {
    let cli_options = CLIOptions::from_args();

    let script = match script::Script::read_from(&cli_options.script) {
        Ok(script) => script,
        Err(err) => {
            writeln!(io::stderr(), "Error: reading script failed: {err}").expect("stderr failed");
            std::process::exit(-1);
        }
    };
    if cli_options.verbose > 3 {
        println!("Script: {script:?}");
    }

    let tempdir = if cli_options.use_temp_dir {
        match TempDir::new("cli_test") {
            Ok(tempdir) => {
                if let Err(err) = std::env::set_current_dir(tempdir.path()) {
                    if let Err(err) = tempdir.close() {
                        writeln!(io::stderr(), "Error: tempdir.close() failed: {err}")
                            .expect("stderr failed");
                    };
                    writeln!(io::stderr(), "Error: cd to tempdir failed: {err}")
                        .expect("stderr failed");
                    std::process::exit(-1);
                };
                Some(tempdir)
            }
            Err(err) => {
                writeln!(io::stderr(), "Error: failed to create tempdir: {err}")
                    .expect("stderr failed");
                std::process::exit(-1);
            }
        }
    } else {
        None
    };

    let result = script.evaluate();

    if let Some(tempdir) = tempdir {
        if let Err(err) = tempdir.close() {
            writeln!(io::stderr(), "Error: tempdir.close() failed: {err}").expect("stderr failed");
        }
    }

    match result {
        Ok(evaluation) => {
            println!("{evaluation}");
        }
        Err(err) => {
            writeln!(io::stderr(), "Error: script evaluation failed: {err}")
                .expect("stderr failed");
            std::process::exit(-1);
        }
    }
}
