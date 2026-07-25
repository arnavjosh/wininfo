use wmr::memory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mem = memory()?;

    println!("Total: {:.2} GiB", mem.total_gib());
    println!("Available: {:.2} GiB", mem.available_gib());
    println!("Used: {:.2} GiB", mem.used_gib());

    Ok(())
}
