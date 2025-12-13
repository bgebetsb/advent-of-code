mod machine;

use std::{fs::read_to_string, io};

use utils::string_handling::StringHandling;

use crate::machine::{Button, Machine};

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content: Vec<_> = read_to_string(filename)?.get_lines();

    let machines: Vec<_> = content
        .into_iter()
        .map(|line| Machine::try_from(line).unwrap())
        .collect();

    let part1: usize = machines.iter().map(part1).sum();

    println!("Part 1: {}", part1);

    let mut part2_result = 0;
    for machine in &machines {
        if let Some(result) = part2(
            &mut machine.buttons.clone(),
            &mut machine.joltage.clone(),
            1,
            0,
        ) {
            part2_result += result;
        } else {
            panic!("No solution found");
        }
    }
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(machine: &Machine) -> usize {
    let mut initial_indicators: Vec<_> =
        (0..machine.indicator_lights.len()).map(|_| false).collect();

    calc(
        &machine.indicator_lights,
        &mut initial_indicators,
        &mut machine.buttons.clone(),
        0,
    )
    .unwrap()
}

fn part2(
    buttons: &mut Vec<Button>,
    joltage: &mut Vec<usize>,
    factor: usize,
    total: usize,
) -> Option<usize> {
    if joltage.iter().sum::<usize>() == 0 {
        return Some(total);
    }

    let mut lowest: Option<usize> = None;
    let mut solutions = Vec::new();

    calc_part2(buttons, joltage, 0, &mut solutions);

    for pressed_buttons in &solutions {
        for button in pressed_buttons {
            for toggle in &button.toggles {
                joltage[*toggle] -= 1;
            }
        }

        for joltage in joltage.iter_mut() {
            *joltage /= 2;
        }

        if let Some(amount) = part2(
            buttons,
            joltage,
            factor * 2,
            total + (pressed_buttons.len() * factor),
        ) {
            lowest = Some(lowest.map_or(amount, |lowest| lowest.min(amount)));
        }

        for joltage in joltage.iter_mut() {
            *joltage *= 2;
        }

        for button in pressed_buttons {
            for toggle in &button.toggles {
                joltage[*toggle] += 1;
            }
        }
    }

    lowest
}

fn calc(
    target_indicators: &[bool],
    current_indicators: &mut [bool],
    buttons: &mut Vec<Button>,
    button_index: usize,
) -> Option<usize> {
    if button_index == buttons.len() {
        let pressed: usize = buttons
            .iter()
            .filter_map(|button| {
                if button.pressed {
                    Some(button.to_owned())
                } else {
                    None
                }
            })
            .count();

        if target_indicators == current_indicators {
            return Some(pressed);
        } else {
            return None;
        }
    }

    let mut lowest: Option<usize> = None;
    if let Some(amount) = calc(
        target_indicators,
        current_indicators,
        buttons,
        button_index + 1,
    ) {
        lowest = Some(amount);
    }

    for toggle in &buttons[button_index].toggles {
        current_indicators[*toggle] = !current_indicators[*toggle];
    }
    buttons[button_index].pressed = true;

    if let Some(amount) = calc(
        target_indicators,
        current_indicators,
        buttons,
        button_index + 1,
    ) {
        lowest = Some(lowest.map_or(amount, |lowest| lowest.min(amount)));
    }

    for toggle in &buttons[button_index].toggles {
        current_indicators[*toggle] = !current_indicators[*toggle];
    }
    buttons[button_index].pressed = false;

    lowest
}

fn calc_part2(
    buttons: &mut [Button],
    joltage: &mut [usize],
    button_index: usize,
    found_possibilities: &mut Vec<Vec<Button>>,
) {
    if button_index == buttons.len() {
        let pressed: Vec<_> = buttons
            .iter()
            .filter_map(|button| {
                if button.pressed {
                    Some(button.to_owned())
                } else {
                    None
                }
            })
            .collect();

        if joltage.iter().all(|&joltage| joltage % 2 == 0) {
            found_possibilities.push(pressed);
        }
        return;
    }

    calc_part2(buttons, joltage, button_index + 1, found_possibilities);

    let limit_reached = buttons[button_index]
        .toggles
        .iter()
        .any(|toggle| joltage[*toggle] == 0);

    if !limit_reached {
        for toggle in &buttons[button_index].toggles {
            joltage[*toggle] -= 1;
        }
        buttons[button_index].pressed = true;

        calc_part2(buttons, joltage, button_index + 1, found_possibilities);

        for toggle in &buttons[button_index].toggles {
            joltage[*toggle] += 1;
        }
        buttons[button_index].pressed = false;
    }
}
