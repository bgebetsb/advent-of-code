use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    io,
};

use utils::string_handling::StringHandling;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Ord, PartialOrd)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        match self {
            Self::North | Self::South => false,
            Self::West | Self::East => true,
        }
    }
}

#[derive(Debug)]
struct FakeGrid {
    rows: BTreeMap<usize, BTreeSet<Coordinates>>,
    columns: BTreeMap<usize, BTreeSet<Coordinates>>,
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let input: Vec<_> = read_to_string(filename)?
        .get_lines()
        .iter()
        .map(|line| {
            let mut splitted = line.split(',');
            let x = splitted.next().unwrap().parse::<usize>().unwrap();
            let y = splitted.next().unwrap().parse::<usize>().unwrap();

            Coordinates { x, y }
        })
        .collect();

    let (p1_corner1, p1_corner2) = part1(&input);
    let part1 = (p1_corner2.x - p1_corner1.x + 1) * (p1_corner2.y - p1_corner1.y + 1);
    println!("Part 1: {}", part1);

    part2(&input, p1_corner1, p1_corner2);

    Ok(())
}

fn part1(input: &[Coordinates]) -> (&Coordinates, &Coordinates) {
    let mut sorted: Vec<_> = input.iter().map(|line| (line.x + line.y, line)).collect();
    sorted.sort_by_key(|value| value.0);

    let corner1 = sorted.first().unwrap().1;
    let corner2 = sorted.last().unwrap().1;

    (corner1, corner2)
}

fn part2(input: &[Coordinates], start: &Coordinates, end: &Coordinates) {
    let grid = create_fake_grid(input);

    let next = find_next_in_direction(&grid, start, Direction::East).unwrap();
    let path1 = path_search_1(&grid, next, end, Direction::East, vec![next]).unwrap();

    let next = find_next_in_direction(&grid, end, Direction::West).unwrap();
    let path2 = path_search_2(&grid, next, start, Direction::West, vec![next]).unwrap();

    let path: Vec<_> = [path1, path2].into_iter().flatten().collect();

    let mut max = 0;

    for element in path {
        for row in grid.rows.values() {
            for item in row.iter().filter(|item| item.x > element.x) {
                if part2_collision_check(&grid, element, item) {
                    let diff_x = item.x.max(element.x) - item.x.min(element.x) + 1;
                    let diff_y = item.y.max(element.y) - item.y.min(element.y) + 1;

                    if diff_x * diff_y > max {
                        max = diff_x * diff_y;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", max);
}

fn path_search_1<'a>(
    grid: &'a FakeGrid,
    current_position: &Coordinates,
    end: &Coordinates,
    current_direction: Direction,
    path: Vec<&'a Coordinates>,
) -> Option<Vec<&'a Coordinates>> {
    if current_position == end {
        return Some(path);
    }

    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    for new_direction in directions.iter().filter(|&item| {
        *item == current_direction || item.is_horizontal() != current_direction.is_horizontal()
    }) {
        if let Some(newpos) = find_next_in_direction(grid, current_position, *new_direction) {
            let mut new_path = path.clone();
            new_path.push(newpos);

            if let Some(path) = path_search_1(grid, newpos, end, *new_direction, new_path) {
                return Some(path);
            }
        }
    }

    None
}

fn path_search_2<'a>(
    grid: &'a FakeGrid,
    current_position: &Coordinates,
    end: &Coordinates,
    current_direction: Direction,
    path: Vec<&'a Coordinates>,
) -> Option<Vec<&'a Coordinates>> {
    if current_position == end {
        return Some(path);
    }

    let directions = [
        Direction::South,
        Direction::East,
        Direction::North,
        Direction::West,
    ];

    for new_direction in directions.iter().filter(|&item| {
        *item == current_direction || item.is_horizontal() != current_direction.is_horizontal()
    }) {
        if let Some(newpos) = find_next_in_direction(grid, current_position, *new_direction) {
            let mut new_path = path.clone();
            new_path.push(newpos);

            if let Some(path) = path_search_2(grid, newpos, end, *new_direction, new_path) {
                return Some(path);
            }
        }
    }

    None
}

fn find_next_in_direction<'a>(
    grid: &'a FakeGrid,
    pos: &Coordinates,
    direction: Direction,
) -> Option<&'a Coordinates> {
    if let Some(set) = match direction {
        Direction::North | Direction::South => grid.columns.get(&pos.x),
        Direction::West | Direction::East => grid.rows.get(&pos.y),
    } {
        let index = set.iter().position(|item| item == pos).unwrap();

        return match direction {
            Direction::North | Direction::West => {
                if index == 0 {
                    None
                } else {
                    set.iter().nth(index - 1)
                }
            }
            Direction::South | Direction::East => set.iter().nth(index + 1),
        };
    }

    None
}

fn create_fake_grid(input: &[Coordinates]) -> FakeGrid {
    let mut rows: BTreeMap<usize, BTreeSet<Coordinates>> = BTreeMap::new();
    let mut columns: BTreeMap<usize, BTreeSet<Coordinates>> = BTreeMap::new();

    for red_tile in input {
        if let Some(columns) = rows.get_mut(&red_tile.y) {
            columns.insert(red_tile.to_owned());
        } else {
            let mut set = BTreeSet::new();
            set.insert(red_tile.to_owned());
            rows.insert(red_tile.y, set);
        }

        if let Some(rows) = columns.get_mut(&red_tile.x) {
            rows.insert(red_tile.to_owned());
        } else {
            let mut set = BTreeSet::new();
            set.insert(red_tile.to_owned());
            columns.insert(red_tile.x, set);
        }
    }

    FakeGrid { rows, columns }
}

fn part2_collision_check(grid: &FakeGrid, a: &Coordinates, b: &Coordinates) -> bool {
    let x_min = a.x.min(b.x);
    let x_max = a.x.max(b.x);
    let y_min = a.y.min(b.y);
    let y_max = a.y.max(b.y);

    for (_, row) in grid.rows.iter().filter(|&(y, _)| *y > y_min && *y < y_max) {
        let first = row.first().unwrap();

        for current_item in row {
            if current_item.x > x_min && current_item.x < x_max {
                return false;
            }
        }
        let last = row.last().unwrap();

        if first.x <= x_min && last.x >= x_max {
            return false;
        }
    }

    for (_, column) in grid
        .columns
        .iter()
        .filter(|&(x, _)| *x > x_min && *x < x_max)
    {
        for c in column {
            if c.y > y_min && c.y < y_max {
                return false;
            }
        }
        let first = column.first().unwrap();
        let last = column.last().unwrap();

        if first.y <= y_min && last.y >= y_max {
            return false;
        }
    }

    true
}
