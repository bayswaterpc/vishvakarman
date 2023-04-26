use super::{AccumulateType, Args};
use chrono::prelude::{DateTime, Utc};
use eyre::Result;
use std::collections::HashMap;
use std::fs::DirEntry;

pub fn get_accumulated_date(dir_entry: &DirEntry, args: &Args) -> Result<String> {
    let created_at = match args.accumulate_type {
        AccumulateType::CreatedAtDate => dir_entry.path().metadata().unwrap().created().unwrap(),
        AccumulateType::ModifiedAtDate => dir_entry.path().metadata().unwrap().modified().unwrap(),
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
