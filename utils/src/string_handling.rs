pub trait StringHandling {
    fn get_lines(&self) -> Vec<String>;
    fn get_chars_trimmed(&self) -> Vec<char>;
}

impl StringHandling for String {
    fn get_lines(&self) -> Vec<String> {
        self.lines().map(String::from).collect()
    }

    fn get_chars_trimmed(&self) -> Vec<char> {
        self.trim().chars().collect()
    }
}
