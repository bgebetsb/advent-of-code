use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    io::Error,
    mem,
};

use utils::*;

fn resolve_item(
    connection_map: &HashMap<String, Connection>,
    value_map: &HashMap<String, u8>,
    target: &String,
) -> u8 {
    let connection = connection_map.get(target).unwrap();

    resolve(connection_map, value_map, connection)
}

fn resolve(
    connection_map: &HashMap<String, Connection>,
    value_map: &HashMap<String, u8>,
    connection: &Connection,
) -> u8 {
    let mut items = [None, None];

    if let Some(value) = value_map.get(&connection.input1) {
        items[0] = Some(*value);
    } else {
        items[0] = Some(resolve_item(connection_map, value_map, &connection.input1));
    }

    if let Some(value) = value_map.get(&connection.input2) {
        items[1] = Some(*value);
    } else {
        items[1] = Some(resolve_item(connection_map, value_map, &connection.input2));
    }

    match connection.operation {
        Operation::And => items[0].unwrap() & items[1].unwrap(),
        Operation::Or => items[0].unwrap() | items[1].unwrap(),
        Operation::Xor => items[0].unwrap() ^ items[1].unwrap(),
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct Connection {
    input1: String,
    operation: Operation,
    input2: String,
    output: String,
    start_wire: bool,
    end_wire: bool,
}

impl Connection {
    fn new(input1: String, operation: &str, input2: String, output: String) -> Connection {
        let operation = match operation {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Invalid operation"),
        };

        let start_wire = input1.starts_with("x") || input1.starts_with("y");
        let end_wire = output.starts_with("z");

        Connection {
            input1,
            operation,
            input2,
            output,
            start_wire,
            end_wire,
        }
    }
}

fn find_broken_xor_wires(connection_map: &HashMap<String, Connection>) -> Vec<String> {
    connection_map
        .iter()
        .filter_map(|(_, connection)| {
            if connection.operation == Operation::Xor {
                if !connection.start_wire && !connection.end_wire {
                    Some(connection.output.clone())
                } else {
                    None
                }
            } else if connection.end_wire && connection.output != "z45" {
                Some(connection.output.clone())
            } else {
                None
            }
        })
        .collect()
}

fn swap_two_elements(
    connection_map: &mut HashMap<String, Connection>,
    output1: &String,
    output2: &String,
) {
    let mut item1 = connection_map.remove(output1).unwrap();
    let mut item2 = connection_map.remove(output2).unwrap();

    mem::swap(&mut item1.output, &mut item2.output);
    connection_map.insert(output1.clone(), item2);
    connection_map.insert(output2.clone(), item1);
}

fn fix_broken_wires(connection_map: &mut HashMap<String, Connection>) -> Vec<(String, String)> {
    let broken_wires = find_broken_xor_wires(connection_map);
    let mut z_node = "z45".to_string();

    let mut queue = VecDeque::new();
    queue.push_back((z_node.clone(), 0, 0));

    let mut swappairs = Vec::new();
    let mut swapitem: Option<String> = None;
    while let Some((wire, and_count, or_count)) = queue.pop_front() {
        let mut result = connection_map.get(&wire).unwrap().clone();

        if result.operation == Operation::Xor && result.start_wire {
            let nbr = &result.input1[1..].parse::<usize>().unwrap();
            z_node = format!("z{:02}", *nbr - 1);
        }

        if broken_wires.contains(&result.output) && broken_wires.contains(&z_node) {
            swappairs.push((result.output.to_string(), z_node.clone()));
            swap_two_elements(connection_map, &result.output, &z_node);
            result = connection_map.get(&wire).unwrap().clone();
        }

        let (and_count, or_count) = match result.operation {
            Operation::And => (and_count + 1, 0),
            _ => (0, or_count + 1),
        };

        if (and_count > 1 || or_count > 1) && result.input1 != "x00" && result.input1 != "y00" {
            if let Some(item) = swapitem {
                swappairs.push((item.clone(), result.output.clone()));
                swap_two_elements(connection_map, &result.output, &item);
                swapitem = None;
            } else {
                swapitem = Some(result.output);
            }
        }

        if !result.start_wire {
            queue.push_back((result.input1.clone(), and_count, or_count));
            queue.push_back((result.input2.clone(), and_count, or_count));
        }
    }
    swappairs
}

fn calculate(connection_map: &HashMap<String, Connection>, value_map: &HashMap<String, u8>) -> u64 {
    let mut sum = 0;

    for z_value in (0..=45).rev() {
        let z_named = format!("z{:02}", z_value);
        let connection = connection_map.get(&z_named).unwrap();
        let result = resolve(connection_map, value_map, connection);
        sum <<= 1;
        sum += result as u64;
    }

    sum
}

fn part2(connection_map: &mut HashMap<String, Connection>, value_map: &HashMap<String, u8>) {
    let mut x_sum = 0;
    let mut y_sum = 0;

    for i in (0..=44).rev() {
        let x_str = format!("x{:02}", i);
        let x_value = value_map.get(&x_str).unwrap();
        x_sum <<= 1;
        x_sum += *x_value as u64;

        let y_str = format!("y{:02}", i);
        let y_value = value_map.get(&y_str).unwrap();
        y_sum <<= 1;
        y_sum += *y_value as u64;
    }

    let wire_pairs = fix_broken_wires(connection_map);
    let mut wires: Vec<&str> = wire_pairs
        .iter()
        .flat_map(|(first, second)| [first.as_str(), second.as_str()])
        .collect();
    wires.sort_unstable();
    println!("Expected Result: {:046b}", x_sum + y_sum);
    let result = calculate(connection_map, value_map);
    println!("Actual result:   {:046b}", result);
    println!("Part 2: {}", wires.join(","));
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?.get_lines();

    let mut value_map = HashMap::new();
    let values: Vec<(String, Vec<u8>)> = input.split_with_key(':');
    for item in &values {
        value_map.insert(item.0.clone(), item.1[0]);
    }

    let mut connections = HashMap::new();
    for line in &input {
        let splitted: Vec<&str> = line.split(" ").collect();
        if splitted.len() == 5 {
            let item = Connection::new(
                splitted[0].to_string(),
                splitted[1],
                splitted[2].to_string(),
                splitted[4].to_string(),
            );
            connections.insert(splitted[4].to_string(), item);
        }
    }

    let part1 = calculate(&connections, &value_map);
    println!("Part 1: {}", part1);

    part2(&mut connections.clone(), &value_map);

    Ok(())
}
