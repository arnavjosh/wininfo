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
use std::error::Error;
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
    .args(["cpu", "memory", "disks", "network", "all"])
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

    /// Display network adapter information
    #[arg(short = 'n', long)]
    network: bool,

    /// Show only enabled network adapters
    #[arg(long = "only-enabled", global = true)]
    only_enabled: bool,

    #[arg(long, value_name = "INDEX|DRIVE", conflicts_with = "disks")]
    disk: Option<String>,

    /// Display all system information
    #[arg(short = 'a', long)]
    all: bool,

    /// Output information as JSON
    #[arg(short = 'j', long)]
    json: bool,

    /// Use compact output
    #[arg(short = 'q', long)]
    quiet: bool,

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
    /// Display network adapter information
    Network,
    /// Display all system information
    Info,
}

fn main() -> CliResult<()> {
    let cli = Cli::parse();

    let system = System::new()?;

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
            Commands::Network => print_network(system, cli)?,
            Commands::Info => print_all(system, cli)?,
        }

        return Ok(());
    }

    if !cli.cpu
        && !cli.memory
        && !cli.disks
        && !cli.network
        && !cli.only_enabled
        && !cli.all
        && cli.disk.is_none()
    {
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

    if cli.disks || cli.disk.is_some() {
        if !first && !cli.quiet {
            println!();
        }

        print_disks(system, cli)?;
        first = false;
    }

    if cli.network || cli.only_enabled {
        if !first && !cli.quiet {
            println!();
        }

        print_network(system, cli)?;
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
    print_network(system, cli)?;

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

fn print_network(system: &System, cli: &Cli) -> CliResult<()> {
    let adapters = network_adapters(system, cli)?;

    if !cli.quiet {
        section_header("NETWORK");
    }

    for adapter in &adapters {
        println!();
        println!("  {}", adapter.name());
        println!(
            "  {:<24} {}",
            "MAC address",
            adapter.mac_address().unwrap_or("Unknown")
        );
        println!(
            "  {:<24} {}",
            "IPv4 address",
            adapter
                .ipv4_address()
                .map(|address| address.to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        );
        println!("  {:<24} {}", "Enabled", adapter.enabled());

        match adapter.speed() {
            Some(speed) => println!("  {:<24} {} bps", "Speed", speed),
            None => println!("  {:<24} Unknown", "Speed"),
        }
    }

    if !cli.quiet {
        println!();
    }

    Ok(())
}

fn network_adapters(system: &System, cli: &Cli) -> CliResult<Vec<wininfo::NetworkAdapterInfo>> {
    let adapters = system.network_adapters()?;

    if cli.only_enabled {
        Ok(adapters.into_iter().filter(|adapter| adapter.enabled()).collect())
    } else {
        Ok(adapters)
    }
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

    if let Some(index) = &cli.disk {
        let disk = select_disk(&disks, &index)?;

        print_disk(disk);
        return Ok(());
    }

    for disk in &disks {
        print_disk(disk);
    }

    if !cli.quiet {
        println!();
    }

    Ok(())
}

fn print_disk(disk: &wininfo::DiskInfo) {
    println!();
    println!("  {}", disk.device_id());

    match (disk.size(), disk.free()) {
        (Some(size), Some(free)) => {
            let used = size.subtract(free).unwrap_or_default();
            let usage = percentage(used, size);

            println!("  {:<12} {} bytes", "Used", used.as_u64());
            println!("  {:<12} {} bytes", "Free", free.as_u64());
            println!("  {:<12} {} bytes", "Total", size.as_u64());
            println!("  {:<12} {:.1}%", "Usage", usage);
        }
        (Some(size), None) => {
            println!("  {:<12} {} bytes", "Total", size.as_u64());
            println!("  {:<12} Unknown", "Free");
        }
        (None, Some(free)) => {
            println!("  {:<12} {} bytes", "Free", free.as_u64());
            println!("  {:<12} Unknown", "Total");
        }
        (None, None) => {
            println!("  Disk capacity information not available.");
        }
    }
}

fn print_json(system: &System, cli: &Cli) -> CliResult<()> {
    if cli.all
        || (!cli.cpu && !cli.memory && !cli.disks && !cli.network && !cli.only_enabled)
    {
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

    if let Some(index) = &cli.disk {
        let disks = system.disks()?;
        let disk = select_disk(&disks, &index)?;

        output.insert("disk".into(), serde_json::to_value(disk)?);
    } else if cli.disks {
        output.insert("disks".into(), serde_json::to_value(system.disks()?)?);
    }

    if cli.network || cli.only_enabled {
        output.insert(
            "network_adapters".into(),
            serde_json::to_value(network_adapters(system, cli)?)?,
        );
    }

    let json = serde_json::to_string_pretty(&Value::Object(output))?;
    println!("{json}");

    Ok(())
}

fn select_disk<'a>(
    disks: &'a [wininfo::DiskInfo],
    selector: &str,
) -> CliResult<&'a wininfo::DiskInfo> {
    if let Ok(index) = selector.parse::<usize>() {
        return disks
            .get(index)
            .ok_or_else(|| format!("Disk index {index} does not exist"))
            .map_err(Into::into);
    }

    let normalized_selector = normalize_drive_selector(selector);

    disks
        .iter()
        .find(|disk| normalize_drive_selector(disk.device_id()) == normalized_selector)
        .ok_or_else(|| format!("Disk {selector} does not exist"))
        .map_err(Into::into)
}

fn normalize_drive_selector(selector: &str) -> String {
    let trimmed = selector.trim().trim_end_matches(':');
    trimmed.to_ascii_uppercase() + ":"
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
