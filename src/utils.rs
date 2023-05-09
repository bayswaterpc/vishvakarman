use chrono::prelude::{DateTime, Utc};
use eyre::Result;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::DirEntry;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AccumulateType {
    Created,
    Modified,
}

pub fn get_accumulated_date(
    dir_entry: &DirEntry,
    accumulate_type: &AccumulateType,
) -> Result<String> {
    let created_at = match accumulate_type {
        AccumulateType::Created => dir_entry.path().metadata().unwrap().created().unwrap(),
        AccumulateType::Modified => dir_entry.path().metadata().unwrap().modified().unwrap(),
    };
    let created_at: DateTime<Utc> = created_at.into();
    let created_at = created_at.format("%Y-%m-%d").to_string();

    Ok(created_at)
}

pub fn get_default_file_labels() -> Vec<(String, Vec<String>)> {
    let video_files = vec![".mp4", ".srt", ".xml", ".mov", "insv"];
    let image_files = vec![".png", ".jpg", ".jpeg", ".arw", ".bmp", ".tiff"];

    vec![
        (
            "video".to_string(),
            video_files.iter().map(|f| f.to_string()).collect(),
        ),
        (
            "image".to_string(),
            image_files.iter().map(|f| f.to_string()).collect(),
        ),
    ]
}

pub fn get_default_file_split_map() -> HashMap<String, String> {
    let default_file_labels = get_default_file_labels();
    let mut file_map_split: HashMap<String, String> = HashMap::new();
    default_file_labels.iter().for_each(|(lbl, files)| {
        files.iter().for_each(|f| {
            file_map_split.insert(lbl.clone(), f.clone());
        })
    });
    file_map_split
}

pub fn string_to_args(string: &str) -> Vec<OsString> {
    // TODO: add handling of whitespace characters in quotes and character escaping
    let mut args = vec![OsString::from("vishvakarman")];
    for arg in string.split_whitespace() {
        args.push(arg.into());
    }
    args
}
