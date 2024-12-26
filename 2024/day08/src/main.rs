use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use utils_2024::*;

#[derive(Clone)]
struct Item {
    antinode: bool,
    x: usize,
    y: usize,
}

impl Item {
    fn new(x: usize, y: usize) -> Self {
        Item {
            antinode: false,
            x,
            y,
        }
    }
}

fn check_position(map: &mut [Vec<Item>], y: i64, x: i64) -> Option<bool> {
    if x < 0 || y < 0 {
        return None;
    }

    let y = y as usize;
    let x = x as usize;

    if y >= map.len() || x >= map[y].len() {
        return None;
    }

    if !map[y][x].antinode {
        map[y][x].antinode = true;
        Some(true)
    } else {
        Some(false)
    }
}

fn searching(
    map: &mut [Vec<Item>],
    x: i64,
    y: i64,
    x_diff: i64,
    y_diff: i64,
    part2: bool,
) -> usize {
    let mut total_antinodes = 0;
    let mut x = x;
    let mut y = y;

    while let Some(result) = check_position(map, y, x) {
        if result {
            total_antinodes += 1;
        }
        x += x_diff;
        y += y_diff;
        if !part2 {
            return total_antinodes;
        }
    }
    total_antinodes
}

fn calculation(map: &mut [Vec<Item>], groups: &HashMap<char, Vec<Item>>, part2: bool) -> usize {
    let mut total_antinodes = 0;

    for group in groups {
        for (i, item) in group.1.iter().enumerate() {
            for other in &group.1[i + 1..] {
                let y1 = item.y as i64;
                let x1 = item.x as i64;
                let y2 = other.y as i64;
                let x2 = other.x as i64;

                let x_diff = x1 - x2;
                let y_diff = y1 - y2;

                if !part2 {
                    total_antinodes +=
                        searching(map, x1 + x_diff, y1 + y_diff, x_diff, y_diff, part2);
                    total_antinodes +=
                        searching(map, x2 - x_diff, y2 - y_diff, -x_diff, -y_diff, part2);
                } else {
                    total_antinodes += searching(map, x1, y1, x_diff, y_diff, part2);
                    total_antinodes += searching(map, x2, y2, -x_diff, -y_diff, part2);
                }
            }
        }
    }
    total_antinodes
}

fn main() -> Result<(), io::Error> {
    let chars = read_to_string("input.txt")?.get_lines().lines_as_chars();

    let mut map: Vec<Vec<Item>> = chars
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| Item::new(x, y))
                .collect()
        })
        .collect();

    let groups = chars
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &character)| character != '.')
                .map(move |(x, &character)| (character, Item::new(x, y)))
        })
        .fold(
            HashMap::new(),
            |mut items: HashMap<char, Vec<Item>>, (key, item)| {
                items.entry(key).or_default().push(item);
                items
            },
        );

    let mut total_antinodes;

    total_antinodes = calculation(&mut map, &groups, false);
    println!("Part 1: {}", total_antinodes);
    total_antinodes += calculation(&mut map, &groups, true);
    println!("Part 2: {}", total_antinodes);

    Ok(())
}
