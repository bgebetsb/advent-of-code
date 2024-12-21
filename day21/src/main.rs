use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::io;
use utils::*;

fn find_shortest_paths(
    keypad: &Vec<[char; 3]>,
    y: usize,
    x: usize,
    character: char,
) -> Vec<(usize, usize, String)> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut results = Vec::new();

    queue.push_back((y, x, 0, String::new()));
    visited.insert((y, x), 0);

    while let Some((y, x, steps, path)) = queue.pop_front() {
        if keypad[y][x] == character {
            results.push((y, x, format!("{}A", path)));
            continue;
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

    results
}

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A'],
];

const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [['\0', '^', 'A'], ['<', 'v', '>']];

struct Robot {
    pos_y: usize,
    pos_x: usize,
    keypad: Vec<[char; 3]>,
    next: Option<Box<Robot>>,
}

impl Robot {
    fn new(numeric: bool, next: Option<Box<Robot>>) -> Self {
        let keypad = {
            if numeric {
                NUMERIC_KEYPAD.to_vec()
            } else {
                DIRECTIONAL_KEYPAD.to_vec()
            }
        };

        let (pos_y, pos_x) = if numeric { (3, 2) } else { (0, 2) };

        Self {
            pos_y,
            pos_x,
            keypad,
            next,
        }
    }

    fn calculate_steps(
        &mut self,
        input: &str,
        update_position: bool,
        cache: &mut HashMap<(Option<Vec<char>>, char, usize, usize), String>,
    ) -> String {
        let mut output = String::new();
        let chars = input.chars().collect::<Vec<char>>();
        let mut i = 0;

        let mut surrounding = None;
        if let Ok(offset) = i.offset(-5) {
            surrounding = Some(chars[offset..i].to_vec());
        }

        let (old_y, old_x) = (self.pos_y, self.pos_x);
        // let mut cache: HashMap<(Option<Vec<char>>, char, usize, usize), String> = HashMap::new();

        for c in input.chars() {
            if let Some(value) = cache.get(&(surrounding.clone(), c, self.pos_y, self.pos_x)) {
                if c == 'A' && self.keypad[self.pos_y][self.pos_x] == '<' {
                    output.push_str(">>^A");
                } else {
                    output.push_str(value);
                }
                self.update_position(c);
                i += 1;
                if let Ok(offset) = i.offset(-1) {
                    surrounding = Some(chars[offset..i].to_vec());
                } else {
                    surrounding = None;
                }
                continue;
            }
            let result = find_shortest_paths(&self.keypad, self.pos_y, self.pos_x, c);
            let mut lowest = result[0].2.clone();
            if let Some(ref mut next) = self.next {
                let mut lowest_length = next.calculate_steps(&lowest, false, cache).len();
                for item in result.iter().skip(1) {
                    let result = next.calculate_steps(&item.2, false, cache);
                    if result.len() < lowest_length {
                        lowest = item.2.clone();
                        lowest_length = result.len();
                    }
                }
            }
            cache.insert((surrounding, c, self.pos_y, self.pos_x), lowest.clone());
            i += 1;
            if let Ok(offset) = i.offset(-1) {
                surrounding = Some(chars[offset..i].to_vec());
            } else {
                surrounding = None;
            }
            self.update_position(c);
            output.push_str(&lowest);
        }

        if let Some(ref mut next) = self.next {
            return next.calculate_steps(&output, true, cache);
        }

        if update_position {
            (self.pos_y, self.pos_x) = (old_y, old_x);
        }
        output
    }

    fn update_position(&mut self, target_char: char) {
        for (y, row) in self.keypad.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == target_char {
                    self.pos_y = y;
                    self.pos_x = x;
                    return;
                }
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = read_to_string("example.txt")?.get_lines().lines_as_chars();

    let mut robo3 = Robot::new(false, None);
    let mut robo2 = Robot::new(false, Some(Box::new(robo3)));
    let mut robo = Robot::new(true, Some(Box::new(robo2)));
    let mut cache = HashMap::new();

    println!("EXAMPLES");
    println!(
        "Without Cache: {}",
        robo.calculate_steps("029A", true, &mut cache)
    );
    println!("{}", robo.calculate_steps("029A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("980A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("179A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("456A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("379A", true, &mut cache).len());

    println!("REAL");

    println!("{}", robo.calculate_steps("540A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("582A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("169A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("593A", true, &mut cache).len());
    println!("{}", robo.calculate_steps("579A", true, &mut cache).len());

    let mut robo26 = Robot::new(false, None);
    let mut robo25 = Robot::new(false, Some(Box::new(robo26)));
    let mut robo24 = Robot::new(false, Some(Box::new(robo25)));
    let mut robo23 = Robot::new(false, Some(Box::new(robo24)));
    let mut robo22 = Robot::new(false, Some(Box::new(robo23)));
    let mut robo21 = Robot::new(false, Some(Box::new(robo22)));
    let mut robo20 = Robot::new(false, Some(Box::new(robo21)));
    let mut robo19 = Robot::new(false, Some(Box::new(robo20)));
    let mut robo18 = Robot::new(false, Some(Box::new(robo19)));
    let mut robo17 = Robot::new(false, Some(Box::new(robo18)));
    let mut robo16 = Robot::new(false, Some(Box::new(robo17)));
    let mut robo15 = Robot::new(false, Some(Box::new(robo16)));
    let mut robo14 = Robot::new(false, Some(Box::new(robo15)));
    let mut robo13 = Robot::new(false, Some(Box::new(robo14)));
    let mut robo12 = Robot::new(false, Some(Box::new(robo13)));
    let mut robo11 = Robot::new(false, Some(Box::new(robo12)));
    let mut robo10 = Robot::new(false, Some(Box::new(robo11)));
    let mut robo9 = Robot::new(false, Some(Box::new(robo10)));
    let mut robo8 = Robot::new(false, Some(Box::new(robo9)));
    let mut robo7 = Robot::new(false, Some(Box::new(robo8)));
    let mut robo6 = Robot::new(false, Some(Box::new(robo7)));
    let mut robo5 = Robot::new(false, Some(Box::new(robo6)));
    let mut robo4 = Robot::new(false, Some(Box::new(robo5)));
    let mut robo3 = Robot::new(false, Some(Box::new(robo4)));
    let mut robo2 = Robot::new(false, Some(Box::new(robo3)));
    let mut robo = Robot::new(true, Some(Box::new(robo2)));

    let sum1 = robo.calculate_steps("540A", true, &mut cache).len();
    println!("Sum 1: {}", sum1);
    let sum2 = robo.calculate_steps("582A", true, &mut cache).len();
    println!("Sum 2: {}", sum2);
    let sum3 = robo.calculate_steps("169A", true, &mut cache).len();
    println!("Sum 3: {}", sum3);
    let sum4 = robo.calculate_steps("593A", true, &mut cache).len();
    println!("Sum 4: {}", sum4);
    let sum5 = robo.calculate_steps("579A", true, &mut cache).len();
    println!("Sum 5: {}", sum5);

    println!("Sum: {}", sum1 + sum2 + sum3 + sum4 + sum5);

    // println!();
    Ok(())
}
