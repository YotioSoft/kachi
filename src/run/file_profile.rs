extern crate chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use confy;
use confy::ConfyError;

pub struct FileMetadata {
    pub filepath: String,
    pub metadata: Metadata,
}
impl Clone for FileMetadata {
    fn clone(&self) -> Self {
        FileMetadata {
            filepath: self.filepath.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FileModify {
    pub filepath: String,
    pub modify: String,
}

#[derive(Serialize, Deserialize)]
struct FileProfile {
    pub workdir: String,
    pub files: Vec<FileModify>,
}

#[derive(Serialize, Deserialize)]
struct FileProfiles {
    pub profiles: Vec<FileProfile>,
}
impl Default for FileProfiles {
    fn default() -> Self {
        FileProfiles {
            profiles: Vec::new(),
        }
    }
}

pub fn to_datetime(metadata: Metadata) -> chrono::DateTime<chrono::Local> {
    let datetime: DateTime<Utc> = chrono::DateTime::from(metadata.modified().unwrap());
    datetime.with_timezone(&chrono::Local)
}

pub fn add(workdir: String, file_modify: Vec<FileMetadata>) -> Result<(), ConfyError> {
    let mut file_profile = confy::load::<FileProfiles>("kachi", "file_profile");
    if let Ok(file_profile) = &mut file_profile {
        let profiles = file_modify.into_iter().map(|file_meta_data| {
            FileModify {
                filepath: file_meta_data.filepath,
                modify: to_datetime(file_meta_data.metadata).to_string(),
            }
        }).collect();

        file_profile.profiles.push(FileProfile {
            workdir: workdir,
            files: profiles,
        });

        confy::store("kachi", "file_profile", file_profile)?;
    }
    Ok(())
}
