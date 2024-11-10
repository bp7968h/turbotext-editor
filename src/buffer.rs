pub struct Buffer {
    file: Option<String>,
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(file: Option<String>) -> Self {
        let lines = match &file {
            Some(file) => std::fs::read_to_string(file)
                .unwrap()
                .lines()
                .map(|s| s.to_string())
                .collect(),
            None => Vec::new(),
        };

        Self { file, lines }
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_file(&self) -> &Option<String> {
        &self.file
    }
}