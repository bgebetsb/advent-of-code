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
                _ => unimplemented!(),
            }
        }

        total += sum;
    }

    total
}

fn parsing_hell_pt2(number_input: &[Vec<char>], op_line: &str) -> Vec<Vec<i128>> {
    let op_pos: Vec<usize> = op_line
        .chars()
        .enumerate()
        .filter_map(|(pos, c)| {
            if c == '+' || c == '*' {
                Some(pos)
            } else {
                None
            }
        })
        .collect();

    let mut cur_pos = number_input.iter().map(|line| line.len()).max().unwrap() - 1;
    let mut new_numbers = Vec::new();

    for op_pos in op_pos.iter().rev() {
        let mut numbers: Vec<i128> = Vec::new();

        while cur_pos >= *op_pos {
            let nbr_str: String = number_input
                .iter()
                .filter_map(|line| {
                    if line[cur_pos].is_ascii_digit() {
                        Some(line[cur_pos])
                    } else {
                        None
                    }
                })
                .collect();

            numbers.push(nbr_str.parse().unwrap());
            if cur_pos == 0 {
                break;
            }
            cur_pos -= 1;
        }

        cur_pos = cur_pos.saturating_sub(1);
        new_numbers.push(numbers);
    }

    new_numbers.reverse();

    new_numbers
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

    let numbers_part2 = parsing_hell_pt2(&number_lines.lines_as_chars(), op_line);
    let part2 = calc(&numbers_part2, &operations);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
