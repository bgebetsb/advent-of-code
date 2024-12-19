use std::{collections::HashMap, fs::read_to_string, io::Error};
use utils::*;

fn find_matches<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    let mut combinations = 0;

    if design.is_empty() {
        return 1;
    }

    for &pattern in patterns {
        if let Some(found) = cache.get(&(design, pattern)) {
            combinations += found;
        } else if design.len() >= pattern.len() && &design[0..pattern.len()] == pattern {
            let result = find_matches(&design[pattern.len()..], patterns, cache);
            cache.insert((design, pattern), result);
            combinations += result;
        }
    }
    combinations
}

fn calculate(designs: &[String], patterns: &[&str]) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut cache = HashMap::new();

    for design in designs {
        let result = find_matches(design, patterns, &mut cache);

        if result > 0 {
            part1 += 1;
            part2 += result;
        }
    }
    (part1, part2)
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?.get_lines();

    let patterns: Vec<&str> = input.first().unwrap().split(", ").collect();
    let designs: Vec<String> = input.iter().skip(2).map(String::from).collect();

    let (part1, part2) = calculate(&designs, &patterns);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
