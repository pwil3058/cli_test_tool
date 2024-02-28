// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

mod command;
mod script;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use tempdir::TempDir;

/// CLI Test Tool
#[derive(Debug, StructOpt)]
#[structopt(about = "Run CLI test scripts")]
struct CLIOptions {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(short = "t", long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
    /// Run test in a clean temporary directory
    #[structopt(short, long)]
    use_temp_dir: bool,
    /// The name/path of the file containing the test script
    #[structopt(required = true)]
    script: PathBuf,
}

fn main() {
    let cli_options = CLIOptions::from_args();

    stderrlog::new()
        .quiet(cli_options.quiet)
        .verbosity(cli_options.verbose)
        .timestamp(cli_options.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    log::debug!("CLI Test Tool is under construction: {:?}", cli_options);

    let new_script = script::Script::read_from(&cli_options.script).unwrap();
    println!("NEW: {new_script:?}");

    let mut script = String::new();
    match File::open(&cli_options.script) {
        Ok(mut file) => match file.read_to_string(&mut script) {
            Ok(size) => log::trace!("Read {} bytes", size),
            Err(err) => {
                log::error!("Error reading script file: {}", err);
                std::process::exit(-1);
            }
        },
        Err(err) => {
            log::error!(
                "Error opening script file: {:?}: {}. Aborting.",
                cli_options.script,
                err
            );
            std::process::exit(-1);
        }
    }

    let tempdir = if cli_options.use_temp_dir {
        match TempDir::new("cli_test") {
            Ok(tempdir) => {
                log::info!("{:?}: temporary created", tempdir.path());
                if let Err(err) = std::env::set_current_dir(tempdir.path()) {
                    log::error!(
                        "Failed to make {:?} the current directory: {}. Aborting.",
                        tempdir.path(),
                        err
                    );
                    std::process::exit(-1);
                };
                Some(tempdir)
            }
            Err(err) => {
                log::error!("Failed to create temporary directory ({}). Aborting!", err);
                std::process::exit(-1);
            }
        }
    } else {
        None
    };
    log::info!("Current working directory: {:?}", std::env::current_dir());

    println!("Script: {}", script);
    let lines: Vec<&str> = script.lines().collect();
    for line in lines {
        println!("Line: {:?} : {}", line, line.starts_with('$'));
        if let Some(stripped) = line.strip_prefix('$') {
            let cmd = command::Command::new(stripped);
            println!("Command: {:?}", cmd);
            if let Ok(cmd) = cmd {
                println!("{:?}", cmd.cmd_line_string);
                let outcome = cmd.run();
                println!("Outcome: {:?}", &outcome);
                if let Ok(outcome) = &outcome {
                    println!("stdout: {:?}", &outcome.std_out);
                    println!("stderr: {:?}", &outcome.std_err);
                }
            }
        }
    }

    if let Some(tempdir) = tempdir {
        if let Err(err) = tempdir.close() {
            log::error!("Problem closing temporary directory: {}", err);
        }
    }

    if !cli_options.quiet {
        println!("{:?}: PASSED", cli_options.script)
    }
}
