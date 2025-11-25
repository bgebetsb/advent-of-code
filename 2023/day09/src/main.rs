use std::{collections::VecDeque, fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines().numbers_in_line(b' ');

    let mut part1 = 0;
    let mut part2 = 0;
    for line in content {
        let mut history = vec![VecDeque::from(line)];

        let mut last_step = history.iter().next_back().unwrap().clone();
        while last_step.iter().filter(|&nbr| *nbr != 0).count() > 0 {
            let mut cur = last_step[0];
            let new_last = last_step
                .iter()
                .skip(1)
                .map(|nbr| {
                    let new_value = *nbr - cur;
                    cur = *nbr;
                    new_value
                })
                .collect();

            last_step = new_last;
            history.push(last_step.clone());
        }

        let mut current_front = 0;
        let mut current_back = 0;
        for step in history.iter_mut().rev() {
            let first = step.iter().next().unwrap();
            current_front = first - current_front;
            step.push_front(current_front);

            let last = step.iter().next_back().unwrap();
            current_back += last;
            step.push_back(current_back);
        }

        part1 += history.first().unwrap().iter().next_back().unwrap();
        part2 += history.first().unwrap().iter().next().unwrap();
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);

    Ok(())
}
