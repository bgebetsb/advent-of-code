use std::str::FromStr;

pub trait StringVecHandling {
    fn lines_as_chars(&self) -> Vec<Vec<char>>;
    fn digits_grid(&self) -> Vec<Vec<u8>>;
    fn numbers_in_line(&self, delimiter: u8) -> Vec<Vec<i128>>;
    fn numbers_in_column(&self, delimiter: u8) -> Vec<Vec<i128>>;
    fn split_with_key<K, V>(&self, delimiter: char) -> Vec<(K, Vec<V>)>
    where
        K: FromStr,
        V: FromStr;
}

impl StringVecHandling for Vec<String> {
    fn lines_as_chars(&self) -> Vec<Vec<char>> {
        self.iter().map(|line| line.chars().collect()).collect()
    }

    fn digits_grid(&self) -> Vec<Vec<u8>> {
        self.iter()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(|item| item.chars().map(|c| c.to_digit(10).unwrap() as u8))
                    .collect()
            })
            .collect()
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

    fn split_with_key<K, V>(&self, delimiter: char) -> Vec<(K, Vec<V>)>
    where
        K: FromStr,
        V: FromStr,
    {
        let mut items = Vec::new();

        for line in self {
            let parts: Vec<&str> = line.split(delimiter).collect();

            if parts.len() != 2 {
                continue;
            }

            let key = parts[0]
                .trim()
                .parse::<K>()
                .ok()
                .expect("Could not parse key");

            let values: Vec<V> = parts[1]
                .split_whitespace()
                .filter_map(|v| v.parse::<V>().ok())
                .collect();

            items.push((key, values));
        }

        items
    }
}

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

#[derive(Debug)]
pub enum Errors {
    OutOfBoundsError,
    IntegerOverflow,
}

pub trait UsizeOffset {
    fn offset(&self, value: isize) -> Result<Self, Errors>
    where
        Self: Sized;
}

impl UsizeOffset for usize {
    fn offset(&self, value: isize) -> Result<Self, Errors>
    where
        Self: Sized,
    {
        if value < 0 {
            let abs_offset = value.unsigned_abs();
            if abs_offset > *self {
                return Err(Errors::OutOfBoundsError);
            }
            Ok(*self - abs_offset)
        } else {
            self.checked_add(value as usize)
                .ok_or(Errors::IntegerOverflow)
        }
    }
}
