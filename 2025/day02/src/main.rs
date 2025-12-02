use std::{collections::HashSet, fs::read_to_string, io};

use utils::string_handling::StringHandling;

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content: Vec<_> = read_to_string(filename)?
        .get_lines()
        .iter()
        .flat_map(|line| {
            line.split(',')
                .filter(|splitted| !splitted.is_empty())
                .map(|splitted| {
                    let split: Vec<_> = splitted.split('-').collect();
                    let first: usize = split[0].parse().unwrap();
                    let second: usize = split[1].parse().unwrap();
                    first..=second
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let (part1, part2) = content
        .iter()
        .fold((0, 0), |(part1_sum, part2_sum), range| {
            let (part1, part2) = range
                .clone()
                .fold((0, 0), |(part1_sum, part2_sum), number| {
                    let str: Vec<_> = number.to_string().chars().collect();

                    let mut part1_invalid = false;
                    let mut part2_invalid = false;
                    for i in 1..str.len() {
                        if part1_invalid && part2_invalid {
                            break;
                        }

                        let part1: Vec<_> = str.chunks(i).collect();
                        let part2: HashSet<_> = part1.iter().collect();

                        if part1.len() == 2 && part1[0] == part1[1] {
                            part1_invalid = true;
                        }

                        if part2.len() == 1 {
                            part2_invalid = true;
                        }
                    }

                    let part1_sum = if part1_invalid {
                        part1_sum + number
                    } else {
                        part1_sum
                    };

                    let part2_sum = if part2_invalid {
                        part2_sum + number
                    } else {
                        part2_sum
                    };
                    (part1_sum, part2_sum)
                    // (part1_invalid, part2_invalid)
                    // false
                });
            (part1_sum + part1, part2_sum + part2)
            // .sum::<usize>()
        });

    println!("Part 1: {:?}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
