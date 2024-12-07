use std::fs::read_to_string;
use std::io;
use utils::*;

struct InputLine {
    key: i128,
    list: Vec<i128>,
    possible: bool,
}

fn calculate_result(input: &mut InputLine, current_value: i128, position: usize, part2: bool) {
    if position == input.list.len() {
        if current_value == input.key {
            input.possible = true;
        }
        return;
    }

    calculate_result(
        input,
        current_value + input.list[position],
        position + 1,
        part2,
    );
    calculate_result(
        input,
        current_value * input.list[position],
        position + 1,
        part2,
    );
    if part2 {
        calculate_result(
            input,
            format!("{}{}", current_value, input.list[position])
                .parse::<i128>()
                .unwrap(),
            position + 1,
            part2,
        );
    }
}

fn main() -> Result<(), io::Error> {
    let lines = read_to_string("input.txt")?.get_lines();

    let firstpart = lines.numbers_in_line(b':');
    let list = lines.numbers_in_line(b' ');

    let mut input = Vec::new();
    for i in 0..firstpart.len() {
        input.push(InputLine {
            key: firstpart[i][0],
            list: list[i].clone(),
            possible: false,
        });
    }

    let mut sum = 0;
    let mut sum_part2 = 0;
    for item in &mut input {
        calculate_result(item, item.list[0], 1, false);
        if item.possible {
            sum += item.key;
            sum_part2 += item.key;
        } else {
            calculate_result(item, item.list[0], 1, true);
            if item.possible {
                sum_part2 += item.key;
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_part2);

    Ok(())
}
