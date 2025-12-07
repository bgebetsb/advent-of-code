pub trait StringHandling {
    fn get_lines(&self) -> Vec<String>;
    fn get_chars_trimmed(&self) -> Vec<char>;
}

impl StringHandling for String {
    fn get_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = self.lines().map(String::from).collect();

        while lines.iter().next_back().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }

        lines
    }

    fn get_chars_trimmed(&self) -> Vec<char> {
        self.trim().chars().collect()
    }
}
