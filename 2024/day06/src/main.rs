use std::fs::read_to_string;
use std::io;
use utils_2024::*;

#[derive(PartialEq, Clone, Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct MapField {
    field_type: char,
    visited: Vec<Directions>,
}

impl MapField {
    fn new(field_type: &char) -> Self {
        MapField {
            field_type: *field_type,
            visited: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum MapResult {
    Infinite,
    Finished(usize, Vec<(usize, usize)>),
}

fn get_start_pos(map: &[Vec<MapField>]) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, field) in line.iter().enumerate() {
            if field.field_type == '^' {
                return Some((i, j));
            }
        }
    }
    None
}

fn run_simulation(og_map: &[Vec<MapField>]) -> MapResult {
    let mut map = og_map.to_owned();
    let (starty, startx) = get_start_pos(&map).expect("Unable to find start position");
    let mut direction = Directions::North;

    let mut y = starty;
    let mut x = startx;
    let mut fields_visited_count = 0;
    let mut visited_fields = Vec::new();
    loop {
        if map[y][x].visited.contains(&direction) {
            return MapResult::Infinite;
        }

        if map[y][x].visited.is_empty() {
            fields_visited_count += 1;
            if y != starty || x != startx {
                visited_fields.push((y, x));
            }
        }

        map[y][x].visited.push(direction.clone());
        if y == 0 || x == 0 || y == map.len() - 1 || x == map[y].len() - 1 {
            return MapResult::Finished(fields_visited_count, visited_fields);
        }

        match direction {
            Directions::North => {
                if map[y - 1][x].field_type == '#' {
                    direction = Directions::East;
                } else {
                    y -= 1;
                }
            }
            Directions::East => {
                if map[y][x + 1].field_type == '#' {
                    direction = Directions::South;
                } else {
                    x += 1;
                }
            }
            Directions::South => {
                if map[y + 1][x].field_type == '#' {
                    direction = Directions::West;
                } else {
                    y += 1;
                }
            }
            _ => {
                if map[y][x - 1].field_type == '#' {
                    direction = Directions::North;
                } else {
                    x -= 1;
                }
            }
        }
    }
}

fn convert_input(input: &[Vec<char>]) -> Vec<Vec<MapField>> {
    input
        .iter()
        .map(|item| item.iter().map(MapField::new).collect())
        .collect()
}

fn main() -> Result<(), io::Error> {
    let chars = read_to_string("example.txt")?.get_lines().lines_as_chars();
    let mut map = convert_input(&chars);

    let result = run_simulation(&map);

    if let MapResult::Finished(value, visited_fields) = result {
        println!("Part 1: {}", value);
        let mut infinite_count = 0;
        for field in visited_fields {
            map[field.0][field.1].field_type = '#';
            if run_simulation(&map) == MapResult::Infinite {
                infinite_count += 1;
            }
            map[field.0][field.1].field_type = '.';
        }
        println!("Part 2: {}", infinite_count);
    }

    Ok(())
}
