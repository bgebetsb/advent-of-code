use std::{fs::read_to_string, io};

use utils::ranges::RangeHandling;
use utils::string_handling::StringHandling;

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines();

    let mut blank = false;
    let mut ranges = Vec::new();
    let mut part1 = 0;
    for line in content {
        if line.is_empty() {
            blank = true;
            continue;
        }

        if !blank {
            let parts: Vec<_> = line.split('-').collect();
            let range = parts[0].parse().unwrap()..=parts[1].parse::<u64>().unwrap();
            ranges.push(range);
        } else {
            let number: u64 = line.parse().unwrap();
            for range in ranges.iter() {
                if range.contains(&number) {
                    part1 += 1;
                    break;
                }
            }
        }
    }

    let part2_ranges = ranges.deduplicate_ranges();
    let part2: usize = part2_ranges.into_iter().map(|range| range.count()).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
