use std::{collections::HashMap, fs::read_to_string, io::Error};

fn find_matches<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    } else if let Some(value) = cache.get(&design) {
        return *value;
    }

    let combinations = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| find_matches(&design[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design, combinations);
    combinations
}

fn calculate(designs: &[&str], patterns: &[&str]) -> (usize, usize) {
    let mut cache = HashMap::new();

    designs
        .iter()
        .map(|design| find_matches(design, patterns, &mut cache))
        .filter(|result| *result != 0)
        .fold((0, 0), |old, new| (old.0 + 1, old.1 + new))
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?;
    let mut lines = input.lines();

    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = lines.skip(1).collect();

    let (part1, part2) = calculate(&designs, &patterns);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
