use std::error::Error;
use std::fs::read_to_string;

pub trait StringVecHandling {
    fn lines_as_bytes(&self) -> Vec<Vec<u8>>;
    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i32>>;
    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i32>>;
}

impl StringVecHandling for Vec<String> {
    fn lines_as_bytes(&self) -> Vec<Vec<u8>> {
        self.into_iter()
            .map(|line| line.as_bytes().to_vec())
            .collect()
    }

    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i32>> {
        self.iter()
            .map(|line| {
                line.split(delimiter as char)
                    .filter_map(|item| item.trim().parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .filter(|vec| !vec.is_empty())
            .collect()
    }

    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i32>> {
        let mut columns: Vec<Vec<i32>> = Vec::new();

        for line in self {
            let numbers: Vec<i32> = line
                .split(delimiter as char)
                .filter_map(|item| item.trim().parse::<i32>().ok())
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
}

impl StringHandling for String {
    fn get_lines(&self) -> Vec<String> {
        self.lines().map(String::from).collect()
    }
}
