extern crate chrono;
use chrono::prelude::*;
use std::fs::{File, Metadata};
pub mod analyzer;
mod file_profile;

fn to_datetime(metadata: Metadata) -> chrono::DateTime<chrono::Local> {
    let datetime: DateTime<Utc> = chrono::DateTime::from(metadata.modified().unwrap());
    datetime.with_timezone(&chrono::Local)
}

fn get_file_metadata(filepaths: Vec<String>) -> Result<Vec<(String, Metadata)>, std::io::Error> {
    let mut metadata: Vec<(String, Metadata)> = Vec::new();
    for filepath in filepaths {
        let file = File::open(filepath).unwrap();
        metadata.push((filepath, file.metadata()?));
    }
    Ok(metadata)
}

pub fn run(target: analyzer::Target) {
    let metadata = get_file_metadata(target.files);
    if let Ok(metadata) = metadata {
        for file in metadata {
            println!("File modified: {}", to_datetime(file.1));
        }

        let file_modify = metadata.into_iter().map(|file| {
            (file.0, to_datetime(file.1))
        }).collect();
        file_profile::add(std::env::current_dir().unwrap().to_str().unwrap().to_string(), file_modify);
    }
}
