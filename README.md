
# wininfo

`wininfo` is a Windows system information library for Rust. It exposes a small API for collecting CPU, memory, and disk data through WMI and returns the results as strongly typed Rust structures.

## Features

- Query CPU, memory, and disk information from Windows.
- Collect a complete system snapshot with one call.
- Serialize the returned data with `serde`.
- Use the library directly or build on the included CLI binary.

## Platform Support

`wininfo` is Windows-only at runtime. Creating a `System` on a non-Windows platform returns `WinInfoError::Unsupported`.

## Installation

Add the crate to your project:

```toml
[dependencies]
wininfo = "0.1"
```

By default, the crate enables the `cli` feature. If you only want the library API, disable default features:

```toml
[dependencies]
wininfo = { version = "0.1", default-features = false }
```

## Library Overview

The main entry point is `wininfo::System`.

It provides:

- `System::new()` to create a system provider.
- `system.cpu()?` to query CPU information.
- `system.memory()?` to query memory information.
- `system.disks()?` to query disk information.
- `system.info()?` to collect a full snapshot.

The crate also exports:

- `CpuInfo`
- `MemoryInfo`
- `DiskInfo`
- `Info`
- `Byte`
- `Result<T>`
- `WinInfoError`

## Example

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

## Data Types

`CpuInfo` exposes the CPU name, manufacturer, physical core count, logical core count, and maximum clock speed in MHz. It also provides `max_clock_speed_ghz()` for a GHz view.

`MemoryInfo` exposes total and free memory as `Byte` values.

`DiskInfo` exposes the device ID plus optional size and free-space values.

`Info` groups `cpu`, `memory`, and `disks` into one snapshot.

## CLI

The crate includes a `wininfo_cli` binary for terminal use. Run it with Cargo:

```bash
cargo run --bin wininfo_cli
```

You can use either top-level selectors or subcommands:

- `wininfo_cli` prints a full system snapshot.
- `wininfo_cli --cpu`, `--memory`, `--disks`, and `--all` select specific sections.
- `wininfo_cli cpu`, `memory`, `disks`, and `info` provide the same data through subcommands.
- `wininfo_cli --json` prints structured JSON.
- `wininfo_cli --quiet` suppresses section headers and extra spacing.

Examples:

```bash
cargo run --bin wininfo_cli -- --cpu
cargo run --bin wininfo_cli -- memory
cargo run --bin wininfo_cli -- --all --json
```

The CLI is intentionally plain-text only and no longer supports refresh or colorized output.

## License

Licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT).
