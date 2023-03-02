mod cli;
mod prepend_date;
mod utils;
mod folder_accumulate;
use anyhow::{Context, Result};
use cli::run_cli;

fn main() -> Result<(), anyhow::Error> {
    run_cli().with_context(|| "run_cli error".to_string())?;
    Ok(())
}
