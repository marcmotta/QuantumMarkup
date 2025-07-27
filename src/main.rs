// src/main.rs
/*
 * Main executable for QuantumMarkup
 */

use clap::Parser;
use quantummarkup::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumMarkup - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
