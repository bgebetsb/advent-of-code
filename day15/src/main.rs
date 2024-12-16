use std::fs::read_to_string;
use std::io;
use utils::*;

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '@' {
                return Some((i, j));
            }
        }
    }
    None
}

fn part1(map: &mut [Vec<char>], moves: &[char]) -> usize {
    let (mut y, mut x) = find_start(map).unwrap();
    for instruction in moves {
        match instruction {
            '<' => {
                move_horizontal(map, y, &mut x, -1);
            }
            '>' => {
                move_horizontal(map, y, &mut x, 1);
            }
            '^' => {
                if check_vertical(map, y, x, -1) {
                    move_vertical(map, y, x, -1);
                    y -= 1;
                }
            }
            'v' => {
                if check_vertical(map, y, x, 1) {
                    move_vertical(map, y, x, 1);
                    y += 1;
                }
            }
            _ => panic!("Unknown move"),
        }
    }

    calculate_total(map)
}

fn check_vertical(map: &mut [Vec<char>], y: usize, x: usize, offset: isize) -> bool {
    let y = y.offset(offset).unwrap();

    match map[y][x] {
        '[' => check_vertical(map, y, x, offset) && check_vertical(map, y, x + 1, offset),
        ']' => check_vertical(map, y, x, offset) && check_vertical(map, y, x - 1, offset),
        'O' => check_vertical(map, y, x, offset),
        '.' => true,
        _ => false,
    }
}

fn move_vertical(map: &mut [Vec<char>], y: usize, x: usize, offset: isize) {
    let old_y = y;
    let y = y.offset(offset).unwrap();

    match map[y][x] {
        '[' => {
            move_vertical(map, y, x, offset);
            move_vertical(map, y, x + 1, offset);
        }
        ']' => {
            move_vertical(map, y, x - 1, offset);
            move_vertical(map, y, x, offset);
        }
        'O' => move_vertical(map, y, x, offset),
        _ => {}
    }

    map[y][x] = map[old_y][x];
    map[old_y][x] = '.';
}

fn move_horizontal(map: &mut [Vec<char>], y: usize, x: &mut usize, offset: isize) {
    let mut tmp_x = x.offset(offset).unwrap();

    let chars = ['O', '[', ']'];
    while chars.contains(&map[y][tmp_x]) {
        tmp_x = tmp_x.offset(offset).unwrap();
    }

    if map[y][tmp_x] != '.' {
        return;
    }

    while tmp_x != *x {
        map[y][tmp_x] = map[y][tmp_x.offset(-offset).unwrap()];
        tmp_x = tmp_x.offset(-offset).unwrap();
        map[y][tmp_x] = '.';
    }
    *x = x.offset(offset).unwrap()
}

fn calculate_total(map: &[Vec<char>]) -> usize {
    let mut total = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '[' || *c == 'O' {
                total += i * 100 + j;
            }
        }
    }

    total
}

fn part2(map: &mut [Vec<char>], moves: &[char]) -> usize {
    let (mut y, mut x) = find_start(map).unwrap();
    for instruction in moves {
        match instruction {
            '<' => {
                move_horizontal(&mut *map, y, &mut x, -1);
            }
            '>' => {
                move_horizontal(&mut *map, y, &mut x, 1);
            }
            '^' => {
                if check_vertical(map, y, x, -1) {
                    move_vertical(&mut *map, y, x, -1);
                    y -= 1;
                }
            }
            'v' => {
                if check_vertical(map, y, x, 1) {
                    move_vertical(&mut *map, y, x, 1);
                    y += 1;
                }
            }
            _ => panic!("Unknown move"),
        }
    }
    calculate_total(map)
}

fn part2_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();

    for line in map {
        let mut map_line = Vec::new();
        for c in line {
            match c {
                '#' => map_line.extend_from_slice(&['#', '#']),
                'O' => map_line.extend_from_slice(&['[', ']']),
                '.' => map_line.extend_from_slice(&['.', '.']),
                '@' => map_line.extend_from_slice(&['@', '.']),
                _ => panic!("Unknown character"),
            }
        }
        new_map.push(map_line);
    }

    new_map
}

fn main() -> Result<(), io::Error> {
    let input: Vec<Vec<String>> = read_to_string("input.txt")?
        .get_lines()
        .split(|line| line.is_empty())
        .map(|chunk| chunk.to_vec())
        .collect();

    let map = input[0].lines_as_chars();
    let moves = input[1]
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let part1 = part1(&mut map.clone(), &moves);
    println!("Part 1: {}", part1);

    let mut part2_map = part2_map(&map);
    let part2 = part2(&mut part2_map, &moves);
    println!("Part 2: {}", part2);

    Ok(())
}
