use std::fs::read_to_string;
use std::io;
use utils::*;

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                return Some((i, j));
            }
        }
    }
    None
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn part1(map: &mut [Vec<char>], moves: &[char]) -> usize {
    let (mut y, mut x) = find_start(&map).unwrap();
    for instruction in moves {
        match instruction {
            '<' => {
                if map[y][x - 1] == '.' {
                    map[y][x - 1] = '@';
                    map[y][x] = '.';
                    x -= 1;
                } else if map[y][x - 1] == 'O' {
                    let mut posx = x - 1;
                    while map[y][posx] == 'O' {
                        posx -= 1;
                    }
                    if map[y][posx] == '.' {
                        while posx < x {
                            map[y][posx] = map[y][posx + 1];
                            map[y][posx + 1] = '.';
                            posx += 1;
                        }
                        x -= 1;
                    }
                }
            }
            '>' => {
                if map[y][x + 1] == '.' {
                    map[y][x + 1] = '@';
                    map[y][x] = '.';
                    x += 1;
                } else if map[y][x + 1] == 'O' {
                    let mut posx = x + 1;
                    while map[y][posx] == 'O' {
                        posx += 1;
                    }
                    if map[y][posx] == '.' {
                        while posx > x {
                            map[y][posx] = map[y][posx - 1];
                            map[y][posx - 1] = '.';
                            posx -= 1;
                        }
                        x += 1;
                    }
                }
            }
            '^' => {
                if map[y - 1][x] == '.' {
                    map[y - 1][x] = '@';
                    map[y][x] = '.';
                    y -= 1;
                } else if map[y - 1][x] == 'O' {
                    let mut posy = y - 1;
                    while map[posy][x] == 'O' {
                        posy -= 1;
                    }
                    if map[posy][x] == '.' {
                        while posy < y {
                            map[posy][x] = map[posy + 1][x];
                            map[posy + 1][x] = '.';
                            posy += 1;
                        }
                        y -= 1;
                    }
                }
            }
            'v' => {
                if map[y + 1][x] == '.' {
                    map[y + 1][x] = '@';
                    map[y][x] = '.';
                    y += 1;
                } else if map[y + 1][x] == 'O' {
                    let mut posy = y + 1;
                    while map[posy][x] == 'O' {
                        posy += 1;
                    }
                    if map[posy][x] == '.' {
                        while posy > y {
                            map[posy][x] = map[posy - 1][x];
                            map[posy - 1][x] = '.';
                            posy -= 1;
                        }
                        y += 1;
                    }
                }
            }
            _ => panic!("Unknown move"),
        }
    }
    let mut total = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                total += i * 100 + j;
            }
        }
    }

    total
}

fn p2_check_up(map: &mut [Vec<char>], y: usize, x: usize) -> bool {
    if map[y - 1][x] == '.' {
        return true;
    }

    if map[y - 1][x] == '[' {
        let left = p2_check_up(map, y - 1, x);
        let right = p2_check_up(map, y - 1, x + 1);
        return left && right;
    } else if map[y - 1][x] == ']' {
        let left = p2_check_up(map, y - 1, x - 1);
        let right = p2_check_up(map, y - 1, x);
        return left && right;
    }
    false
}

fn p2_move_up(map: &mut [Vec<char>], y: usize, x: usize) {
    if map[y - 1][x] == '[' {
        p2_move_up(map, y - 1, x);
        p2_move_up(map, y - 1, x + 1);
    } else if map[y - 1][x] == ']' {
        p2_move_up(map, y - 1, x - 1);
        p2_move_up(map, y - 1, x);
    }

    if map[y - 1][x] == '.' {
        map[y - 1][x] = map[y][x];
        map[y][x] = '.';
        return;
    }
}

