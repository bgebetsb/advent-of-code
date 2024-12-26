use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_to_string("input.txt")?;

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let mut count_enabled = true;
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    for captures in re.captures_iter(&content) {
        if captures.get(1).is_some() {
            count_enabled = true;
        } else if captures.get(2).is_some() {
            count_enabled = false;
        } else {
            let nbr1 = captures.get(3).unwrap().as_str().parse::<i32>()?;
            let nbr2 = captures.get(4).unwrap().as_str().parse::<i32>()?;
            let result = nbr1 * nbr2;

            sum_part1 += result;
            if count_enabled {
                sum_part2 += result;
            }
        }
    }

    println!("Part 1: {sum_part1}");
    println!("Part 2: {sum_part2}");
    Ok(())
}
