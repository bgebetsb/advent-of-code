use std::{collections::HashSet, fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Ord, PartialOrd)]
struct Coordinates {
    x: i128,
    y: i128,
    z: i128,
}

fn main() -> Result<(), io::Error> {
    let (filename, pairs) = if cfg!(debug_assertions) {
        ("example.txt", 10)
    } else {
        ("input.txt", 1000)
    };

    let content: Vec<_> = read_to_string(filename)?
        .get_lines()
        .numbers_in_line(b',')
        .iter()
        .map(|line| Coordinates {
            x: line[0],
            y: line[1],
            z: line[2],
        })
        .collect();

    let item_count = content.iter().collect::<HashSet<_>>().len();

    let single_circuits = get_single_circuits(&content);

    let p1_lengths = get_circuit_lengths(&single_circuits, pairs);
    let part1 = p1_lengths[0] * p1_lengths[1] * p1_lengths[2];

    println!("Part 1: {}", part1);

    let mut min = 0;
    let mut max = single_circuits.len();
    while min != max {
        let current = (min + max) / 2;
        if get_circuit_lengths(&single_circuits, current)[0] == item_count {
            max = current;
        } else {
            min = current + 1;
        }
    }

    let part2_element = single_circuits[min - 1];
    println!("Part 2: {}", part2_element.0.x * part2_element.1.x);

    Ok(())
}

fn get_single_circuits(content: &[Coordinates]) -> Vec<(&Coordinates, &Coordinates, f64)> {
    let mut single_circuits = Vec::new();

    for (i, current) in content.iter().enumerate() {
        for item in content.iter().skip(i + 1) {
            let diff = calc_distance(current, item);
            single_circuits.push((current, item, diff));
        }
    }

    single_circuits.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    single_circuits
}

fn get_circuit_lengths(
    single_circuits: &[(&Coordinates, &Coordinates, f64)],
    pairs: usize,
) -> Vec<usize> {
    let single_circuits = &single_circuits[0..pairs];
    let mut circuits: Vec<HashSet<&Coordinates>> = Vec::new();

    loop {
        let mut sth_found = false;
        'outer: for item in single_circuits {
            for circuit in circuits.iter_mut() {
                match (circuit.contains(&item.0), circuit.contains(&item.1)) {
                    (true, true) => (),
                    (false, false) => continue,
                    _ => {
                        circuit.insert(item.0);
                        circuit.insert(item.1);
                        sth_found = true;
                    }
                }
                continue 'outer;
            }

            let newcircuit: HashSet<_> = [item.0, item.1].into_iter().collect();
            circuits.push(newcircuit);
        }

        if !sth_found {
            break;
        }
    }

    let mut lengths: Vec<_> = circuits.iter().map(|circuit| circuit.len()).collect();
    lengths.sort_unstable_by(|a, b| b.cmp(a));

    lengths
}

fn calc_distance(a: &Coordinates, b: &Coordinates) -> f64 {
    let x_diff = a.x - b.x;
    let y_diff = a.y - b.y;
    let z_diff = a.z - b.z;

    let sum = x_diff * x_diff + y_diff * y_diff + z_diff * z_diff;

    f64::sqrt(sum as f64)
}
