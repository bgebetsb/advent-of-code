use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

fn split_number(nbr: &str) -> (u64, u64) {
    let left = nbr[0..nbr.len() / 2].parse::<u64>().unwrap();
    let right = nbr[nbr.len() / 2..].parse::<u64>().unwrap();

    (left, right)
}

fn calculate(
    input: u64,
    remaining_iterations: usize,
    cache: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if remaining_iterations == 0 {
        return 1;
    }

    if let Some(cachedvalue) = cache.get(&(input, remaining_iterations)) {
        return *cachedvalue;
    }

    let strval = input.to_string();

    match (input, strval.len() % 2) {
        (0, _) => calculate(1, remaining_iterations - 1, cache),
        (_, 0) => {
            let (left, right) = split_number(&strval);
            let result = calculate(left, remaining_iterations - 1, cache)
                + calculate(right, remaining_iterations - 1, cache);
            cache.insert((input, remaining_iterations), result);
            result
        }
        (_, _) => {
            let result = calculate(input * 2024, remaining_iterations - 1, cache);
            cache.insert((input, remaining_iterations), result);
            result
        }
    }
}

fn main() -> Result<(), io::Error> {
    let input: Vec<u64> = read_to_string("input.txt")?
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect();

    let mut cached_results = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 0;
    for number in input {
        part1 += calculate(number, 25, &mut cached_results);
        part2 += calculate(number, 75, &mut cached_results);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
