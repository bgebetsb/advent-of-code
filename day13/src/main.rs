use std::{fs::read_to_string, io};
use utils::*;

fn calculate(ax: isize, ay: isize, bx: isize, by: isize, px: isize, py: isize) -> Option<isize> {
    let determinant = ax * by - bx * ay;

    if determinant == 0 {
        return None;
    }

    let mut button_a = px * by - py * bx;
    let mut button_b = ax * py - ay * px;

    if button_a % determinant != 0 || button_b % determinant != 0 {
        return None;
    }

    button_a /= determinant;
    button_b /= determinant;

    Some(button_a * 3 + button_b)
}

fn parse_line(line: &str, delimiter: char) -> Vec<isize> {
    line.split(delimiter)
        .skip(1)
        .map(|item| {
            item.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<isize>()
                .unwrap()
        })
        .collect()
}

fn parse_input(lines: &[String]) -> Vec<(Vec<isize>, Vec<isize>, Vec<isize>)> {
    lines
        .chunks(4)
        .map(|chunk| {
            let button_a = parse_line(&chunk[0], '+');
            let button_b = parse_line(&chunk[1], '+');
            let point = parse_line(&chunk[2], '=');
            (button_a, button_b, point)
        })
        .collect()
}

fn main() -> Result<(), io::Error> {
    let lines = read_to_string("input.txt")?.get_lines();
    let machines = parse_input(&lines);

    let mut part1 = 0;
    let mut part2 = 0;
    for machine in &machines {
        let (ax, ay) = (machine.0[0], machine.0[1]);
        let (bx, by) = (machine.1[0], machine.1[1]);
        let (mut px, mut py) = (machine.2[0], machine.2[1]);

        if let Some(tokens) = calculate(ax, ay, bx, by, px, py) {
            part1 += tokens;
        }

        px += 10000000000000;
        py += 10000000000000;

        if let Some(tokens) = calculate(ax, ay, bx, by, px, py) {
            part2 += tokens;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
