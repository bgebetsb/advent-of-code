use nalgebra::DMatrix;
use regex::Regex;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Types {
    ButtonA(i64, i64),
    ButtonB(i64, i64),
    Prize(i64, i64),
    Empty,
}

#[derive(Debug)]
struct ClawMachine(Types, Types, Types);

impl From<&String> for Types {
    fn from(s: &String) -> Types {
        if s.contains("Button A") {
            let re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
            let groups = re.captures_iter(s).next().unwrap();
            let x = groups.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = groups.get(2).unwrap().as_str().parse::<i64>().unwrap();
            return Types::ButtonA(x, y);
        } else if s.contains("Button B") {
            let re = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
            let groups = re.captures_iter(s).next().unwrap();
            let x = groups.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = groups.get(2).unwrap().as_str().parse::<i64>().unwrap();
            return Types::ButtonB(x, y);
        } else if s.contains("Prize") {
            let re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
            let groups = re.captures_iter(s).next().unwrap();
            let x = groups.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = groups.get(2).unwrap().as_str().parse::<i64>().unwrap();
            return Types::Prize(x, y);
        }
        Types::Empty
    }
}

fn calculate(machines: &[ClawMachine]) -> usize {
    let mut tokens = 0;
    for machine in machines {
        println!("{:?}", machine);
        if let (Types::ButtonA(ax, ay), Types::ButtonB(bx, by), Types::Prize(px, py)) =
            (machine.0, machine.1, machine.2)
        {
            let a_matrix = DMatrix::from_row_slice(2, 2, &[ax, bx, ay, by]);
            let b_matrix = DMatrix::from_row_slice(2, 1, &[px, py]);
            let solution = a_matrix.try_lu().unwrap().solve(&b_matrix).unwrap();
            /*
            let mut calc_x = real_x;
            let mut calc_y = real_x;
            let mut try_b = (calc_x / button_b_y).min(calc_x / button_b_x);
            for i in (0..=try_b).rev() {
                let rest_x = calc_x - (i * button_b_x);
                let rest_y = calc_y - (i * button_b_y);
                if (rest_x == 0 && rest_y == 0)
                    || (rest_x % button_a_x == 0
                        && rest_y % button_a_y == 0
                        && (rest_x / button_a_x == rest_y / button_a_y))
                {
                    let multiplicator = (real_x / calc_x) as usize;
                    tokens += ((rest_x / button_a_x) as usize * 3 + i as usize) * multiplicator;
                    break;
                }
            }
            */
        }
    }
    tokens
}

fn handle_input(input: &String) {
    let result: Vec<Types> = input
        .get_lines()
        .iter()
        .map(|line| line.into())
        .filter(|item| *item != Types::Empty)
        .collect();

    let mut clawmachines = Vec::new();

    let mut i = 0;
    while i < result.len() {
        clawmachines.push(ClawMachine(result[i], result[i + 1], result[i + 2]));
        i += 3;
    }

    let mut part1 = calculate(&clawmachines);
    let mut part2 = 0;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part1);
}

fn main() -> Result<(), io::Error> {
    let input = read_to_string("example.txt")?;
    println!("Example: ");
    handle_input(&input);

    println!("\nReal:");
    let input_real = read_to_string("input.txt")?;
    handle_input(&input_real);

    Ok(())
}
