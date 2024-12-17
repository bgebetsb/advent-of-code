use std::array;
use std::fmt::{Display, Formatter, Result};
use std::fs::read_to_string;
use std::ops::Deref;
use utils::*;

#[derive(PartialEq)]
struct Instructions(Vec<usize>);

impl Deref for Instructions {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let formatted = self
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", formatted)
    }
}

fn calculation(registers: &mut [usize], instructions: &Instructions) -> Instructions {
    let mut output = vec![];
    let mut instruction_pointer = 0;

    while instruction_pointer < instructions.len() {
        let opcode = instructions[instruction_pointer];
        let operand = instructions[instruction_pointer + 1];
        match opcode {
            0 => {
                registers[0] /= usize::pow(2, combo_operand(registers, operand) as u32);
            }
            1 => registers[1] ^= operand,
            2 => registers[1] = combo_operand(registers, operand) % 8,
            3 if registers[0] != 0 => instruction_pointer = operand,
            4 => registers[1] ^= registers[2],
            5 => output.push(combo_operand(registers, operand) % 8),
            6 => {
                registers[1] =
                    registers[0] / usize::pow(2, combo_operand(registers, operand) as u32)
            }
            7 => {
                registers[2] =
                    registers[0] / usize::pow(2, combo_operand(registers, operand) as u32)
            }
            _ => (),
        }
        if opcode != 3 || registers[0] == 0 {
            instruction_pointer += 2;
        }
    }
    Instructions(output)
}

fn combo_operand(registers: &[usize], operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("Wrong operand in combo"),
    }
}

fn get_start_number(instructions: &Instructions) -> usize {
    let mut min = 0;
    let mut max = usize::MAX;
    let mut current = min / 2 + max / 2;

    while min + 1 < max {
        let output = calculation(&mut [current, 0, 0], instructions);
        if output.len() < instructions.len() {
            min = current;
        } else if output.len() >= instructions.len() {
            max = current;
        }
        current = min / 2 + max / 2;
    }

    for i in current + 1.. {
        let output = calculation(&mut [i, 0, 0], instructions);
        let search_length = usize::min(instructions.len(), 7);
        if output[0..search_length] == instructions[0..search_length] {
            return i;
        }
    }
    panic!("No number found");
}

fn find_match(instructions: &Instructions, start: usize, step: usize) -> usize {
    let mut current = start;
    let mut step = step;

    loop {
        let result = calculation(&mut [current, 0, 0], instructions);
        if result == *instructions {
            return current;
        } else if result.len() > instructions.len() {
            println!(
                "Couldn't find a match using step size {step}, trying {} now",
                step / 2
            );
            current = start;
            step /= 2;
        } else {
            current += step;
        }
    }
}

fn find_lower_number(instructions: &Instructions, start: usize, step: usize) -> usize {
    let mut lowest = start;
    let mut current = start - step;

    for _ in 0..100000 {
        let result = calculation(&mut [current, 0, 0], instructions);
        if result == *instructions {
            lowest = current;
        }
        current -= step;
    }
    if step > 16384 {
        return find_lower_number(instructions, lowest, step / 2);
    }
    lowest
}

fn part2(instructions: &Instructions) -> usize {
    let start = get_start_number(instructions);
    let step = 134217728; // 16384 * 8192

    let mut lowest = find_match(instructions, start, step);
    lowest = find_lower_number(instructions, lowest, step / 2);

    lowest
}

fn input_parsing(input: &[String]) -> ([usize; 3], Instructions) {
    let registers = array::from_fn(|i| {
        input[i]
            .split(": ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });

    let instructions = input[4]
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .map(|nbr| nbr.parse::<usize>().unwrap())
        .collect();

    (registers, Instructions(instructions))
}

fn main() {
    let input = read_to_string("input.txt").unwrap().get_lines();
    let (registers, instructions) = input_parsing(&input);

    let part1 = calculation(&mut registers.clone(), &instructions);
    println!("Part 1: {part1}");

    let part2 = part2(&instructions);
    println!("Part 2: {part2}");
}
