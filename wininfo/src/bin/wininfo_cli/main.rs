#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::assertions_on_result_states
    )
)]

use std::error::Error;
use wmr::System;

fn main() -> Result<(), Box<dyn Error>> {
    let system = System::new()?;

    let info = system.info()?;

    println!("{:}", serde_json::to_string_pretty(&info)?);
    Ok(())
}
