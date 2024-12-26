use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::io;
use utils::*;

#[derive(Debug, PartialEq)]
enum MapField {
    Space(Option<usize>),
    Wall,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<MapField>>,
    start: Option<(usize, usize)>,
    exit: Option<(usize, usize)>,
}

fn init_map(lines: &[&str]) -> Map {
    let mut start = None;
    let mut exit = None;

    let grid = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Some((y, x));
                        MapField::Space(None)
                    }
                    'E' => {
                        exit = Some((y, x));
                        MapField::Space(Some(0))
                    }
                    '.' => MapField::Space(None),
                    '#' => MapField::Wall,
                    _ => panic!("Invalid map field"),
                })
                .collect()
        })
        .collect();

    Map { grid, start, exit }
}

fn set_finish_distances(map: &mut Map) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    if let Some((y, x)) = map.exit {
        queue.push_back((y, x, 0));
        visited.insert((y, x));
        while let Some((y, x, steps)) = queue.pop_front() {
            map.grid[y][x] = MapField::Space(Some(steps));
            if Some((y, x)) == map.start {
                return Some(steps);
            }
            for &(cur_y, cur_x) in &directions {
                if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                    if new_y >= map.grid.len() || new_x >= map.grid[new_y].len() {
                        continue;
                    }
                    if !visited.contains(&(new_y, new_x))
                        && map.grid[new_y][new_x] != MapField::Wall
                    {
                        visited.insert((new_y, new_x));
                        queue.push_back((new_y, new_x, steps + 1));
                    }
                }
            }
        }
    }
    None
}

fn get_fields_to_check(
    grid: &[Vec<MapField>],
    y: usize,
    x: usize,
    duration: usize,
) -> HashSet<(usize, usize)> {
    let mut fields = HashSet::new();

    for i in 0..=duration {
        let j = (duration - i) as isize;
        let i = i as isize;
        let offsets = [(i, j), (i, -j), (-j, i), (-j, -i)];
        for offset in offsets {
            if let (Ok(y), Ok(x)) = (y.offset(offset.0), x.offset(offset.1)) {
                if y >= grid.len() || x >= grid[y].len() {
                    continue;
                }
                fields.insert((y, x));
            }
        }
    }

    fields
}

fn find_cheat_possibilities(map: &Map, duration: usize, max: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut total = 0;

    let (start_y, start_x) = map.start.unwrap();
    queue.push_back((start_y, start_x, 0));
    visited.insert((start_y, start_x));
    while let Some((y, x, steps)) = queue.pop_front() {
        for i in 2..=duration {
            let fields = get_fields_to_check(&map.grid, y, x, i);
            for (y, x) in fields {
                if let MapField::Space(Some(cost)) = map.grid[y][x] {
                    if steps + i + cost <= max {
                        total += 1;
                    }
                }
            }
        }
        for &(cur_y, cur_x) in &directions {
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y == map.grid.len() || new_x == map.grid[new_y].len() {
                    continue;
                }
                if !visited.contains(&(new_y, new_x)) && map.grid[new_y][new_x] != MapField::Wall {
                    visited.insert((new_y, new_x));
                    queue.push_back((new_y, new_x, steps + 1));
                }
            }
        }
    }
    total
}

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut map = init_map(&lines);

    let steps_without_cheat = set_finish_distances(&mut map).unwrap();

    let part1 = find_cheat_possibilities(&map, 2, steps_without_cheat - 100);
    println!("Part 1: {}", part1);

    let part2 = find_cheat_possibilities(&map, 20, steps_without_cheat - 100);
    println!("Part 2: {}", part2);

    Ok(())
}
