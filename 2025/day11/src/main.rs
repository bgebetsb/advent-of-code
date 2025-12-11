use std::{collections::HashMap, fs::read_to_string, io};

use utils::{string_handling::StringHandling, string_vec_handling::StringVecHandling};

fn main() -> Result<(), io::Error> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example.txt"
    } else {
        "input.txt"
    };

    let content: Vec<(String, Vec<String>)> = read_to_string(filename)?
        .get_lines()
        .split_with_key(':', &[' ']);

    let map: HashMap<String, Vec<String>> = content.into_iter().collect();

    let part1 = calc(&map, &mut HashMap::new(), "you");
    println!("Part 1: {}", part1);

    Ok(())
}

fn calc<'a>(
    map: &'a HashMap<String, Vec<String>>,
    cache: &mut HashMap<&'a str, usize>,
    item: &'a str,
) -> usize {
    if let Some(value) = cache.get(item) {
        return *value;
    }

    if item == "out" {
        return 1;
    }

    let mut total = 0;
    if let Some(conns) = map.get(item) {
        for conn in conns {
            total += calc(map, cache, conn);
        }
    }

    cache.insert(item, total);

    total
}

fn part2() -> Result<(), io::Error> {
    let filename = if cfg!(debug_assertions) {
        "example2.txt"
    } else {
        "input.txt"
    };

    let content: Vec<(String, Vec<String>)> = read_to_string(filename)?
        .get_lines()
        .split_with_key(':', &[' ']);

    let map: HashMap<String, Vec<String>> = content.into_iter().collect();

    let part2 = calc2(&map, &mut HashMap::new(), "svr", false, false);

    println!("Part 2: {}", part2);

    Ok(())
}

fn calc2<'a>(
    map: &'a HashMap<String, Vec<String>>,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
    item: &'a str,
    dac: bool,
    fft: bool,
) -> usize {
    if let Some(value) = cache.get(&(item, dac, fft)) {
        return *value;
    }

    if item == "out" {
        if dac && fft {
            return 1;
        } else {
            return 0;
        }
    }

    let mut dac = dac;
    let mut fft = fft;

    if item == "dac" {
        dac = true;
    }
    if item == "fft" {
        fft = true;
    }

    let mut total = 0;
    if let Some(conns) = map.get(item) {
        for conn in conns {
            total += calc2(map, cache, conn, dac, fft);
        }
    }

    cache.insert((item, dac, fft), total);

    total
}
