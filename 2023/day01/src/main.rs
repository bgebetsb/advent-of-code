use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?;
    let lines = input.lines();

    let part1: usize = lines
        .clone()
        .map(|line| {
            let mut filtered = line.chars().filter(|c| c.is_ascii_digit());
            let first = filtered.next().unwrap();
            let last = filtered.next_back().unwrap_or(first);
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum();

    println!("Part 1: {}", part1);

    let part2: usize = lines
        .map(|line| {
            let mut line = line.to_string();

            let replacements = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            for (i, item) in replacements.iter().enumerate() {
                let mut chars = item.chars();
                let replacement = format!(
                    "{}{}{}",
                    chars.next().unwrap(),
                    i + 1,
                    chars.last().unwrap()
                );
                line = line.replace(replacements[i], &replacement);
            }

            let mut filtered = line.chars().filter(|c| c.is_ascii_digit());
            let first = filtered.next().unwrap();
            let last = filtered.next_back().unwrap_or(first);
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum();

    println!("Part 2: {}", part2);

    Ok(())
}
