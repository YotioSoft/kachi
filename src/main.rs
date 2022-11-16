mod analyzer;

fn main() {
    analyzer::analyze("kachifile".to_string()).expect("Failed to analyze file");
}
