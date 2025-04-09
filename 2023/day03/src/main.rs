use std::fs::read_to_string;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn digit_adjacent_symbol(grid: &[Vec<char>], y: usize, x: usize) -> bool {
    let start_y = if y == 0 { 0 } else { y - 1 };
    let start_x = if x == 0 { 0 } else { x - 1 };
    let end_y = (y + 1).min(grid.len() - 1);
    let end_x = (x + 1).min(grid[y].len() - 1);

    for line in grid.iter().take(end_y + 1).skip(start_y) {
        for c in line.iter().take(end_x + 1).skip(start_x) {
            if is_symbol(*c) {
                return true;
            }
        }
    }

    false
}

fn part1(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;

    for y in 0..grid.len() {
        let mut number_start = None;
        let mut adjacent_symbol = false;
        for x in 0..grid[y].len() {
            if grid[y][x].is_ascii_digit() {
                if digit_adjacent_symbol(grid, y, x) {
                    adjacent_symbol = true;
                }

                if number_start.is_none() {
                    number_start = Some(x);
                }
            } else if let Some(start) = number_start {
                if adjacent_symbol {
                    let number = grid[y][start..x]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    sum += number;
                }

                number_start = None;
                adjacent_symbol = false;
            }
        }

        if let Some(start) = number_start {
            if adjacent_symbol {
                let number = grid[y][start..grid[y].len()]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                sum += number;
            }
        }
    }

    sum
}

fn get_full_number(line: &[char], x: usize) -> usize {
    assert!(line[x].is_ascii_digit());

    let mut start = x;
    let mut end = x;

    while start > 0 && line[start - 1].is_ascii_digit() {
        start -= 1;
    }

    while end + 1 < line.len() && line[end + 1].is_ascii_digit() {
        end += 1;
    }

    line.iter()
        .take(end + 1)
        .skip(start)
        .collect::<String>()
        .parse()
        .unwrap()
}

fn left_right_numbers(grid: &[Vec<char>], y: usize, x: usize) -> Vec<usize> {
    let mut numbers = Vec::new();

    if grid[y][x].is_ascii_digit() {
        numbers.push(get_full_number(&grid[y], x));
    } else {
        if x > 0 && grid[y][x - 1].is_ascii_digit() {
            numbers.push(get_full_number(&grid[y], x - 1));
        }

        if x + 1 < grid[y].len() && grid[y][x + 1].is_ascii_digit() {
            numbers.push(get_full_number(&grid[y], x + 1));
        }
    }

    numbers
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '*' {
                let mut numbers = left_right_numbers(grid, y, x);

                if y > 0 {
                    for number in left_right_numbers(grid, y - 1, x) {
                        numbers.push(number);
                    }
                }

                if y + 1 < grid.len() {
                    for number in left_right_numbers(grid, y + 1, x) {
                        numbers.push(number);
                    }
                }

                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }

    sum
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let part1 = part1(&grid);
    let part2 = part2(&grid);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
