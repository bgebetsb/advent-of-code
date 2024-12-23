use std::fs::read_to_string;
use std::io::Error;

use utils::*;
mod keypad;
mod robots;
use crate::keypad::*;
use crate::robots::*;

fn create_robots(amount: usize) -> Vec<Robot> {
    let mut robots = Vec::new();
    robots.push(Robot::new(KeypadType::Numeric));
    for _ in 0..amount {
        robots.push(Robot::new(KeypadType::Directional));
    }
    robots
}

fn simulation(robots: &mut Vec<Robot>, index: usize, input: &str) -> usize {
    let mut total = 0;
    for c in input.chars() {
        if let Some(value) = robots[index].search_cached_value(c) {
            total += value;
            continue;
        }
        let path = &robots[index].find_paths(c);
        let result = match index != robots.len() - 1 {
            true => simulation(robots, index + 1, &path.0),
            false => path.0.len(),
        };
        total += result;
        robots[index].insert_in_cache(c, result, path.1, path.2);
    }
    total
}

fn spawn_robots(input: &str, robots: usize) -> usize {
    let value = input[0..=2].parse::<usize>().unwrap();

    let mut robots = create_robots(robots);
    let result = simulation(&mut robots, 0, input);
    result * value
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?.get_lines();

    let part1: usize = input.iter().map(|line| spawn_robots(line, 2)).sum();
    println!("Part 1: {}", part1);

    let part2: usize = input.iter().map(|line| spawn_robots(line, 25)).sum();
    println!("Part 2: {}", part2);

    Ok(())
}
