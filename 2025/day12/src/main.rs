mod grid;
mod shape;

use std::{
    fs::read_to_string,
    io::{self},
    iter::{Peekable, from_fn},
    str::Lines,
    time::Instant,
};

use crate::{
    grid::Grid,
    shape::{Shape, ShapeOrientations},
};

#[derive(Debug)]
struct Region {
    grid: Grid,
    shape_counts: Vec<usize>,
}

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?;
    let mut lines = content.lines().peekable();

    let shapes: Vec<_> = from_fn(|| get_shape(&mut lines)).collect();

    let mut part1 = 0;

    while let Some(region) = get_region(&mut lines).as_mut() {
        if calc(
            &mut region.grid,
            &shapes,
            &mut region.shape_counts,
            0,
            &Instant::now(),
            0,
            0,
        ) {
            part1 += 1;
        }
    }

    println!("Part 1: {}", part1);

    Ok(())
}

fn get_shape(iter: &mut Peekable<Lines>) -> Option<Vec<Shape>> {
    let mut grid = Vec::new();
    while let Some(line) = iter.next_if(|line| !line.contains('x')) {
        if line.contains(':') {
            continue;
        }

        if line.is_empty() {
            let mut grid = Grid { grid };
            let mut orientations = Vec::new();
            for _ in 0..4 {
                let orientation = Shape::from(&grid);
                if !orientations.contains(&orientation) {
                    orientations.push(orientation);
                }
                grid = grid.rotate();
            }
            return Some(orientations);
        }

        grid.push(line.chars().collect::<Vec<_>>());
    }

    None
}

fn get_region(iter: &mut Peekable<Lines>) -> Option<Region> {
    if let Some(line) = iter.next() {
        if line.is_empty() {
            return None;
        }

        let splitted: Vec<&str> = line.split(':').collect();
        let mut size = splitted[0].split('x');
        let width = size.next().unwrap().parse::<usize>().unwrap();
        let height = size.next().unwrap().parse::<usize>().unwrap();
        let shapes: Vec<usize> = splitted[1]
            .split_whitespace()
            .filter(|&item| !item.is_empty())
            .map(|item| item.parse::<usize>().unwrap())
            .collect();
        let grid: Vec<Vec<char>> = (0..height)
            .map(|_| (0..width).map(|_| '.').collect())
            .collect();

        return Some(Region {
            grid: Grid { grid },
            shape_counts: shapes,
        });
    }

    None
}

fn calc(
    grid: &mut Grid,
    shapes: &[ShapeOrientations],
    counts: &mut [usize],
    index: usize,
    start: &Instant,
    min_x: usize,
    min_y: usize,
) -> bool {
    if index < shapes.len() && counts[index] == 0 {
        return calc(grid, shapes, counts, index + 1, start, 0, 0);
    }

    if index == shapes.len() {
        return true;
    }

    if start.elapsed().as_millis() != 0 {
        return false;
    }

    let shape = &shapes[index];

    for orientation in shape {
        for y in min_y..=grid.grid.len() - orientation.height {
            let min_x = if y == min_y { min_x } else { 0 };
            for x in min_x..=grid.grid[y].len() - orientation.width {
                if placement_possible(grid, orientation, x, y) {
                    place_shape(grid, orientation, x, y, '#');
                    counts[index] -= 1;

                    if calc(grid, shapes, counts, index, start, x, y) {
                        return true;
                    }

                    counts[index] += 1;
                    place_shape(grid, orientation, x, y, '.');
                }
            }
        }
    }

    false
}

fn placement_possible(grid: &Grid, shape: &Shape, x: usize, y: usize) -> bool {
    for &(shape_x, shape_y) in &shape.coords {
        if grid.grid[y + shape_y][x + shape_x] == '#' {
            return false;
        }
    }
    true
}

fn place_shape(grid: &mut Grid, shape: &Shape, x: usize, y: usize, fill: char) {
    for &(shape_x, shape_y) in &shape.coords {
        grid.grid[y + shape_y][x + shape_x] = fill;
    }
}
