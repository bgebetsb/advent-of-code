use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use utils::*;

#[derive(Clone)]
struct NumberWithLength {
    value: i128,
    length: usize,
}

impl NumberWithLength {
    fn new(value: i128) -> Self {
        let length = {
            if value == 0 {
                1
            } else {
                (value as f64).log10().floor() as usize + 1
            }
        };
        NumberWithLength { value, length }
    }

    fn new_with_length(value: i128, length: usize) -> Self {
        NumberWithLength { value, length }
    }

    fn update_value(&mut self, value: i128) {
        let length = {
            if value == 0 {
                1
            } else {
                (value as f64).log10().floor() as usize + 1
            }
        };
        self.value = value;
        self.length = length;
    }
}

fn calculate_recursive(input: i128, currentlength: Option<usize>, iterations: usize) -> i128 {
    if iterations == 75 {
        return 1;
    }

    if input == 0 {
        return calculate_recursive(1, Some(1), iterations + 1);
    }

    let length = match currentlength {
        Some(value) => value,
        _ => input.to_string().len(),
    };
    if length % 2 == 0 {
        let strval = input.to_string();
        let left = strval[0..strval.len() / 2].parse::<i128>().unwrap();
        let right = strval[strval.len() / 2..].parse::<i128>().unwrap();
        return calculate_recursive(left, Some(length / 2), iterations + 1)
            + calculate_recursive(right, None, iterations + 1);
    }

    return calculate_recursive(input * 2024, None, iterations + 1);
}

fn recursive_with_hashmap(
    hashmap: &mut HashMap<i128, HashMap<usize, i128>>,
    value: i128,
    remaining_iterations: usize,
) -> i128 {
    if remaining_iterations == 0 {
        return 1;
    }

    let mut found_position = None;
    let mut found_value = None;
    if let Some(values) = hashmap.get_mut(&value) {
        if let Some(inner_value) = values.get(&remaining_iterations) {
            return *inner_value;
        } else {
            for i in (1..remaining_iterations).rev() {
                if let Some(inner_value) = values.get(&i) {
                    found_position = Some(i);
                    found_value = Some(*inner_value);
                    break;
                }
            }
        }
    }

    if let (Some(pos), Some(val)) = (found_position, found_value) {
        let result = recursive_with_hashmap(hashmap, val, remaining_iterations - pos);
        hashmap
            .entry(value)
            .or_insert(HashMap::new())
            .insert(remaining_iterations, result);
        return result;
    }

    let inputstr = value.to_string();
    if inputstr.len() % 2 == 0 {
        let left = inputstr[0..inputstr.len() / 2].parse::<i128>().unwrap();
        let right = inputstr[inputstr.len() / 2..].parse::<i128>().unwrap();
        let result = recursive_with_hashmap(hashmap, left, remaining_iterations - 1)
            + recursive_with_hashmap(hashmap, right, remaining_iterations - 1);
        hashmap
            .entry(value)
            .or_insert(HashMap::new())
            .insert(remaining_iterations, result);
        return result;
    } else {
        let result = recursive_with_hashmap(hashmap, value * 2024, remaining_iterations - 1);
        hashmap
            .entry(value)
            .or_insert(HashMap::new())
            .insert(remaining_iterations, result);
        return result;
    }
}

fn main() -> Result<(), io::Error> {
    let input: Vec<i128> = read_to_string("example.txt")?
        .split_whitespace()
        .map(|number| number.parse::<i128>().unwrap())
        .collect();

    let mut hashmap: HashMap<i128, HashMap<usize, i128>> = HashMap::new();

    let mut total = 0;
    for number in input {
        println!("Starting with number(current total: {total})");
        // total += calculate_recursive(number, None, 0);
        total += recursive_with_hashmap(&mut hashmap, number, 25)
    }

    println!("{:?}", hashmap);

    println!("Total: {total}");

    /*
        let input: Vec<NumberWithLength> = read_to_string("input.txt")?
            .get_lines()
            .numbers_in_line(b' ')[0]
            .iter()
            .map(|number| NumberWithLength::new(*number))
            .collect();

        let mut total = 0;
        for number in input {
            let mut tmp: Vec<NumberWithLength> = Vec::new();
            tmp.push(number.clone());
            for i in 0..75 {
                println!("Iteration: {}, Count: {}", i, tmp.len());
                for j in (0..tmp.len()).rev() {
                    if tmp[j].value == 0 {
                        tmp[j].value = 1;
                    } else if tmp[j].length % 2 == 0 {
                        let splitlength = tmp[j].length / 2;
                        let divisor = 10i128.pow(splitlength as u32);
                        let left = tmp[j].value / divisor;
                        let right = tmp[j].value % divisor;
                        /*
                        let strval = input[i].value.to_string();
                        let left = &strval[0..strval.len() / 2];
                        let right = &strval[strval.len() / 2..];
                        */
                        tmp.insert(j, NumberWithLength::new_with_length(left, splitlength));
                        tmp[j + 1].update_value(right);
                    } else {
                        let newval = tmp[j].value * 2024;
                        tmp[j].update_value(newval);
                    }
                }
            }
            total += tmp.len();
            println!("New Total: {}", total);
        }
    */
    /*
    for i in 0..75 {
        println!("Iteration: {}, Count: {}", i, input.len());
        for i in (0..input.len()).rev() {
            if input[i].value == 0 {
                input[i].value = 1;
            } else if input[i].length % 2 == 0 {
                let splitlength = input[i].length / 2;
                let divisor = 10i128.pow(splitlength as u32);
                let left = input[i].value / divisor;
                let right = input[i].value % divisor;
                /*
                let strval = input[i].value.to_string();
                let left = &strval[0..strval.len() / 2];
                let right = &strval[strval.len() / 2..];
                */
                input.insert(i, NumberWithLength::new_with_length(left, splitlength));
                input[i + 1].update_value(right);
            } else {
                let newval = input[i].value * 2024;
                input[i].update_value(newval);
            }
        }
        /*
        for (index, stone) in input.iter().enumerate() {
            match stone {
                0 => stone[index] = 1,
                _ => {
                    let strval = stone.to_string();
                    if strval.len() % 2 == 0 {
                        let left = &strval[0..strval.len() / 2];
                        let right = &strval[strval.len() / 2..];
                        input.remove(index);
                        input.insert(index, right.parse::<i128>().unwrap());
                        input.insert(index, left.parse::<i128>().unwrap());
                    } else {
                        *stone = *stone * 2024;
                    }
                }
            }
        }
        */
    }

    println!("Part 1: {}", input.len());
    */

    Ok(())
}
