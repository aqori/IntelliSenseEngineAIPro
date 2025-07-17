// src/main.rs
/*
 * Main executable for IntelliSenseEngineAIPro
 */

use clap::Parser;
use intellisenseengineaipro::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliSenseEngineAIPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
