use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io;
use utils::*;

fn find_char_pos(map: &[Vec<char>], search: char) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == search {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_offsets_current_direction(direction: char) -> (isize, isize) {
    match direction {
        'N' => (-1, 0),
        'E' => (0, 1),
        'S' => (1, 0),
        'W' => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn get_other_directions(direction: char) -> [char; 2] {
    match direction {
        'N' | 'S' => ['W', 'E'],
        'W' | 'E' => ['N', 'S'],
        _ => panic!("Invalid direction"),
    }
}

fn search_path(
    map: &Vec<Vec<char>>,
    y: usize,
    x: usize,
    direction: char,
    cost: usize,
    cache: &mut HashMap<(usize, usize, char), usize>,
) -> Option<(usize, HashSet<(usize, usize)>)> {
    let horizontal_vertical = match direction {
        'N' | 'S' => 'V',
        'W' | 'E' => 'H',
        _ => panic!("Invalid direction"),
    };

    if map[y][x] == '#'
        || cache
            .get(&(y, x, horizontal_vertical))
            .is_some_and(|prevcost| *prevcost < cost)
    {
        return None;
    } else if map[y][x] == 'E' {
        return Some((cost, HashSet::from([(y, x)])));
    }

    cache.insert((y, x, horizontal_vertical), cost);

    let (y_offset, x_offset) = get_offsets_current_direction(direction);
    let mut lowest = search_path(
        map,
        y.offset(y_offset).unwrap(),
        x.offset(x_offset).unwrap(),
        direction,
        cost + 1,
        cache,
    );

    for direction in get_other_directions(direction) {
        if let Some(result) = search_path(map, y, x, direction, cost + 1000, cache) {
            match &mut lowest {
                Some(low) => match low.0.cmp(&result.0) {
                    std::cmp::Ordering::Greater => *low = result,
                    std::cmp::Ordering::Equal => low.1.extend(result.1),
                    std::cmp::Ordering::Less => (),
                },
                None => lowest = Some(result),
            }
        }
    }

    if let Some(low) = &mut lowest {
        low.1.extend([(y, x)]);
    }

    lowest
}

fn main() -> Result<(), io::Error> {
    let map = read_to_string("input.txt")?.get_lines().lines_as_chars();

    let (starty, startx) = find_char_pos(&map, 'S').unwrap();

    let mut cache: HashMap<(usize, usize, char), usize> = HashMap::new();

    let result = search_path(&map, starty, startx, 'E', 0, &mut cache).unwrap();
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1.len());

    Ok(())
}
