extern crate chrono;
use chrono::prelude::*;
use std::fs::{File, Metadata};
pub mod analyzer;

fn to_datetime(metadata: Metadata) -> chrono::DateTime<chrono::Local> {
    let datetime: DateTime<Utc> = chrono::DateTime::from(metadata.modified().unwrap());
    datetime.with_timezone(&chrono::Local)
}

fn get_file_metadata(filepaths: Vec<String>) -> Result<Vec<Metadata>, std::io::Error> {
    let mut metadata: Vec<Metadata> = Vec::new();
    for filepath in filepaths {
        let file = File::open(filepath).unwrap();
        metadata.push(file.metadata()?);
    }
    Ok(metadata)
}

pub fn run(target: analyzer::Target) {
    let metadata = get_file_metadata(target.files);
    if let Ok(metadata) = metadata {
        for file in metadata {
            println!("File modified: {}", to_datetime(file));
        }
    }
}
