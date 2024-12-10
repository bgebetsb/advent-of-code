use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
    Increasing,
    Decreasing,
}

fn check_valid(list: &Vec<&str>) -> Result<bool, Box<dyn Error>> {
    let mut status: Option<Status> = None;
    let mut previous: Option<i32> = None;

    for part in list {
        let current = part.parse::<i32>()?;
        match (status, previous, current) {
            (_, None, _) => previous = Some(current),
            (None, Some(prev_val), cur_val) => {
                if prev_val == cur_val || (prev_val - cur_val).abs() > 3 {
                    return Ok(false);
                } else if prev_val < cur_val {
                    status = Some(Status::Increasing);
                } else {
                    status = Some(Status::Decreasing);
                }
                previous = Some(cur_val);
            }
            (Some(status_val), Some(prev_val), cur_val) => {
                if prev_val == cur_val
                    || (prev_val - cur_val).abs() > 3
                    || ((prev_val > cur_val) == (status_val == Status::Increasing))
                {
                    return Ok(false);
                }
                previous = Some(cur_val);
            }
        }
    }
    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut safe_part_1 = 0;
    let mut safe_part_2 = 0;
    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if check_valid(&parts)? {
            safe_part_1 += 1;
            safe_part_2 += 1;
        } else {
            for i in 0..parts.len() {
                let mut parts_clone = parts.clone();
                parts_clone.remove(i);
                if check_valid(&parts_clone)? {
                    safe_part_2 += 1;
                    break;
                }
            }
        }
    }

    println!("Safe Part 1: {}", safe_part_1);
    println!("Safe Part 2: {}", safe_part_2);
    Ok(())
}
