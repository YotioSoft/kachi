use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn analyze(filepath: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}