fn p2_check_down(map: &mut [Vec<char>], y: usize, x: usize) -> bool {
    if map[y + 1][x] == '.' {
        return true;
    }

    if map[y + 1][x] == '[' {
        let left = p2_check_down(map, y + 1, x);
        let right = p2_check_down(map, y + 1, x + 1);
        return left && right;
    } else if map[y + 1][x] == ']' {
        let left = p2_check_down(map, y + 1, x - 1);
        let right = p2_check_down(map, y + 1, x);
        return left && right;
    }
    false
}

fn p2_move_down(map: &mut [Vec<char>], y: usize, x: usize) {
    if map[y + 1][x] == '[' {
        p2_move_down(map, y + 1, x);
        p2_move_down(map, y + 1, x + 1);
    } else if map[y + 1][x] == ']' {
        p2_move_down(map, y + 1, x - 1);
        p2_move_down(map, y + 1, x);
    }

    if map[y + 1][x] == '.' {
        map[y + 1][x] = map[y][x];
        map[y][x] = '.';
        return;
    }
}

fn part2(map: &mut [Vec<char>], moves: &[char]) -> usize {
    let (mut y, mut x) = find_start(&map).unwrap();
    for instruction in moves {
        match instruction {
            '<' => {
                if map[y][x - 1] == '.' {
                    map[y][x - 1] = '@';
                    map[y][x] = '.';
                    x -= 1;
                } else if map[y][x - 1] == '[' || map[y][x - 1] == ']' {
                    let mut posx = x - 1;
                    while map[y][posx] == '[' || map[y][posx] == ']' {
                        posx -= 1;
                    }
                    if map[y][posx] == '.' {
                        while posx < x {
                            map[y][posx] = map[y][posx + 1];
                            map[y][posx + 1] = '.';
                            posx += 1;
                        }
                        x -= 1;
                    }
                }
            }
            '>' => {
                if map[y][x + 1] == '.' {
                    map[y][x + 1] = '@';
                    map[y][x] = '.';
                    x += 1;
                } else if map[y][x + 1] == '[' || map[y][x + 1] == ']' {
                    let mut posx = x + 1;
                    while map[y][posx] == '[' || map[y][posx] == ']' {
                        posx += 1;
                    }
                    if map[y][posx] == '.' {
                        while posx > x {
                            map[y][posx] = map[y][posx - 1];
                            map[y][posx - 1] = '.';
                            posx -= 1;
                        }
                        x += 1;
                    }
                }
            }
            '^' => {
                if p2_check_up(map, y, x) {
                    p2_move_up(&mut *map, y, x);
                    y -= 1;
                }
            }
            'v' => {
                if p2_check_down(map, y, x) {
                    p2_move_down(&mut *map, y, x);
                    y += 1;
                }
            }
            _ => panic!("Unknown move"),
        }
    }
    let mut total = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '[' {
                total += i * 100 + j;
            }
        }
    }

    total
}

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?.get_lines();
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut map_complete = false;
    for line in input {
        if line.is_empty() {
            map_complete = true;
        } else if !map_complete {
            map.push(line);
        } else {
            moves.push(line);
        }
    }

    let map = map.lines_as_chars();
    let moves: Vec<char> = moves
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let total = part1(&mut map.clone(), &moves);
    println!("{}", total);

    let mut part2_map = Vec::new();
    for i in 0..map.len() {
        let mut part2_line = Vec::new();
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                part2_line.push('#');
                part2_line.push('#');
            } else if map[i][j] == 'O' {
                part2_line.push('[');
                part2_line.push(']');
            } else if map[i][j] == '.' {
                part2_line.push('.');
                part2_line.push('.');
            } else if map[i][j] == '@' {
                part2_line.push('@');
                part2_line.push('.');
            } else {
                panic!("Unknown character");
            }
        }
        part2_map.push(part2_line);
    }
    print_map(&part2_map);
    let total = part2(&mut part2_map, &moves);
    print_map(&part2_map);
    println!("Total: {}", total);

    Ok(())
}
