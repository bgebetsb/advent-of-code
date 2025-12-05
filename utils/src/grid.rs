use std::collections::{HashMap, VecDeque};

use crate::usize_offset::UsizeOffset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub trait GridHandling<T> {
    fn get_positions(&self, target: T) -> Vec<Position>;
}

impl<T> GridHandling<T> for Vec<Vec<T>>
where
    T: PartialEq,
{
    fn get_positions(&self, target: T) -> Vec<Position> {
        let mut positions = Vec::new();

        for (y, item) in self.iter().enumerate() {
            for (x, item) in item.iter().enumerate() {
                if *item == target {
                    positions.push(Position { x, y })
                }
            }
        }

        positions
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

pub struct GridSolver<T> {
    grid: Vec<Vec<T>>,
    obstacle: T,
    start_position: Position,
    end_position: Position,
    direction: Direction,
    cost_per_step: usize,
    cost_per_turn: usize,
}

impl<T> GridSolver<T>
where
    T: PartialEq,
{
    pub fn load_grid(
        grid: Vec<Vec<T>>,
        start_position: Position,
        end_position: Position,
        obstacle: T,
    ) -> Self {
        Self {
            grid,
            obstacle,
            start_position,
            end_position,
            direction: Direction::North,
            cost_per_step: 1,
            cost_per_turn: 0,
        }
    }

    pub fn start_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn cost_per_step(mut self, cost: usize) -> Self {
        self.cost_per_step = cost;
        self
    }

    pub fn cost_per_turn(mut self, cost: usize) -> Self {
        self.cost_per_turn = cost;
        self
    }

    pub fn lowest_cost(&self) -> (usize, Vec<Vec<Position>>) {
        let mut visited_cache: HashMap<(Position, Direction), usize> = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back((
            self.start_position,
            self.direction,
            0,
            vec![self.start_position],
        ));

        let mut best_cost = usize::MAX;
        let mut paths = Vec::new();

        while let Some((pos, cur_direction, cost, visited)) = queue.pop_front() {
            if let Some(prev_cost) = visited_cache.get(&(pos, cur_direction))
                && *prev_cost < cost
            {
                continue;
            }

            if cost > best_cost {
                continue;
            }

            if pos == self.end_position {
                if cost == best_cost {
                    paths.push(visited);
                } else if cost < best_cost {
                    paths = vec![visited];
                    best_cost = cost;
                }
                continue;
            }

            visited_cache.insert((pos, cur_direction), cost);
            let directions = [
                (Direction::West, [-1, 0]),
                (Direction::North, [0, -1]),
                (Direction::East, [1, 0]),
                (Direction::South, [0, 1]),
            ];

            for (direction, offset) in directions {
                let new_x = pos.x.offset(offset[0]);
                let new_y = pos.y.offset(offset[1]);

                if let (Ok(new_x), Ok(new_y)) = (new_x, new_y)
                    && new_x < self.grid[0].len()
                    && new_y < self.grid.len()
                    && self.grid[new_y][new_x] != self.obstacle
                {
                    let mut visited = visited.clone();
                    let newpos = Position { x: new_x, y: new_y };
                    visited.push(newpos);

                    let mut directions = [cur_direction, direction];
                    directions.sort();

                    let turns = match (direction == cur_direction, directions) {
                        (true, _) => 0,
                        (false, [Direction::North, Direction::South])
                        | (false, [Direction::West, Direction::East]) => 2,
                        _ => 1,
                    };

                    queue.push_back((
                        newpos,
                        direction,
                        cost + self.cost_per_step + self.cost_per_turn * turns,
                        visited,
                    ));
                }
            }
        }

        (best_cost, paths)
    }
}
