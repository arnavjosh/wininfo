#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::assertions_on_result_states
    )
)]
use byte_unit::Byte;
use clap::{ArgGroup, Parser, Subcommand};
use serde_json::{Map, Value};
use std::{
    error::Error,
    io::{self, Write},
    thread,
    time::Duration,
};
use wininfo::System;

type CliResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    name = "wininfo_cli",
    version,
    about = "A Windows system information tool",
    long_about = "
"
)]
#[command(group(
    ArgGroup::new("info")
        .args(["cpu", "memory", "disks", "all"])
        .multiple(true)
))]
struct Cli {
    /// Display CPU information
    #[arg(short = 'c', long)]
    cpu: bool,

    /// Display memory information
    #[arg(short = 'm', long)]
    memory: bool,

    /// Display disk information
    #[arg(short = 'd', long)]
    disks: bool,

    /// Display all system information
    #[arg(short = 'a', long)]
    all: bool,

    /// Output information as JSON
    #[arg(short = 'j', long)]
    json: bool,

    /// Use compact output
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Refresh the output every N seconds
    #[arg(long, value_name = "SECONDS")]
    watch: Option<u64>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Display CPU information
    Cpu,

    /// Display memory information
    Memory,
    /// Display disk information
    Disks,
    /// Display all system information
    Info,
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    let system = System::new()?;

    if let Some(interval) = cli.watch {
        if interval == 0 {
            return Err("--watch interval must be greater than zero".into());
        }
        if cli.json {
            //can't make a properly updating system if it's a json print
            return Err("--watch cannot be combined with --json".into());
        }
        return watch(&system, &cli, interval);
    }

    run_once(&system, &cli)
}

fn run_once(system: &System, cli: &Cli) -> CliResult<()> {
    if cli.json {
        return print_json(system, cli);
    }

    if let Some(command) = &cli.command {
        match command {
            Commands::Cpu => print_cpu(system, cli)?,
            Commands::Memory => print_memory(system, cli)?,
            Commands::Disks => print_disks(system, cli)?,
            Commands::Info => print_all(system, cli)?,
        }

        return Ok(());
    }

    if !cli.cpu && !cli.memory && !cli.disks && !cli.all {
        return print_all(system, cli);
    }

    if cli.all {
        return print_all(system, cli);
    }

    let mut first = true;

    if cli.cpu {
        if !first && !cli.quiet {
            println!();
        }

        print_cpu(system, cli)?;
        first = false;
    }

    if cli.memory {
        if !first && !cli.quiet {
            println!();
        }

        print_memory(system, cli)?;
        first = false;
    }

    if cli.disks {
        if !first && !cli.quiet {
            println!();
        }

        print_disks(system, cli)?;
    }

    Ok(())
}

fn print_all(system: &System, cli: &Cli) -> CliResult<()> {
    if !cli.quiet {
        println!();
        println!("WININFO");
        println!("Windows system information");
        println!();
    }

    print_cpu(system, cli)?;
    print_memory(system, cli)?;
    print_disks(system, cli)?;

    Ok(())
}

fn print_cpu(system: &System, cli: &Cli) -> CliResult<()> {
    let cpu = system.cpu()?;

    if !cli.quiet {
        section_header("CPU");
    }

    println!("  {:<24} {}", "Name", cpu.name());
    println!("  {:<24} {}", "Manufacturer", cpu.manufacturer());

    match cpu.physical_cores() {
        Some(cores) => println!("  {:<24} {}", "Physical cores", cores),
        None => println!("  {:<24} {}", "Physical cores", "Unknown"),
    }

    match cpu.logical_cores() {
        Some(cores) => println!("  {:<24} {}", "Logical cores", cores),
        None => println!("  {:<24} {}", "Logical cores", "Unknown"),
    }

    match cpu.max_clock_speed_ghz() {
        Some(speed) => println!("  {:<24} {:.2} GHz", "Max clock speed", speed),
        None => println!("  {:<24} {}", "Max clock speed", "Unknown"),
    }
    if !cli.quiet {
        println!();
    }

    Ok(())
}

fn print_memory(system: &System, cli: &Cli) -> CliResult<()> {
    let memory = system.memory()?;

    let total = memory.total();
    let free = memory.free();
    let used = total.subtract(free).unwrap_or_default();

    let usage_percent = percentage(used, total);

    if !cli.quiet {
        section_header("memory");
    }

    println!("  {:<24} {} bytes", "Total", total.as_u64());
    println!("  {:<24} {} bytes", "Used", used.as_u64());
    println!("  {:<24} {} bytes", "Free", free.as_u64());
    println!("  {:<24} {:.1}%", "Usage", usage_percent);

    if !cli.quiet {
        println!();
    }

    Ok(())
}

fn print_disks(system: &System, cli: &Cli) -> CliResult<()> {
    let disks = system.disks()?;

    if !cli.quiet {
        section_header("DISKS");
    }

    if disks.is_empty() {
        println!("  No disks wer e found.");

        if !cli.quiet {
            println!();
        }

        return Ok(());
    }

    for disk in disks {
        println!();
        println!("  {}", disk.device_id());

        match (disk.size(), disk.free()) {
            (Some(size), Some(free)) => {
                let used = size.subtract(free).unwrap_or_default();
                let usage_percent = percentage(used, size);

                println!("  {:<12} {} bytes", "Used", used.as_u64());
                println!("  {:<12} {} bytes", "Free", free.as_u64());
                println!("  {:<12} {} bytes", "Total", size.as_u64());
                println!("  {:<12} {:.1}%", "Usage", usage_percent);
            }

            (Some(size), None) => {
                println!("  {:<12} {} bytes", "Total", size.as_u64());
                println!("  {:<12} {}", "Free", "Unknown");
            }

            (None, Some(free)) => {
                println!("  {:<12} {} bytes", "Free", free.as_u64());
                println!("  {:<12} {}", "Total", "Unknown");
            }

            (None, None) => {
                println!("  Disk capacity information unavailable.");
            }
        }
    }

    if !cli.quiet {
        println!();
    }

    Ok(())
}

fn print_json(system: &System, cli: &Cli) -> CliResult<()> {
    if cli.all || (!cli.cpu && !cli.memory && !cli.disks) {
        let info = system.info()?;

        println!("{}", serde_json::to_string_pretty(&info)?);

        return Ok(());
    }

    let mut output = Map::new();

    if cli.cpu {
        let cpu = system.cpu()?;
        output.insert("cpu".to_owned(), serde_json::to_value(cpu)?);
    }

    if cli.memory {
        let memory = system.memory()?;
        output.insert("memory".to_owned(), serde_json::to_value(memory)?);
    }

    if cli.disks {
        let disks = system.disks()?;
        output.insert("disks".to_owned(), serde_json::to_value(disks)?);
    }

    let json = serde_json::to_string_pretty(&Value::Object(output))?;
    println!("{json}");

    Ok(())
}

fn watch(system: &System, cli: &Cli, interval: u64) -> CliResult<()> {
    loop {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;

        run_once(system, cli)?;

        println!();
        println!("Refreshing every {interval}s · Press Ctrl+C to exit");

        thread::sleep(Duration::from_secs(interval));
    }
}

fn format_byte(byte: Byte) -> String {
    format!("{byte:#.2}")
}

fn percentage(value: Byte, total: Byte) -> f64 {
    let total_bytes = total.as_u64();

    if total_bytes == 0 {
        return 0.0;
    }

    (value.as_u64() as f64 / total_bytes as f64) * 100.0
}

fn section_header(title: &str) {
    println!("{title}");
    println!("{}", "─".repeat(64));
}
