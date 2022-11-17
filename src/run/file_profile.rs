extern crate chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use confy;
use confy::ConfyError;

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

pub fn add(workdir: String, file_modify: Vec<(String, DateTime<Utc>)>) -> Result<(), ConfyError> {
    let mut file_profile = confy::load::<FileProfiles>("kachi", "file_profile");
    if let Ok(file_profile) = &mut file_profile {
        let profiles = file_modify.into_iter().map(|(filepath, modify)| {
            FileModify {
                filepath,
                modify: modify.to_string(),
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
