mod cli;
mod prepend_date;
mod utils;
mod folder_accumulate;
use eyre::{Result, WrapErr};
use cli::run_cli;

fn main() -> Result<()> {
    run_cli().wrap_err("Run CLI error")?;
    Ok(())
}
