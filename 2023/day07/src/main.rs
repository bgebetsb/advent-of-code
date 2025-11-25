mod carddeck;
mod cards;

use std::{fs::read_to_string, io};

use utils::string_handling::StringHandling;

use crate::{
    carddeck::{CardDeck, CardDeckPartTwo},
    cards::{Cards, CardsPartTwo},
};

fn main() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content = read_to_string(filename)?.get_lines();

    let mut input: Vec<_> = content
        .iter()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            let cards = parts[0].chars().map(Cards::from).collect();
            (CardDeck(cards), parts[1].parse::<usize>().unwrap())
        })
        .collect();

    input.sort_by(|a, b| b.cmp(a));

    let part1 = input
        .iter()
        .enumerate()
        .fold(0, |total, (index, (_, stake))| {
            total + ((input.len() - index) * stake)
        });

    println!("{}", part1);

    let mut input_part2: Vec<_> = content
        .iter()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            let cards = parts[0].chars().map(CardsPartTwo::from).collect();
            (CardDeckPartTwo(cards), parts[1].parse::<usize>().unwrap())
        })
        .collect();

    input_part2.sort_by(|a, b| b.cmp(a));

    let part2 = input_part2
        .iter()
        .enumerate()
        .fold(0, |total, (index, (_, stake))| {
            total + ((input_part2.len() - index) * stake)
        });

    println!("Part 2: {:?}", part2);

    Ok(())
}
