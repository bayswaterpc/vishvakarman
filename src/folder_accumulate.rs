use crate::utils::string_to_args;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::{fs, fs::DirEntry, path::Path};
use chrono::prelude::{DateTime, Utc};
use fs_extra::file::{move_file, CopyOptions};

const UTILITY_FUNCTION_NAME: & str = "Folder Accumulate";

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_enum, long, short, value_parser, default_value = "./")]
    directory: String,

    /// set to true to quit
    #[clap(long, short, value_parser, default_value_t = false)]
    recursive: bool,

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
        },
    }
}

pub fn run_cli() -> Result<()> {
    // if we want to read from executable invocation
    //let mut args = Args::parse();
    println!("** {}: Move Files Into Created By Date Folders YYYY-MM-DD **", UTILITY_FUNCTION_NAME);
    println!("*** Enter directory and recursive options, run -h for more help ***");
    let mut args = read_commands()?;
    while !args.back {
        organize_files(args).with_context(|| "Move Files Into Created By Date Folders".to_string())?;
        println!("*** {}: Run again or enter '-b' to go back ***", UTILITY_FUNCTION_NAME);
        args = read_commands()?;
    }

    anyhow::Ok(())
}

fn organize_files(args: Args) -> Result<(), anyhow::Error> {
    let res = match args.recursive {
        true => recursive_directory_organize(&args),
        false => flat_directory_organize(&args),
    };
    match res {
      Ok(_) =>  println!("** Success!  Files in \"{}\" accumulated to their date wise folders.  **", args.directory),
      Err(err) => println!("** Error Making Folders Failed! {}", err),
    }
    Ok(())
}

fn get_created_at(dir_entry: &DirEntry) -> Result<String, anyhow::Error> {
  let created_at = dir_entry.path().metadata().unwrap().created().unwrap();
  let created_at: DateTime<Utc> = created_at.into();
  let created_at = created_at.format("%Y-%m-%d").to_string();

  anyhow::Ok(created_at)
}


fn flat_directory_organize(args: & Args) -> Result<(), anyhow::Error> {
  let options = CopyOptions::new();

  // Iterate through paths and move folders to date folders
  let paths = fs::read_dir(args.directory.clone()).unwrap();
  for dir_entry in paths.flatten() {

    if dir_entry.path().is_dir() {
      continue;
    }
    let file_name = String::from(dir_entry.path().file_name().unwrap().to_str().unwrap());
    // Mac Specific file elimination
    if file_name.as_str() == ".DS_Store" {
      continue;
    }

    let created_at = get_created_at(&dir_entry)?;

    let parent = String::from(dir_entry.path().parent().unwrap().as_os_str().to_str().unwrap());
    let new_parent = Path::new(&parent).join(created_at);

    if !new_parent.exists() {
      if let Err(err) = fs::create_dir(new_parent.clone()) {
        return Err(anyhow!("create_dir err: {}", err));
      }
    }

    let new_path = new_parent.join(&file_name);

    if let Err(err) =  move_file(dir_entry.path(), new_path, &options) {
      return Err(anyhow!("move_file err: {}", err));
    }

  }
  anyhow::Ok(())
}


fn recursive_directory_organize(args: & Args) -> Result<(), anyhow::Error>  {
  let options = CopyOptions::new();

  // Iterate through paths and move folders to date folders
  let paths = fs::read_dir(args.directory.clone()).unwrap();
  for dir_entry in paths.flatten() {
    if dir_entry.path().is_file() {
      continue;
    }

    let orig_dir_name = String::from(dir_entry.path().file_name().unwrap().to_str().unwrap());
    let parent = String::from(dir_entry.path().parent().unwrap().as_os_str().to_str().unwrap());

    let sub_dir_paths = fs::read_dir(dir_entry.path()).unwrap();
    for de in sub_dir_paths.flatten() {
      if de.path().is_dir() {
        continue;
      }

      let file_name = String::from(de.path().file_name().unwrap().to_str().unwrap());
      // Mac Specific file elimination
      if file_name.as_str() == ".DS_Store" {
        continue;
      }

      let created_at = get_created_at(&de)?;
      let new_parent = Path::new(&parent).join(format!("{} - {}", created_at, orig_dir_name));

      if !new_parent.exists() {
        if let Err(err) = fs::create_dir(new_parent.clone()) {
          return Err(anyhow!("create_dir err: {}", err));
        }
      }

      let new_path = new_parent.join(&file_name);
      if let Err(err) =  move_file(de.path(), new_path, &options) {
        return Err(anyhow!("move_file err: {}", err));
      }
    }
    if let Err(err) = fs::remove_dir(dir_entry.path()) {
      return Err(anyhow!("remove_dir err: {}", err));
    }
  }
  
  Ok(())
}
