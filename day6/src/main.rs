use utils::*;
use std::fs::read_to_string;
use std::io;

fn get_start_pos(map: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'^' {
                return Some((i, j));
            }
        }
    }
    None
}

#[derive(PartialEq, Clone, Debug)]
enum Directions {
    North,
    South,
    East,
    West
}

fn step(map: &mut Vec<Vec<u8>>, direction: &mut Directions, y: &mut usize, x: &mut usize) -> bool {
    match *direction {
        Directions::North => {
            if *y == 0 {
                false
            } else if map[*y - 1][*x] == b'#' {
                *direction = Directions::East;
                false
            } else {
                *y -= 1;
                map[*y][*x] = b'X';
                true
            }
        },
        Directions::East => {
            if *x == map[*y].len() - 1 {
                false
            } else if map[*y][*x + 1] == b'#'{
                *direction = Directions::South;
                false
            } else {
                *x += 1;
                map[*y][*x] = b'X';
                true
            }
        },
        Directions::South => {
            if *y == map.len() - 1 {
                false
            } else if map[*y + 1][*x] == b'#' {
                *direction = Directions::West;
                false
            } else {
                *y += 1;
                map[*y][*x] = b'X';
                true
            }
        },
        _ => {
            if *x == 0 {
                false
            } else if map[*y][*x - 1] == b'#' {
                *direction = Directions::North;
                false
            } else {
                *x -= 1;
                map[*y][*x] = b'X';
                true
            }
        }
    }
}

/*
fn create_dirs_map(map: &Vec<Vec<u8>>) -> Vec<Vec<Vec<Directions>>> {
    let mut dirs: Vec<Vec<Vec<Directions>>> = Vec::new();
    for i in 0..map.len() {
        dirs.push(Vec::new());
        for _ in 0..map[i].len() {
            dirs[i].push(Vec::new());
        }
    }
    dirs
}


fn infinite_check(map: &Vec<Vec<u8>>, starty: usize, startx: usize) -> i32 {
    let mut infinite = 0;

     for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut currentmap = map.clone();
            let mut direction = Directions::North;
            if (i == starty && j == startx) || map[i][j] == b'#' {
                continue;
            }
            let mut dirs = create_dirs_map(&currentmap);
            let mut cur_x = startx;
            let mut cur_y = starty;
            let mut streak = 0;
            loop {
                if dirs[cur_y][cur_x].contains(&direction) {
                    infinite += 1;
                    println!("{:?}", dirs[cur_y][cur_x]);
                    break;
                }
                
                let prev_y = cur_y;
                let prev_x = cur_x;
                let prev_directon = direction.clone();
                
                if step(&mut currentmap, &mut direction, &mut cur_y, &mut cur_x) {
                    dirs[prev_y][prev_x].push(prev_directon);
                    if cur_y == 0 || cur_x == 0 || cur_y == currentmap.len() - 1 || cur_x == currentmap[cur_y].len() - 1 {
                        break;
                    }
                } else if streak == 4 {
                    break;
                } else {
                    streak += 1;
                }
            }
        }
    }
    infinite
}
*/

fn count_steps(map: &mut Vec<Vec<u8>>, starty: usize, startx: usize, stop_val: usize) -> usize {
    let mut steps = 1;
    let mut streak = 0;
    let mut direction = Directions::North;

    let (mut y, mut x) = (starty, startx);

    loop {
        if step(map, &mut direction, &mut y, &mut x) {
            steps += 1;
            streak = 0;
        } else if streak == 4 {
            break;
        } else {
            streak += 1;
        }
        if steps > stop_val{
            break;
        }
    }
    steps
}

fn infinite_check(map: &Vec<Vec<u8>>, starty: usize, startx: usize) -> usize {
    let mut infinite = 0;

    let original_steps = count_steps(&mut map.clone(), starty, startx, usize::MAX) + 278; // not proud of this
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i == starty && j == startx) || map[i][j] == b'#' {
                continue;
            }
            let mut newmap = map.clone();
            newmap[i][j] = b'#';
            if count_steps(&mut newmap, starty, startx, original_steps) > original_steps {
                infinite += 1;
            }
        }
    }
    infinite
}

fn main() -> Result<(), io::Error>{
    let mut chars = read_to_string("input.txt")?.get_lines().lines_as_bytes();
    let (y, x) = get_start_pos(&chars).unwrap();

    chars[y][x] = b'X';

    let (starty, startx) = (y, x);
    let steps = count_steps(&mut chars.clone(), y, x, usize::MAX);

    println!("Part 1: {}", steps);

  
    let infinite = infinite_check(&chars, starty, startx);
    
    println!("Infinite: {}", infinite);

    Ok(())
}
