use std::collections::HashMap;

use crate::keypad::*;

#[derive(Clone, Debug)]
pub struct Robot {
    pos_y: usize,
    pos_x: usize,
    keypad: Keypad,
    cache: HashMap<((usize, usize), char), (usize, usize, usize)>,
}

impl Robot {
    pub fn new(keypad_type: KeypadType) -> Self {
        let keypad = Keypad::new(keypad_type);

        let (pos_y, pos_x) = match keypad_type {
            KeypadType::Numeric => (3, 2),
            KeypadType::Directional => (0, 2),
        };

        Self {
            pos_y,
            pos_x,
            keypad,
            cache: HashMap::new(),
        }
    }

    pub fn search_cached_value(&mut self, character: char) -> Option<usize> {
        let cached = self.cache.get(&(self.get_pos(), character)).cloned();
        if let Some((value, y, x)) = cached {
            self.move_grid(y, x);
            return Some(value);
        }
        None
    }

    pub fn insert_in_cache(&mut self, character: char, result: usize, y: usize, x: usize) {
        self.cache
            .insert((self.get_pos(), character), (result, y, x));
        self.move_grid(y, x);
    }

    pub fn find_paths(&self, target: char) -> (String, usize, usize) {
        self.keypad
            .get_shortest_paths(self.keypad.grid[self.pos_y][self.pos_x], target)
    }

    pub fn move_grid(&mut self, y: usize, x: usize) {
        self.pos_y = y;
        self.pos_x = x;
    }

    pub fn get_pos(&self) -> (usize, usize) {
        (self.pos_y, self.pos_x)
    }
}
