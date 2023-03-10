use crate::utils::string_to_args;
use clap::Parser;
use eyre::{Result, WrapErr};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Target {
    Files,
    Directories,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_enum, long, short, value_parser, default_value_t=Target::Directories)]
    target: Target,

    #[clap(value_enum, long, short, value_parser, default_value = "")]
    directory: String,

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
    println!("Prepend Date: Enter target and directory options, run -h for more help");
    let mut args = read_commands()?;
    while !args.back {
        prepend_date(args).wrap_err("Prepend date error")?;
        println!("Prepend Date: Run again or enter '-b' to go back");
        args = read_commands()?;
    }
    Ok(())
}

fn prepend_date(args: Args) -> Result<()> {
    let target = match args.target {
        Target::Files => "files",
        Target::Directories => "directories",
    };
    println!("Target is : {}", target);
    Ok(())
}
