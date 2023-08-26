use crate::constants::EXCLUDED_FILES;
use crate::utils::string_to_args;
use crate::utils::{get_accumulated_date, AccumulateType};
use clap::Parser;
use eyre::{Result, WrapErr};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Target {
    Files,
    Directories,
    All,
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
#[derive(Parser, Debug)]
struct Args {
    #[clap(value_enum, long, short, value_parser, default_value_t=Target::Directories)]
    target: Target,

    #[clap(value_enum, long, short, value_parser, default_value = "./")]
    directory: String,

    #[clap(value_enum, long, short, value_parser, default_value_t = AccumulateType::Created)]
    accumulate_type: AccumulateType,

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
    println!("Current working directory is: {:?}", get_current_working_dir()?);
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
        Target::All => "all",
    };
    println!("Target is : {}", target);
    let paths = fs::read_dir(args.directory.clone()).unwrap();
    let mut visited_paths = HashSet::new();
    for dir_entry in paths.flatten() {
        let is_target = match args.target {
            Target::Files => !dir_entry.path().is_dir(),
            Target::Directories => dir_entry.path().is_dir(),
            Target::All => true,
        };
        if !is_target {
            continue;
        }
        let dir_entry_name = String::from(dir_entry.path().file_name().unwrap().to_str().unwrap());
        if visited_paths.contains(&dir_entry_name)
            || EXCLUDED_FILES.contains(&dir_entry_name.as_str())
        {
            continue;
        }


        let parent = String::from(
            dir_entry
                .path()
                .parent()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
        );

        let created_at = get_accumulated_date(&dir_entry, &args.accumulate_type)?;
        let new_dir_name = format!("{} - {}", created_at, dir_entry_name);
        visited_paths.insert(new_dir_name.clone());
        let new_path = Path::new(&parent).join(&new_dir_name);
        std::fs::rename(dir_entry.path(), new_path).expect("Should have successfully renamed")
    }
    Ok(())
}
