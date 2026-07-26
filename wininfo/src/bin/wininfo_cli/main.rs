use byte_unit::{Byte, Unit};
use wmr::System;

fn main() -> Result<(), wmr::WmrError> {
    let system = System::new()?;

    let info = system.info()?;
    println!("{:?}", info);
    let memory = system.memory()?;
    println!(
        "Total:     {:.2}",
        memory.total().get_adjusted_unit(Unit::GiB)
    );
    println!(
        "Free: {:.2} GiB",
        memory.free().get_adjusted_unit(Unit::GiB)
    );
    let cpu = system.cpu()?;
    println!();
    println!("=== CPU ===");
    println!("Name:         {}", cpu.name());
    println!("Manufacturer: {}", cpu.manufacturer());
    if let Some(cores) = cpu.physical_cores() {
        println!("Physical cores: {}", cores);
    }
    if let Some(threads) = cpu.logical_cores() {
        println!("Logical processors: {}", threads);
    }
    if let Some(speed) = cpu.max_clock_speed_ghz() {
        println!("Max clock speed: {:.2} GHz", speed);
    }

    let disk = system.disk()?;
    //println!("Name: {}", disk.device_id());
    println!(
        "Size: {:.2}",
        disk.size()
            .unwrap_or(Byte::from_u64(0))
            .get_adjusted_unit(Unit::GiB)
    );
    println!(
        "Free: {:.2}",
        disk.free()
            .unwrap_or(Byte::from_u64(0))
            .get_adjusted_unit(Unit::GiB)
    );
    Ok(())
}
