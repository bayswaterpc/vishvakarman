use super::utils::{ get_default_file_split_map, get_accumulated_date};
use super::Args;
use super::constants::EXCLUDED_FILES;

use std::collections::{HashSet};

use eyre::{eyre, Result};
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::{fs, path::Path};

pub fn organize_recursive_directory(args: &Args) -> Result<()> {
    // TODO make file map splitting work in recursive case
    let _file_map_split = get_default_file_split_map();

    let mut visited_paths = HashSet::new();
    // Iterate through paths and move folders to date folders
    let paths = fs::read_dir(args.directory.clone()).unwrap();
    for dir_entry in paths.flatten() {
        if dir_entry.path().is_file() {
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
        let orig_dir_name = String::from(dir_entry.path().file_name().unwrap().to_str().unwrap());
        if visited_paths.contains(&orig_dir_name) {
            continue;
        }
        if !args.silent {
            print!(
                "\n*** Creating and Moving directories for : {}",
                orig_dir_name
            );
            io::stdout().flush().unwrap();
        }

        let sub_dir_paths = fs::read_dir(dir_entry.path()).unwrap();
        for de in sub_dir_paths.flatten() {
            if de.path().is_dir() {
                continue;
            }

            let file_name = String::from(de.path().file_name().unwrap().to_str().unwrap());
            // Mac Specific file elimination
            if EXCLUDED_FILES.contains(&file_name.as_str()) {
                continue;
            }

            let created_at = get_accumulated_date(&de, args)?;
            let new_dir_name = format!("{} {}", created_at, orig_dir_name);
            visited_paths.insert(new_dir_name.clone());
            let new_parent = Path::new(&parent).join(new_dir_name);
            if !new_parent.exists() {
                if let Err(err) = fs::create_dir(new_parent.clone()) {
                    return Err(eyre!("create_dir err: {}", err));
                }
            }

            let new_path = new_parent.join(&file_name);
            if let Err(err) = fs::rename(de.path(), new_path) {
                return Err(eyre!("rename file err: {}", err));
            }
            if !args.silent {
                print!(".");
                io::stdout().flush().unwrap();
            }
        }
        if let Err(err) = fs::remove_dir(dir_entry.path()) {
            return Err(eyre!("remove_dir err: {}", err));
        }
    }

    Ok(())
}
