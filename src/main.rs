mod cli;
mod prepend_date;
mod utils;
use anyhow::{Context, Result};
use cli::run_cli;

fn main() -> Result<()> {
    run_cli().with_context(|| "run_cli error".to_string())?;
    Ok(())
}
