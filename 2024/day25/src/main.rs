use std::{fs::read_to_string, io::Error};

use utils_2024::*;

fn calculate_heights(lock: &Vec<Vec<char>>) -> (bool, Vec<i32>) {
    let mut heights = Vec::new();
    let mut is_lock = true;
    for field in &lock[0] {
        heights.push(-1);
        if *field != '#' {
            is_lock = false;
        }
    }
    for row in lock {
        for (i, field) in row.iter().enumerate() {
            if *field == '#' {
                heights[i] += 1;
            }
        }
    }
    (is_lock, heights)
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?.get_lines().lines_as_chars();
    let mut locks = Vec::new();
    let mut current_lock = Vec::new();
    for line in &input {
        if line.is_empty() {
            locks.push(current_lock.clone());
            current_lock = Vec::new();
        } else {
            current_lock.push(line.clone());
        }
    }
    if !current_lock.is_empty() {
        locks.push(current_lock.clone());
    }

    let mut locks_new = Vec::new();
    let mut keys = Vec::new();
    for lock in &locks {
        let (is_lock, heights) = calculate_heights(lock);
        if is_lock {
            locks_new.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut total = 0;
    for lock in &locks_new {
        for key in &keys {
            let mut possible = true;
            for (field_lock, field_key) in lock.iter().zip(key.iter()) {
                if *field_lock + *field_key > 5 {
                    possible = false;
                }
            }
            if possible {
                total += 1;
            }
        }
    }

    println!("Part 1: {}", total);

    Ok(())
}
