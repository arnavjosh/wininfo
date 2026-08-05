
# wininfo

`wininfo` is a small Windows-focused Rust library for collecting system information through WMI. It exposes typed structures for CPU, memory, disk, and network adapter data and also ships with a simple CLI binary for quick inspection from the terminal.

## Features

- Query CPU, memory, disk, and network adapter information on Windows.
- Gather a complete system snapshot in one call.
- Serialize results with `serde` and `serde_json`.
- Use the library directly or run the included `wininfo_cli` binary.

## Platform support

`wininfo` is Windows-only at runtime. Creating a `System` on a non-Windows platform returns `WinInfoError::Unsupported`.

## Installation

Add the crate to your project:

```toml
[dependencies]
wininfo = "0.1.0"
```

By default, the crate enables the `cli` feature. If you only want the library API, disable default features:

```toml
[dependencies]
wininfo = { version = "0.1.0", default-features = false }
```

## Library overview

The main entry point is `wininfo::System`.

It provides:

- `System::new()` to create a system provider.
- `system.cpu()?` to query CPU information.
- `system.memory()?` to query memory information.
- `system.disks()?` to query disk information.
- `system.network_adapters()?` to query network adapter information.
- `system.info()?` to collect a full snapshot.

The crate also exports:

- `CpuInfo`
- `MemoryInfo`
- `DiskInfo`
- `NetworkAdapterInfo`
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
    println!("Max clock speed: {:?} GHz", cpu.max_clock_speed_ghz());

    let memory = system.memory()?;
    println!("Total memory: {}", memory.total());
    println!("Free memory: {}", memory.free());

    for disk in system.disks()? {
        println!("Disk: {}", disk.device_id());
        println!("  Size: {:?}", disk.size());
        println!("  Free: {:?}", disk.free());
    }

    for adapter in system.network_adapters()? {
        println!("Network adapter: {}", adapter.name());
        println!("  MAC: {:?}", adapter.mac_address());
        println!("  IPv4: {:?}", adapter.ipv4_address());
        println!("  Enabled: {}", adapter.enabled());
        println!("  Speed: {:?}", adapter.speed());
    }

    let info = system.info()?;
    println!("Collected full system snapshot: {info:?}");

    Ok(())
}
```

## Data types

- `CpuInfo` exposes the CPU name, manufacturer, physical and logical core counts, and maximum clock speed.
- `MemoryInfo` exposes total and free memory as `Byte` values.
- `DiskInfo` exposes the device ID and optional size/free-space values.
- `NetworkAdapterInfo` exposes the adapter name, MAC address, IPv4 address, enabled state, and speed.
- `Info` groups `cpu`, `memory`, `disks`, and `network_adapters` into a single snapshot.

## CLI

The crate includes a `wininfo_cli` binary for terminal use. Run it with Cargo:

```bash
cargo run --bin wininfo_cli
```

Use either flags or subcommands:

- `wininfo_cli` prints a full system snapshot by default.
- `wininfo_cli --cpu`, `--memory`, `--disks`, `--network`, `--disk 0`, and `--all` select specific sections.
- `wininfo_cli --only-enabled --network` shows only enabled network adapters.
- `wininfo_cli cpu`, `memory`, `disks`, `network`, and `info` provide the same data through subcommands.
- `wininfo_cli --json` prints structured JSON output.
- `wininfo_cli --quiet` suppresses section headers and extra spacing.

Examples:

```bash
cargo run --bin wininfo_cli -- --cpu
cargo run --bin wininfo_cli -- memory
cargo run --bin wininfo_cli -- --all --json
cargo run --bin wininfo_cli -- --disk 0
cargo run --bin wininfo_cli -- --network
cargo run --bin wininfo_cli -- --network --only-enabled
```

## License

Licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT).
