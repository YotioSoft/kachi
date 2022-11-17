pub mod analyzer;

pub fn run(targets: Vec<analyzer::Target>) {
    for target in targets {
        println!("Target: {}", target.target_name);
        println!("Files: {:?}", target.files);
        println!("Commands: {:?}", target.commands);
    }
}
