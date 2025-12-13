use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Button {
    pub _index: usize,
    pub toggles: Vec<usize>,
    pub pressed: bool,
}

#[derive(Debug)]
pub struct Machine {
    pub indicator_lights: Vec<bool>,
    pub buttons: Vec<Button>,
    pub joltage: Vec<usize>,
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
            .enumerate()
            .map(|(index, button)| {
                let toggles = button
                    .split(',')
                    .filter_map(|item| item.parse::<usize>().ok())
                    .collect();

                Button {
                    _index: index,
                    toggles,
                    pressed: false,
                }
            })
            .collect();

        Ok(Self {
            indicator_lights,
            buttons,
            joltage,
        })
    }
}
