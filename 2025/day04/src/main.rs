use std::{fs::read_to_string, io};

use utils::{
    string_handling::StringHandling, string_vec_handling::StringVecHandling,
    usize_offset::UsizeOffset,
};

fn calc(grid: &mut [Vec<char>], replace: bool) -> usize {
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }

            let mut current = 0;
            let offsets = [
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1],
            ];

            for offset in offsets {
                if let Ok(y) = y.offset(offset[0])
                    && let Ok(x) = x.offset(offset[1])
                    && y < grid.len()
                    && x < grid[y].len()
                    && grid[y][x] == '@'
                {
                    current += 1;
                }
            }

            if current < 4 {
                if replace {
                    grid[y][x] = 'x';
                }
                total += 1;
            }
        }
    }

    total
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let mut content = read_to_string(filename)?.get_lines().lines_as_chars();

    let part1 = calc(&mut content.clone(), false);
    let mut part2 = 0;

    while let current = calc(&mut content, true)
        && current > 0
    {
        part2 += current;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
