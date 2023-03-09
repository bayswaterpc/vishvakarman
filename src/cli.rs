use crate::folder_accumulate;
use crate::prepend_date;
use crate::utils::string_to_args;
use clap::Parser;
use eyre::{Result, WrapErr};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Function {
    HelloWorld,
    PrependDate,
    FolderAccumulate,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// enum for supported http request
    #[clap(value_enum, long, short, value_parser, default_value_t=Function::FolderAccumulate)]
    function: Function,

    /// set to true to quit
    #[clap(long, short, value_parser, default_value_t = false)]
    quit: bool,
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
    println!("* Vishvakarman: Enter function, run -h for help *");
    println!("** Default \"folder-accumulate\" **");
    let mut args = read_commands()?;
    while !args.quit {
        cli_execute(args).wrap_err("cli_execute error")?;

        println!("** Vishvakarman: Run another command, enter '-q' to quit **");
        args = read_commands()?;
    }
    Ok(())
}

fn cli_execute(args: Args) -> Result<()> {
    match args.function {
        Function::HelloWorld => {
            println!("Hello World");
        }
        Function::PrependDate => {
            prepend_date::run_cli().wrap_err("prepend date error")?;
        }
        Function::FolderAccumulate => {
            folder_accumulate::run_cli().wrap_err("folder accumulate error")?;
        }
    }
    Ok(())
}
