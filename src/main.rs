mod cli;
mod folder_accumulate;
mod prepend_date;
mod utils;
use cli::run_cli;
use eyre::{Result, WrapErr};

fn main() -> Result<()> {
    run_cli().wrap_err("Run CLI error")?;
    Ok(())
}
