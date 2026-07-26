# wininfo

`wininfo` is a small Windows system information library and CLI. It uses WMI to collect CPU, memory, and disk data and exposes the results as Rust types or pretty-printed JSON.

## Features

- Library API for querying system information from Windows.
- CLI that prints system, CPU, memory, or disk information.
- Serde support for serializing the returned data structures.
- MIT licensed.

## Platform Support

This crate is Windows-only at runtime. Creating a `System` on non-Windows platforms returns `WinInfoError::Unsupported`.

## Installation

Add the crate to your project:

```toml
[dependencies]
wininfo = "0.1"
```

The CLI is enabled by default through the `cli` feature.

## Library Usage

```rust
use wininfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = System::new()?;

    let cpu = system.cpu()?;
    println!("CPU: {}", cpu.name());
    println!("Manufacturer: {}", cpu.manufacturer());
    println!("Physical cores: {:?}", cpu.physical_cores());
    println!("Logical cores: {:?}", cpu.logical_cores());
    println!("Max clock speed: {:?} MHz", cpu.max_clock_speed_mhz());

    let memory = system.memory()?;
    println!("Total memory: {}", memory.total());
    println!("Free memory: {}", memory.free());

    for disk in system.disks()? {
        println!("Disk: {}", disk.device_id());
        println!("  Size: {:?}", disk.size());
        println!("  Free: {:?}", disk.free());
    }

    let info = system.info()?;
    println!("Collected full system snapshot: {info:?}");

    Ok(())
}
```

The main entry point is `System::new()`, which gives access to:

- `system.cpu()?`
- `system.memory()?`
- `system.disks()?`
- `system.info()?`

## CLI Usage

Run the CLI with Cargo:

```bash
cargo run --bin wininfo_cli
```

Available subcommands:

- `wininfo_cli` prints a full system snapshot.
- `wininfo_cli cpu` prints CPU information.
- `wininfo_cli memory` prints memory information.
- `wininfo_cli disks` prints disk information.

Examples:

```bash
cargo run --bin wininfo_cli -- cpu
cargo run --bin wininfo_cli -- memory
cargo run --bin wininfo_cli -- disks
```

The CLI prints formatted JSON to standard output.

## Data Types

- `CpuInfo` includes the CPU name, manufacturer, physical cores, logical cores, and maximum clock speed.
- `MemoryInfo` includes total and free memory.
- `DiskInfo` includes the device ID plus optional size and free-space values.

## License

Licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT).