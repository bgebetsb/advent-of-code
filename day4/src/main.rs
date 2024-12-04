use std::error::Error;
use utils::read_characters;

enum Directions {
    Left,
    Right,
    Up,
    Down,
    DiagonalTopLeftBottomRight,
    DiagonalTopRightBottomLeft,
    DiagonalBottomRightTopLeft,
    DiagonalBottomLeftTopRight
}

fn search_string(input: &Vec<Vec<u8>>, direction: Directions, i: usize, j: usize, search: &[u8], sum: &mut u32) {
    if search == &[] {
        *sum += 1;
        return;
    }

    match direction {
        Directions::Left => {
            if j != 0 && input[i][j - 1] == search[0] {
                search_string(input, direction, i, j - 1, &search[1..], sum);
            }
        },
        Directions::Right => {
            if j != input[i].len() - 1 && input[i][j + 1] == search[0] {
                search_string(input, direction, i, j + 1, &search[1..], sum);
            }
        },
        Directions::Up => {
            if i != 0 && input[i - 1][j] == search[0] {
                search_string(input, direction, i - 1, j, &search[1..], sum);
            }
        },
        Directions::Down => {
            if i != input.len() - 1 && input[i + 1][j] == search[0] {
                search_string(input, direction, i + 1, j, &search[1..], sum);
            }
        },
        Directions::DiagonalTopLeftBottomRight => {
            if i != input.len() - 1 && j != input.len() - 1 && input[i + 1][j + 1] == search[0] {
                search_string(input, direction, i + 1, j + 1, &search[1..], sum);
            }
        },
        Directions::DiagonalTopRightBottomLeft => {
            if i != input[i].len() - 1 && j != 0 && input[i + 1][j - 1] == search[0] {
                search_string(input, direction, i + 1, j - 1, &search[1..], sum);
            }
        },
        Directions::DiagonalBottomLeftTopRight => {
            if i != 0 && j != input[i].len() - 1 && input[i - 1][j + 1] == search[0] {
                search_string(input, direction, i - 1, j + 1, &search[1..], sum);
            }
        }
        Directions::DiagonalBottomRightTopLeft => {
            if i != 0 && j != 0 && input[i - 1][j - 1] == search[0] {
                search_string(input, direction, i - 1, j - 1, &search[1..], sum);
            }
        }
    }
} 

fn main() -> Result<(), Box<dyn Error>> {
    let chars = read_characters("input.txt")?;

    let mut total = 0;
    let haystack = "MAS".as_bytes();

    let mut total_2 = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == b'X' {
                search_string(&chars, Directions::Left, i, j, haystack, &mut total);
                search_string(&chars, Directions::Right, i, j, haystack, &mut total);
                search_string(&chars, Directions::Up, i, j, haystack, &mut total);
                search_string(&chars, Directions::Down, i, j, haystack, &mut total);
                search_string(&chars, Directions::DiagonalTopLeftBottomRight, i, j, haystack, &mut total);
                search_string(&chars, Directions::DiagonalTopRightBottomLeft, i, j, haystack, &mut total);
                search_string(&chars, Directions::DiagonalBottomLeftTopRight, i, j, haystack, &mut total);
                search_string(&chars, Directions::DiagonalBottomRightTopLeft, i, j, haystack, &mut total);
            } else if chars[i][j] == b'A' && i != 0 && j != 0 && i != chars.len() - 1 && j != chars[i].len() - 1 {
                if ((chars[i - 1][j - 1] == b'M' && chars[i + 1][j + 1] == b'S')
                    || (chars[i - 1][j - 1] == b'S' && chars[i + 1][j + 1] == b'M'))
                    && ((chars[i + 1][j - 1] == b'M' && chars[i - 1][j + 1] == b'S')
                    || ((chars[i - 1][j + 1] == b'M' && chars[i + 1][j - 1] == b'S')))
                {
                    total_2 += 1;
                }
            }
        }
    }

    println!("Part 1: {total}");
    println!("Part 2: {total_2}");

    Ok(())
}
