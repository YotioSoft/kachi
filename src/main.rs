mod run;
use std::fs;

fn main() {
    // kachifileの検索
    fs::read_dir("./")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_ascii_lowercase().to_str().unwrap() == "kachifile")
        .for_each(|entry| {
            // kachifile を実行
            if entry.file_type().is_ok() {
                let filepath = entry.path().to_str().unwrap().to_string();
                println!("kachifile: {}", filepath);
                let targets = run::analyze(filepath).unwrap();
                let target_name = "hello".to_string();
                let target = targets.into_iter().find(|t| t.target_name == target_name).expect("Failed to find target");
                run::run(target);
            }
        });
}
