mod constants;
mod utils;
mod organize_recursive_directory;
mod organize_flat_directory;

use organize_flat_directory::organize_flat_directory;
use organize_recursive_directory::organize_recursive_directory;
use constants::UTILITY_FUNCTION_NAME;

use crate::utils::string_to_args;

use clap::Parser;
use eyre::{Result, WrapErr};

#[derive(clap::ValueEnum, Clone, Debug)]
enum AccumulateType {
    CreatedAtDate,
    ModifiedAtDate,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(value_enum, long, short, value_parser, default_value = "./")]
    directory: String,

    #[clap(value_enum, long, short, value_parser, default_value_t = AccumulateType::ModifiedAtDate)]
    accumulate_type: AccumulateType,

    #[clap(long, short, value_parser, default_value_t = false)]
    recursive: bool,

    #[clap(long, short, value_parser, default_value_t = false)]
    silent: bool,

    #[clap(long, short, value_parser, default_value_t = false)]
    file_type_split: bool,

    /// set to true to quit
    #[clap(long, short, value_parser, default_value_t = false)]
    back: bool,
}

fn read_commands() -> Result<Args> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let strings = string_to_args(&buffer);
    match Args::try_parse_from(strings.iter()) {
        Ok(args) => Ok(args),
        Err(err) => {
            err.print()?;
            read_commands()
        }
    }
}

pub fn run_cli() -> Result<()> {
    println!(
        "** {}: Move Files Into Created By Date Folders YYYY-MM-DD **",
        UTILITY_FUNCTION_NAME
    );
    println!("*** Enter directory and recursive options, run -h for more help ***");
    let mut args = read_commands()?;
    while !args.back {
        organize_files(args).wrap_err("Move Files Into Created By Date Folders".to_string())?;
        println!(
            "*** {}: Run again or enter '-b' to go back ***",
            UTILITY_FUNCTION_NAME
        );
        args = read_commands()?;
    }

    Ok(())
}

fn organize_files(args: Args) -> Result<()> {
    let res = match args.recursive {
        true => organize_recursive_directory(&args),
        false => organize_flat_directory(&args),
    };
    match res {
        Ok(_) => println!(
            "\n*** Success!  Files in \"{}\" accumulated to their date wise folders. ***",
            args.directory
        ),
        Err(err) => println!("*** Error Making Folders Failed! {}", err),
    }
    Ok(())
}