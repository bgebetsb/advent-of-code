use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    io::Error,
};
use utils::*;

#[derive(Clone, PartialEq)]
enum MapField {
    Space,
    Corrupted,
}

fn search_path(map: &[Vec<MapField>], y: usize, x: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    queue.push_back((y, x, 0));
    visited.insert((y, x));
    while let Some((y, x, steps)) = queue.pop_front() {
        if y == map.len() - 1 && x == map[y].len() - 1 {
            return Some(steps);
        }

        for &(cur_y, cur_x) in &directions {
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y == map.len() || new_x == map[new_y].len() {
                    continue;
                }
                if !visited.contains(&(new_y, new_x)) && map[new_y][new_x] == MapField::Space {
                    visited.insert((new_y, new_x));
                    queue.push_back((new_y, new_x, steps + 1));
                }
            }
        }
    }
    None
}

fn parse_line(line: &str) -> [usize; 2] {
    let parts: Vec<usize> = line
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    [parts[0], parts[1]]
}

fn part1(map: &mut [Vec<MapField>], tiles: &[[usize; 2]]) -> Option<usize> {
    for tile in tiles.iter().take(1024) {
        map[tile[1]][tile[0]] = MapField::Corrupted;
    }
    search_path(map, 0, 0)
}

fn part2(map: &mut [Vec<MapField>], tiles: &[[usize; 2]]) -> Option<(usize, usize)> {
    for tile in tiles {
        map[tile[1]][tile[0]] = MapField::Corrupted;
        if search_path(map, 0, 0).is_none() {
            return Some((tile[0], tile[1]));
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let tiles = read_to_string("input.txt")?
        .get_lines()
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<[usize; 2]>>();

    let (max_x, max_y) = tiles.iter().fold((0, 0), |(max_x, max_y), &[x, y]| {
        (max_x.max(x), max_y.max(y))
    });

    let width = max_x + 1;
    let height = max_y + 1;

    let map = vec![vec![MapField::Space; width]; height];

    if let Some(steps) = part1(&mut map.clone(), &tiles) {
        println!("Part 1: {}", steps);
    }

    if let Some((x, y)) = part2(&mut map.clone(), &tiles) {
        println!("Part 2: {},{}", x, y);
    }

    Ok(())
}
