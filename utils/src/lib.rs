pub trait StringVecHandling {
    fn lines_as_bytes(&self) -> Vec<Vec<u8>>;
    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i128>>;
    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i128>>;
}

impl StringVecHandling for Vec<String> {
    fn lines_as_bytes(&self) -> Vec<Vec<u8>> {
        self.iter().map(|line| line.as_bytes().to_vec()).collect()
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
}

impl StringHandling for String {
    fn get_lines(&self) -> Vec<String> {
        self.lines().map(String::from).collect()
    }
}
