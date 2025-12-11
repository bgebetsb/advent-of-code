mod machine;

use std::{collections::VecDeque, fs::read_to_string, io};

use utils::string_handling::StringHandling;

use crate::machine::Machine;

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content: Vec<_> = read_to_string(filename)?.get_lines();

    let machines: Vec<_> = content
        .into_iter()
        .map(|line| Machine::try_from(line).unwrap())
        .collect();

    println!("This might take a few seconds...");
    let part1: usize = machines.iter().map(|machine| part1(machine)).sum();

    println!("Part 1: {}", part1);

    Ok(())
}

fn part1(machine: &Machine) -> usize {
    let initial_indicators: Vec<_> = (0..machine.indicator_lights.len()).map(|_| false).collect();
    let mut queue = VecDeque::new();

    for button in &machine.buttons {
        let mut current_indicators = initial_indicators.clone();
        for indicator in button {
            current_indicators[*indicator] = true;
        }
        queue.push_back((current_indicators, 1));
    }

    while let Some((indicators, count)) = queue.pop_front() {
        if indicators == machine.indicator_lights {
            return count;
        }

        for button in &machine.buttons {
            let mut current_indicators = indicators.clone();
            for indicator in button {
                current_indicators[*indicator] = !current_indicators[*indicator];
            }
            queue.push_back((current_indicators, count + 1));
        }
    }

    0
}
