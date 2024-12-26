use std::{collections::HashMap, fs::read_to_string, io::Error};
use utils_2024::*;

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn calculation(number: usize, iterations: usize) -> (usize, Vec<isize>) {
    let mut secret = number;
    let mut numbers = Vec::new();
    for _ in 0..iterations {
        let value = secret * 64;
        secret = mix(secret, value);
        secret = prune(secret);
        let value = secret / 32;
        secret = mix(secret, value);
        secret = prune(secret);
        let value = secret * 2048;
        secret = mix(secret, value);
        secret = prune(secret);
        numbers.push((secret % 10) as isize);
    }
    (secret, numbers)
}

fn part2(numbers: Vec<isize>, cache: &mut HashMap<Vec<isize>, usize>) {
    let mut local_cache = HashMap::new();
    for (i, number) in numbers.iter().enumerate().skip(4) {
        let mut previous = numbers[i - 4..i].to_vec();
        for i in 1..previous.len() {
            previous[i - 1] = previous[i] - previous[i - 1];
        }
        previous[3] = *number - previous[3];
        if !local_cache.contains_key(&(*previous)) {
            local_cache.insert(previous.clone(), *number);
            cache
                .entry(previous.clone())
                .and_modify(|value| *value += *number as usize)
                .or_insert(*number as usize);
        }
    }
}

fn main() -> Result<(), Error> {
    let input: Vec<usize> = read_to_string("input.txt")?
        .get_lines()
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut part1 = 0;
    let mut global_cache = HashMap::new();

    for number in input {
        let (result, numbers) = calculation(number, 2000);
        part1 += result;
        part2(numbers, &mut global_cache);
    }
    println!("Part 1: {}", part1);

    let part2 = global_cache.iter().max_by_key(|&(_, value)| value).unwrap();
    println!("Part 2: {}", part2.1);

    Ok(())
}
