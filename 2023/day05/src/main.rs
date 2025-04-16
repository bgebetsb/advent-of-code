use std::{collections::VecDeque, fs::read_to_string, ops::Range};

#[derive(Debug, Clone)]
struct MapRange {
    input: Range<isize>,
    diff: isize,
}

fn part1(seeds: &[isize], ranges: &Vec<Vec<MapRange>>) -> isize {
    let mut current = seeds.to_owned();

    for range in ranges {
        for seed in &mut current {
            for current in range {
                if current.input.contains(seed) {
                    *seed += current.diff;
                    break;
                }
            }
        }
    }

    *current.iter().min().unwrap()
}

fn create_seed_ranges(seeds: &[isize]) -> VecDeque<Range<isize>> {
    let chunks = seeds.chunks(2);
    let mut ranges = VecDeque::new();

    for chunk in chunks {
        ranges.push_back(chunk[0]..(chunk[0] + chunk[1]));
    }

    ranges
}

fn part2(seeds: &[isize], ranges: &Vec<Vec<MapRange>>) -> isize {
    let mut seed_ranges = create_seed_ranges(seeds);

    for range in ranges {
        let mut new_seed_ranges = VecDeque::new();
        while let Some(current_range) = seed_ranges.pop_front() {
            let mut found = false;
            for range in range {
                let start = range.input.start.max(current_range.start);
                let end = range.input.end.min(current_range.end);
                if start >= end {
                    continue;
                }

                if start > current_range.start {
                    seed_ranges.push_back(current_range.start..start);
                }

                if end < current_range.end {
                    seed_ranges.push_back(end..current_range.end);
                }

                new_seed_ranges.push_back(start + range.diff..end + range.diff);
                found = true;
            }

            if !found {
                new_seed_ranges.push_back(current_range);
            }
        }
        seed_ranges = new_seed_ranges;
    }

    seed_ranges.iter().map(|range| range.start).min().unwrap()
}

fn parse_ranges<'a, I>(lines: &mut I) -> Vec<Vec<MapRange>>
where
    I: Iterator<Item = &'a str>,
{
    let mut skip_next = false;
    let mut sections = Vec::new();
    let mut current_section = Vec::new();

    for line in lines {
        if skip_next {
            skip_next = false;
            continue;
        }

        if line.is_empty() {
            if !current_section.is_empty() {
                sections.push(current_section);
                current_section = Vec::new();
            }
            skip_next = true;
        } else {
            let values: Vec<_> = line
                .split_whitespace()
                .map(|part| part.parse::<isize>().unwrap())
                .collect();

            current_section.push(MapRange {
                input: (values[1]..(values[1] + values[2])),
                diff: values[0] - values[1],
            })
        }
    }
    sections.push(current_section);

    sections
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|splitted| splitted.parse::<isize>().unwrap())
        .collect();

    let ranges = parse_ranges(&mut lines);
    let part1 = part1(&seeds, &ranges);
    let part2 = part2(&seeds, &ranges);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
