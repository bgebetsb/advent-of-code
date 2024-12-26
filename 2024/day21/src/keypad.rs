use std::collections::{HashMap, VecDeque};
use utils::UsizeOffset;

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A'],
];

const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [['\0', '^', 'A'], ['<', 'v', '>']];

#[derive(PartialEq, Copy, Clone)]
pub enum KeypadType {
    Numeric,
    Directional,
}

#[derive(Debug, Clone)]
pub struct Keypad {
    pub grid: Vec<[char; 3]>,
    lookup_table: HashMap<(char, char), (String, usize, usize)>,
}

impl Keypad {
    pub fn new(keypad_type: KeypadType) -> Self {
        let grid = match keypad_type {
            KeypadType::Directional => DIRECTIONAL_KEYPAD.to_vec(),
            KeypadType::Numeric => NUMERIC_KEYPAD.to_vec(),
        };

        let mut lookup_table = HashMap::new();
        let chars: Vec<char> = grid
            .as_flattened()
            .iter()
            .filter(|&c| *c != '\0')
            .copied()
            .collect();

        for (y, row) in grid.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                for c in &chars {
                    let paths = find_shortest_paths(&grid, y, x, *c);
                    lookup_table.insert((*field, *c), paths);
                }
            }
        }

        Self { grid, lookup_table }
    }

    pub fn get_shortest_paths(&self, from: char, to: char) -> (String, usize, usize) {
        self.lookup_table.get(&(from, to)).unwrap().clone()
    }
}

fn count_direction_changes(path: &str) -> usize {
    path.chars()
        .fold((0, None), |(count, prev), new| {
            if prev.is_some_and(|c| new != c) {
                (count + 1, Some(new))
            } else {
                (count, Some(new))
            }
        })
        .0
}

pub fn find_shortest_paths(
    keypad: &[[char; 3]],
    y: usize,
    x: usize,
    character: char,
) -> (String, usize, usize) {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let directions = [(0, -1), (-1, 0), (1, 0), (0, 1)];

    queue.push_back((y, x, 0, String::new()));
    visited.insert((y, x), 0);

    while let Some((y, x, steps, path)) = queue.pop_front() {
        if keypad[y][x] == character {
            return (format!("{}A", path), y, x);
        }

        for &(cur_y, cur_x) in &directions {
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y < keypad.len() && new_x < keypad[new_y].len() {
                    let steps = steps + 1;
                    let path = match (cur_y, cur_x) {
                        (0, -1) => format!("{}<", path),
                        (1, 0) => format!("{}v", path),
                        (0, 1) => format!("{}>", path),
                        (-1, 0) => format!("{}^", path),
                        _ => panic!("Invalid direction"),
                    };

                    if keypad[new_y][new_x] != '\0'
                        && count_direction_changes(&path) <= 1
                        && (!visited.contains_key(&(new_y, new_x))
                            || visited[&(new_y, new_x)] == steps)
                    {
                        visited.insert((new_y, new_x), steps);
                        queue.push_back((new_y, new_x, steps, path));
                    }
                }
            }
        }
    }
    panic!("No path found");
}
