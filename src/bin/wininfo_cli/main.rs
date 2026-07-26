#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::assertions_on_result_states
    )
)]

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::error::Error;
use wininfo::System;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(about = "wininfo_cli - Windows system information tool", long_about = None)]
#[command(version)]
// The \x1B[4m and \x1B[0m are ANSI escape codes to underline the text and to reset formatting
#[command(after_help = "\x1B[4mExamples:\x1B[0m\n\
    # Print all system information\n\
    wininfo_cli\n\
    # Print only memory information\n\
    wininfo_cli memory\n")]
struct Cli {
    // Then subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// print system memory info.
    Memory {},
    /// print system CPU info.
    Cpu {},
    /// print system disk info.
    Disks {},
}

#[cfg(feature = "cli")]
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let system = System::new()?;

    let out = match &cli.command {
        Some(Commands::Cpu {}) => serde_json::to_string_pretty(&system.cpu()?),
        Some(Commands::Memory {}) => serde_json::to_string_pretty(&system.memory()?),
        Some(Commands::Disks {}) => serde_json::to_string_pretty(&system.disks()?),
        None => serde_json::to_string_pretty(&system.info()?),
    }?;
    println!("{:}", out);
    Ok(())
}
