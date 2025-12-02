use std::{fs::read_to_string, io};

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

                    for i in (1..=str.len() / 2).rev() {
                        if str.len() % i != 0 {
                            continue;
                        }

                        let mut chunks = str.chunks(i);
                        let first = chunks.next().unwrap();

                        let part2_invalid = chunks.all(|chunk| chunk == first);

                        if part2_invalid {
                            if i * 2 == str.len() {
                                return (part1_sum + number, part2_sum + number);
                            }
                            return (part1_sum, part2_sum + number);
                        }
                    }

                    (part1_sum, part2_sum)
                });
            (part1_sum + part1, part2_sum + part2)
        });

    println!("Part 1: {:?}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
