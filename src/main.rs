// Copyright 2021 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use log;
use std::path::PathBuf;
use stderrlog;
use structopt::StructOpt;
use tempdir;
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
    if cli_options.use_temp_dir {
        match TempDir::new("cli_test") {
            Ok(tempdir) => log::info!("{:?}: temporary created", tempdir.path()),
            Err(err) => {
                log::error!("Failed to create temporary directory ({}). Aborting!", err);
                std::process::exit(-1);
            }
        }
    }
}
