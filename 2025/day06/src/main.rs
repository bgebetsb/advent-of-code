use std::{fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn calc(numbers: &[Vec<i128>], operations: &[&str]) -> i128 {
    let mut total = 0;
    for (numbers, &op) in numbers.iter().zip(operations.iter()) {
        let mut sum = *numbers.first().unwrap();

        for number in numbers.iter().skip(1) {
            match op.chars().next().unwrap() {
                '*' => sum *= *number,
                '+' => sum += *number,
                _ => panic!(),
            }
        }

        total += sum;
    }

    total
}

fn calc_part2(input: &[Vec<char>]) -> i128 {
    let longest_line = input.iter().map(|line| line.len()).max().unwrap();
    let mut op = None;
    let mut numbers: Vec<i128> = Vec::new();
    let mut total = 0;

    for x in 0..longest_line {
        let mut nbr_str = String::new();
        for line in input {
            if x >= line.len() {
                continue;
            }

            if line[x].is_ascii_digit() {
                nbr_str.push(line[x]);
            } else if line[x] == '+' || line[x] == '*' {
                op = Some(line[x]);
            }
        }

        if !nbr_str.is_empty() {
            numbers.push(nbr_str.parse().unwrap());
        }

        if nbr_str.is_empty() || x + 1 == longest_line {
            total += match op {
                Some('+') => numbers
                    .iter()
                    .skip(1)
                    .fold(*numbers.first().unwrap(), |total, number| total + *number),
                Some('*') => numbers
                    .iter()
                    .skip(1)
                    .fold(*numbers.first().unwrap(), |total, number| total * *number),
                _ => panic!(),
            };
            numbers.clear();
            op = None;
        }
    }

    total
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines();
    let number_lines: Vec<_> = content
        .lines_as_chars()
        .iter()
        .filter(|line| line.iter().any(|c| c.is_ascii_digit()))
        .map(|line| line.iter().collect())
        .collect();

    let numbers_part1 = number_lines.numbers_in_column(b' ');
    let op_line = content
        .iter()
        .find(|line| line.contains('*') || line.contains('+'))
        .unwrap();
    let operations: Vec<_> = op_line.split_whitespace().collect();

    let part1 = calc(&numbers_part1, &operations);

    println!("Part 1: {}", part1);

    let part2 = calc_part2(&content.lines_as_chars());
    println!("Part 2: {}", part2);

    Ok(())
}
