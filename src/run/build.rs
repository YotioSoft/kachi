pub struct Target {
    pub target_name: String,
    pub files: Vec<String>,
    pub commands: Vec<String>,
}

pub fn build(targets: Vec<Target>) {
    for target in targets {
        println!("Target: {}", target.target_name);
    }
}
