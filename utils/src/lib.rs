pub trait StringVecHandling {
    fn lines_as_chars(&self) -> Vec<Vec<char>>;
    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i128>>;
    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i128>>;
}

impl StringVecHandling for Vec<String> {
    fn lines_as_chars(&self) -> Vec<Vec<char>> {
        self.iter().map(|line| line.chars().collect()).collect()
    }

    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i128>> {
        self.iter()
            .map(|line| {
                line.split(delimiter as char)
                    .filter_map(|item| item.trim().parse::<i128>().ok())
                    .collect::<Vec<i128>>()
            })
            .filter(|vec| !vec.is_empty())
            .collect()
    }

    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i128>> {
        let mut columns: Vec<Vec<i128>> = Vec::new();

        for line in self {
            let numbers: Vec<i128> = line
                .split(delimiter as char)
                .filter_map(|item| item.trim().parse::<i128>().ok())
                .collect();

            if columns.len() < numbers.len() {
                columns.resize_with(numbers.len(), Vec::new);
            }

            for (i, num) in numbers.into_iter().enumerate() {
                columns[i].push(num);
            }
        }

        columns
    }
}

pub trait StringHandling {
    fn get_lines(&self) -> Vec<String>;
    fn get_chars_without_nl(&self) -> Vec<char>;
}

impl StringHandling for String {
    fn get_lines(&self) -> Vec<String> {
        self.lines().map(String::from).collect()
    }

    fn get_chars_without_nl(&self) -> Vec<char> {
        self.chars().filter(|&c| c != '\n').collect()
    }
}
