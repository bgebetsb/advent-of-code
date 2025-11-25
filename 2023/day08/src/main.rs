use std::{collections::HashMap, fs::read_to_string, io};

use utils::{math::lcm, string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn get_step_count<F>(
    map: &HashMap<&String, (&String, &String)>,
    instructions: &str,
    initial: &String,
    end_condition: F,
) -> usize
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0;
    let mut current = initial;
    let mut current_instructions = instructions;

    while !end_condition(current) {
        let current_instruction = current_instructions.chars().next().unwrap();

        current = match current_instruction {
            'L' => map.get(current).unwrap().0,
            'R' => map.get(current).unwrap().1,
            _ => unreachable!(),
        };

        steps += 1;

        current_instructions = &current_instructions[1..];
        if current_instructions.is_empty() {
            current_instructions = instructions;
        }
    }

    steps
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines();
    let instructions = content.first().unwrap();

    let elements: Vec<(String, Vec<String>)> = content[2..]
        .to_vec()
        .split_with_key('=', &['(', ')', ' ', ',']);

    let map: HashMap<_, _> = elements
        .iter()
        .map(|(key, value)| (key, (&value[0], &value[1])))
        .collect();

    let part1_initial = elements
        .iter()
        .filter_map(|(key, _)| if key == "AAA" { Some(key) } else { None })
        .next()
        .unwrap();

    let part1 = get_step_count(&map, instructions, part1_initial, |item| item == "ZZZ");
    println!("Part 1: {}", part1);

    let part2_initial: Vec<_> = elements
        .iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(key) } else { None })
        .collect();

    let part2_step_counts: Vec<_> = part2_initial
        .iter()
        .map(|initial| get_step_count(&map, instructions, initial, |item| item.ends_with('Z')))
        .collect();

    let part2 = lcm(&part2_step_counts);
    println!("Part 2: {}", part2);

    Ok(())
}
