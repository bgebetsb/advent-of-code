use std::char;
use std::fs::read_to_string;
use std::io;
use utils::*;

fn search_path(input: &mut [Vec<char>], y: usize, x: usize, value: char, part1: bool) -> u32 {
    if value == '9' {
        if part1 {
            input[y][x] = '0';
        }
        return 1;
    }

    let expected = char::from_digit(value.to_digit(10).unwrap() + 1, 10).unwrap();

    let mut total = 0;
    if x > 0 && input[y][x - 1] == expected {
        total += search_path(input, y, x - 1, expected, part1);
    }

    if y > 0 && input[y - 1][x] == expected {
        total += search_path(input, y - 1, x, expected, part1);
    }

    if x < input[0].len() - 1 && input[y][x + 1] == expected {
        total += search_path(input, y, x + 1, expected, part1);
    }

    if y < input.len() - 1 && input[y + 1][x] == expected {
        total += search_path(input, y + 1, x, expected, part1);
    }

    total
}

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?.get_lines().lines_as_chars();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '0' {
                part1 += search_path(&mut input.clone(), i, j, '0', true);
                part2 += search_path(&mut input.clone(), i, j, '0', false);
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
