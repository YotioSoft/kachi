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
}
