use std::{fs::read_to_string, io};
use utils_2024::*;

fn main() -> Result<(), io::Error> {
    let numbers = read_to_string("input.txt")?
        .get_lines()
        .numbers_in_column(b' ');

    assert_eq!(numbers.len(), 2, "There should be exactly two columns");
    assert_eq!(
        numbers[0].len(),
        numbers[1].len(),
        "Both columns should have the same amount of numbers"
    );

    let mut left = numbers[0].clone();
    let mut right = numbers[1].clone();
    left.sort();
    right.sort();

    let mut part1 = 0;
    let mut part2 = 0;
    for (left_value, right_value) in left.iter().zip(right.iter()) {
        part1 += left_value.abs_diff(*right_value);
        part2 += left_value * right.iter().filter(|&val| val == left_value).count() as i128;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
