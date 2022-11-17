mod run;

fn main() {
    let targets = run::analyzer::analyze("kachifile".to_string()).expect("Failed to analyze file");

    run::run(targets);
}
