use std::collections::VecDeque;

#[derive(Debug)]
pub struct Machine {
    pub indicator_lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub _joltage: Vec<usize>,
}

impl TryFrom<String> for Machine {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splitted: VecDeque<_> = value
            .split_whitespace()
            .map(|item| item.replace(['[', ']', '(', ')', '{', '}'], ""))
            .collect();

        if splitted.len() < 3 {
            return Err("Need at least 3 Elements");
        }

        let indicator_lights: Vec<_> = splitted
            .pop_front()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();

        let joltage: Vec<_> = splitted
            .pop_back()
            .unwrap()
            .split(',')
            .filter_map(|item| item.parse::<usize>().ok())
            .collect();

        let buttons = splitted
            .iter()
            .map(|button| {
                button
                    .split(',')
                    .filter_map(|item| item.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        Ok(Self {
            indicator_lights,
            buttons,
            _joltage: joltage,
        })
    }
}
