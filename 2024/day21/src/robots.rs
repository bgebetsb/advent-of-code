use std::collections::HashMap;

use crate::keypad::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    y: usize,
    x: usize,
}

#[derive(Clone, Debug)]
pub struct Robot {
    pos: Pos,
    keypad: Keypad,
    cache: HashMap<(Pos, char), (usize, usize, usize)>,
}

impl Robot {
    pub fn new(keypad_type: KeypadType) -> Self {
        let keypad = Keypad::new(keypad_type);

        let pos = match keypad_type {
            KeypadType::Numeric => Pos { y: 3, x: 2 },
            KeypadType::Directional => Pos { y: 0, x: 2 },
        };

        Self {
            pos,
            keypad,
            cache: HashMap::new(),
        }
    }

    pub fn search_cached_value(&mut self, character: char) -> Option<usize> {
        let cached = self.cache.get(&(self.pos, character)).cloned();
        if let Some((value, y, x)) = cached {
            self.move_grid(y, x);
            return Some(value);
        }
        None
    }

    pub fn insert_in_cache(&mut self, character: char, result: usize, y: usize, x: usize) {
        self.cache.insert((self.pos, character), (result, y, x));
        self.move_grid(y, x);
    }

    pub fn find_paths(&self, target: char) -> (String, usize, usize) {
        self.keypad
            .get_shortest_paths(self.keypad.grid[self.pos.y][self.pos.x], target)
    }

    pub fn move_grid(&mut self, y: usize, x: usize) {
        self.pos.y = y;
        self.pos.x = x;
    }
}
