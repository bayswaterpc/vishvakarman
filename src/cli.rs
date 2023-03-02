use crate::prepend_date;
use crate::utils::string_to_args;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Function {
    HelloWorld,
    PrependDate,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// enum for supported http request
    #[clap(value_enum, long, short, value_parser, default_value_t=Function::HelloWorld)]
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
        Ok(args) => anyhow::Ok(args),
        Err(err) => {
            err.print()?;
            read_commands()
        },
    }
}

pub fn run_cli() -> Result<()> {
    // if we want to read from executable invocation
    //let mut args = Args::parse();

    println!("Photorio: Enter function, run -h for help");
    let mut args = read_commands()?;
    while !args.quit {
        cli_execute(args).with_context(|| "command execution error".to_string())?;

        println!("Photorio: Run another command, enter '-q' to quit");
        args = read_commands()?;
    }
    anyhow::Ok(())
}

fn cli_execute(args: Args) -> Result<()> {
    match args.function {
        Function::HelloWorld => {
            println!("Hello World");
        }
        Function::PrependDate => {
            prepend_date::run_cli().with_context(|| "prepend_date execution error".to_string())?;
        }
    }
    anyhow::Ok(())
}
