mod analyzer;

fn main() {
    let targets = analyzer::analyze("kachifile".to_string()).expect("Failed to analyze file");

    for target in targets {
        println!("Target: {}", target.target_name);
        println!("Files: {:?}", target.files);
        println!("Commands: {:?}", target.commands);
    }
}
