extern crate chrono;
use chrono::prelude::*;
use std::fs::{File, Metadata};
pub mod analyzer;
mod file_profile;

fn get_file_metadata(filepaths: Vec<String>) -> Result<Vec<file_profile::FileMetadata>, std::io::Error> {
    let mut metadata: Vec<file_profile::FileMetadata> = Vec::new();
    for filepath in filepaths {
        let file = File::open(&filepath).unwrap();
        metadata.push(file_profile::FileMetadata {
            filepath: filepath,
            metadata: file.metadata()?,
        });
    }
    Ok(metadata)
}

pub fn run(target: analyzer::Target) {
    let file_metadata = get_file_metadata(target.files);
    if let Ok(file_metadata) = file_metadata {
        for file in file_metadata.clone() {
            println!("File modified: {}", file_profile::to_datetime(file.metadata));
        }

        file_profile::add("workdir".to_string(), file_metadata).expect("Failed to add file profile");
    }
}
