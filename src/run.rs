use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
mod file_profile;
mod target;

pub fn analyze(filepath: String) -> Result<Vec<target::Target>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut targets = Vec::new();
    {
        for line in contents.lines() {
            if line.contains(":") {
                let mut split = line.split(":");
                let mut target = target::Target::default();
                target.target_name = split.next().unwrap().to_string();
                target.files = split.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
                targets.push(target);
            }
            if line.contains('\t') || line.contains("    ") {
                let mut split = if line.contains('\t') {
                    line.split("\t")
                } else {
                    line.split("    ")
                };
                split.next();
                let command = split.next().unwrap().to_string();
                targets.last_mut().unwrap().commands.push(command);
                
            }
        }
    }

    Ok(targets)
}

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

pub fn run(target: target::Target) {
    let file_metadata = get_file_metadata(target.files);
    if let Ok(file_metadata) = file_metadata {
        for file in file_metadata.clone() {
            println!("File modified: {}", file_profile::to_datetime(file.metadata));
        }

        file_profile::add("workdir".to_string(), file_metadata).expect("Failed to add file profile");
    }
}
