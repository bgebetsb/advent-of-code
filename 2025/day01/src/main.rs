use std::{fs::read_to_string, io};

use utils::string_handling::StringHandling;

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines();
    let mut current = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    for line in content.iter() {
        if line.is_empty() {
            continue;
        }

        let lower = line.starts_with('L');
        let amount: i32 = line[1..].parse().unwrap();

        if current == 0 && lower {
            part2 -= 1;
        }

        if lower {
            current -= amount;
        } else {
            current += amount;
        }

        while current < 0 {
            current += 100;
            part2 += 1;
        }

        while current > 99 {
            current -= 100;
            if current != 0 {
                part2 += 1;
            }
        }

        if current == 0 {
            part1 += 1;
            part2 += 1;
        }
    }

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
