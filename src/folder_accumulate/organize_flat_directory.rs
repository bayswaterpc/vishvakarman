use super::Args;
use crate::constants::EXCLUDED_FILES;
use crate::utils::{get_accumulated_date, get_default_file_split_map};

use eyre::{eyre, Result};
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::{fs, path::Path};

pub fn organize_flat_directory(args: &Args) -> Result<()> {
    let file_map_split = get_default_file_split_map();
    // Iterate through paths and move folders to date folders
    let paths = fs::read_dir(args.directory.clone()).unwrap();
    for dir_entry in paths.flatten() {
        if dir_entry.path().is_dir() {
            continue;
        }
        let file_name = String::from(dir_entry.path().file_name().unwrap().to_str().unwrap());

        if EXCLUDED_FILES.contains(&file_name.as_str()) {
            continue;
        }

        let created_at = get_accumulated_date(&dir_entry, &args.accumulate_type)?;

        let parent = String::from(
            dir_entry
                .path()
                .parent()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
        );

        let new_parent = if args.file_type_split {
            let extension = String::from(
                dir_entry
                    .path()
                    .parent()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
            );
            let sub_dir = match file_map_split.get(&extension) {
                Some(subdir) => subdir.clone(),
                None => "misc".to_string(),
            };
            Path::new(&parent).join(sub_dir).join(created_at)
        } else {
            Path::new(&parent).join(created_at)
        };

        if !new_parent.exists() {
            if !args.silent {
                print!(
                    "\n*** Creating and moving folders into : {}",
                    new_parent.as_os_str().to_str().unwrap()
                );
                io::stdout().flush().unwrap();
            }
            if let Err(err) = fs::create_dir_all(new_parent.clone()) {
                return Err(eyre!("create_dir err: {}", err));
            }
        }

        let new_path = new_parent.join(&file_name);

        if let Err(err) = fs::rename(dir_entry.path(), new_path) {
            return Err(eyre!("rename file err: {}", err));
        }
        if !args.silent {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }
    Ok(())
}
