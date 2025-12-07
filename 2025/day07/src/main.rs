use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io,
};

use utils::{
    grid::{GridHandling, Position},
    string_handling::StringHandling,
    string_vec_handling::StringVecHandling,
};

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines().lines_as_chars();
    let start = content.get_positions('S')[0];

    let part1 = part1(&content, start.x);
    println!("Part 1: {}", part1);

    let part2 = 1 + part2(&content, &mut HashMap::new(), start);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(grid: &[Vec<char>], start_x: usize) -> usize {
    let mut total = 0;

    let mut unique_x = HashSet::new();
    unique_x.insert(start_x);

    for line in grid {
        let mut new_unique_x = HashSet::new();

        for item in &unique_x {
            if line[*item] == '^' {
                total += 1;
                if *item > 0 {
                    new_unique_x.insert(*item - 1);
                }

                if item + 1 < line.len() {
                    new_unique_x.insert(item + 1);
                }
            } else {
                new_unique_x.insert(*item);
            }
        }

        unique_x = new_unique_x;
    }

    total
}

fn part2(grid: &[Vec<char>], cache: &mut HashMap<Position, usize>, pos: Position) -> usize {
    if let Some(value) = cache.get(&pos) {
        return *value;
    }

    let x = pos.x;
    let mut y = pos.y;

    while y + 1 < grid.len() && grid[y + 1][x] == '.' {
        y += 1;
    }

    if y + 1 == grid.len() {
        return 0;
    }

    let mut total = 0;
    let left_poss = x > 0;
    let right_poss = x + 1 < grid[y].len();

    if left_poss && right_poss && grid[y + 1][x - 1] == '.' && grid[y + 1][x + 1] == '.' {
        total += 1;
    }

    if left_poss {
        total += part2(grid, cache, Position { x: x - 1, y: y + 1 });
    }

    if right_poss {
        total += part2(grid, cache, Position { x: x + 1, y: y + 1 });
    }

    cache.insert(pos, total);

    total
}
