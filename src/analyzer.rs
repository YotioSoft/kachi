use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Target {
    pub target_name: String,
    pub files: Vec<String>,
    pub commands: Vec<String>,
}
impl Target {
    pub fn default() -> Self {
        Target {
            target_name: String::new(),
            files: Vec::new(),
            commands: Vec::new(),
        }
    }
    pub fn clone(&self) -> Target {
        Target {
            target_name: self.target_name.clone(),
            files: self.files.clone(),
            commands: self.commands.clone(),
        }
    }
}

pub fn analyze(filepath: String) -> Result<Vec<Target>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut targets = Vec::new();
    for line in contents.lines() {
        match line {
            line if line.contains(":") => {
                let mut split = line.split(":");
                let mut target = Target::default();
                target.target_name = split.next().unwrap().to_string();
                target.files = split.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
                targets.push(target);
            },
            line if line.contains('\t') || line.contains("    ") => {
                let mut split = line.split("\t");
                split.next();
                let mut commands = split.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
                targets.last_mut().unwrap().commands.append(&mut commands);
                
            },
            _ => {
                println!("Invalid line: {}", line);
            },
        }
    }

    Ok(targets)
}
