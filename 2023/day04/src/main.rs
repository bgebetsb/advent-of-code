use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut copies = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let stripped = line.split(": ").nth(1).unwrap();
        let splitted: Vec<_> = stripped.split('|').collect();

        let winning: Vec<_> = splitted[0]
            .split(' ')
            .filter_map(|value| {
                if value.is_empty() {
                    None
                } else {
                    Some(value.parse::<usize>().unwrap())
                }
            })
            .collect();

        let actual: Vec<_> = splitted[1]
            .split(' ')
            .filter_map(|value| {
                if value.is_empty() {
                    None
                } else {
                    Some(value.parse::<usize>().unwrap())
                }
            })
            .collect();

        let mut total: usize = 0;
        for nbr in actual {
            if winning.contains(&nbr) {
                total += 1;
            }
        }

        let mut amount = 1;
        if let Some(value) = copies.get(&index) {
            amount += *value;
        }

        if total != 0 {
            part1 += usize::pow(2, (total - 1).try_into().unwrap());

            for i in 0..total {
                if let Some(value) = copies.get_mut(&(index + i + 1)) {
                    *value += amount;
                } else {
                    copies.insert(index + i + 1, amount);
                }
            }
        }

        part2 += amount;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
