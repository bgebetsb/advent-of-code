use std::{collections::VecDeque, fs::read_to_string, io};

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
            let range: std::ops::RangeInclusive<u64> =
                parts[0].parse().unwrap()..=parts[1].parse().unwrap();
            ranges.push(range);
        } else {
            let number: u64 = line.parse().unwrap();
            for range in ranges.clone().into_iter() {
                if range.contains(&number) {
                    part1 += 1;
                    break;
                }
            }
        }
    }

    let mut part2_ranges = Vec::new();
    let mut part2 = 0;
    for range in ranges.into_iter() {
        let mut cur_range = VecDeque::new();
        cur_range.push_back(range.clone());

        'outer: while let Some(value) = cur_range.pop_front() {
            for part2range in part2_ranges.clone().into_iter() {
                if value == part2range
                    || value.start() >= part2range.start() && value.end() <= part2range.end()
                {
                    continue 'outer;
                }

                if value.start() >= part2range.start()
                    && value.end() >= part2range.end()
                    && value.start() <= part2range.end()
                {
                    cur_range.push_back(part2range.end() + 1..=*value.end());
                    continue 'outer;
                }
                if value.start() <= part2range.start() && value.end() > part2range.end() {
                    cur_range.push_back(*value.start()..=part2range.start() - 1);
                    cur_range.push_back(*part2range.end() + 1..=*value.end());
                    continue 'outer;
                }
                if value.start() <= part2range.start() && value.end() >= part2range.start() {
                    cur_range.push_back(*value.start()..=part2range.start() - 1);
                    continue 'outer;
                }
            }

            part2_ranges.push(value.clone());
            part2 += value.count();
        }
    }

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
