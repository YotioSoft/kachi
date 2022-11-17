mod run;

fn main() {
    let targets = run::analyzer::analyze("kachifile".to_string()).expect("Failed to analyze file");

    let target_name = "hello".to_string();
    let target = targets.into_iter().find(|t| t.target_name == target_name).expect("Failed to find target");
    run::run(target);
}
