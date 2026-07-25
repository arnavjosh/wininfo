use wmr::memory;

fn main() {
    //TODO: make constants and stuff for byte conversions (create a constatns file like in go projdefcts?)
    match memory() {
        Ok(mem) => {
            println!("{:#?}", mem);
            println!(
                "Total: {:.2} GB",
                mem.total_bytes as f64 / 1024.0 / 1024.0 / 1024.0
            );
            println!(
                "Available: {:.2} GB",
                mem.available_bytes as f64 / 1024.0 / 1024.0 / 1024.0
            );
            println!("Usage: {:.1}%", mem.usage_percent);
        }

        Err(e) => println!("{e}"),
    }
}
