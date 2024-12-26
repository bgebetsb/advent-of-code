use std::fs::read_to_string;
use std::ops::AddAssign;
use std::{char, io};
use utils_2024::*;

struct Plant(char, bool);

impl PartialEq for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct Values(u64, u64, u64);

impl From<Values> for (u64, u64, u64) {
    fn from(value: Values) -> Self {
        (value.0, value.1, value.2)
    }
}

impl AddAssign for Values {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

fn is_perimeter(
    map: &mut [Vec<Plant>],
    y: usize,
    y_offset: isize,
    x: usize,
    x_offset: isize,
) -> bool {
    let new_y = y as isize + y_offset;
    let new_x = x as isize + x_offset;

    new_y < 0
        || new_x < 0
        || (new_y as usize) >= map.len()
        || (new_x as usize) >= map[new_y as usize].len()
        || map[new_y as usize][new_x as usize] != map[y][x]
}

fn diagonal_corners(
    map: &mut [Vec<Plant>],
    y: usize,
    y_offset: isize,
    x: usize,
    x_offset: isize,
) -> bool {
    let new_y = y as isize + y_offset;
    let new_x = x as isize + x_offset;

    if new_y < 0 || new_x < 0 {
        return false;
    }

    let new_y = new_y as usize;
    let new_x = new_x as usize;

    if new_y >= map.len() || new_x >= map[new_y].len() {
        return false;
    }

    map[new_y][x] != map[y][x] && map[y][new_x] == map[y][x] && map[new_y][new_x] == map[y][x]
}

fn perimeter_corner_count(map: &mut [Vec<Plant>], y: usize, x: usize) -> (u64, u64) {
    let top = is_perimeter(map, y, -1, x, 0);
    let bottom = is_perimeter(map, y, 1, x, 0);
    let left = is_perimeter(map, y, 0, x, -1);
    let right = is_perimeter(map, y, 0, x, 1);

    let perimeters = [top, bottom, left, right]
        .iter()
        .filter(|&&val| val)
        .count() as u64;

    let mut corners = (top && left) as u64
        + (top && right) as u64
        + (right && bottom) as u64
        + (left && bottom) as u64;

    if (1..=3).contains(&perimeters) {
        corners += diagonal_corners(map, y, -1, x, -1) as u64;
        corners += diagonal_corners(map, y, -1, x, 1) as u64;
        corners += diagonal_corners(map, y, 1, x, -1) as u64;
        corners += diagonal_corners(map, y, 1, x, 1) as u64;
    }

    (perimeters, corners)
}

fn calculate(map: &mut [Vec<Plant>], y: usize, x: usize) -> Values {
    if map[y][x].1 {
        return Values(0, 0, 0);
    }

    map[y][x].1 = true;

    let (perimeters, corners) = perimeter_corner_count(map, y, x);
    let mut values = Values(1, perimeters, corners);

    if y > 0 && map[y - 1][x] == map[y][x] {
        values += calculate(map, y - 1, x)
    }

    if x > 0 && map[y][x - 1] == map[y][x] {
        values += calculate(map, y, x - 1)
    }

    if y < map.len() - 1 && map[y + 1][x] == map[y][x] {
        values += calculate(map, y + 1, x)
    }

    if x < map[y].len() - 1 && map[y][x + 1] == map[y][x] {
        values += calculate(map, y, x + 1)
    }

    values
}

fn main() -> Result<(), io::Error> {
    let mut input: Vec<Vec<Plant>> = read_to_string("input.txt")?
        .get_lines()
        .lines_as_chars()
        .iter()
        .map(|line| line.iter().map(|&c| Plant(c, false)).collect())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j].1 {
                continue;
            }
            let (area, perimeters, sides) = calculate(&mut input, i, j).into();
            part1 += area * perimeters;
            part2 += area * sides;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
