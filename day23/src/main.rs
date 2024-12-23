use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    io::Error,
};
mod connection;
use crate::connection::*;

fn part1(connections: &[Connection]) -> usize {
    let filtered = connections
        .iter()
        .filter(|conn| conn.from.starts_with("t") || conn.to.starts_with("t"));

    let mut results = HashSet::new();
    for filteredconn in filtered {
        for conn2 in connections {
            if !conn2.connected(filteredconn.from) || filteredconn == conn2 {
                continue;
            }
            for conn3 in connections {
                if !conn3.connected(filteredconn.to) {
                    continue;
                }
                if let Some(result) = filteredconn.three_connected(conn2, conn3) {
                    results.insert(result);
                }
            }
        }
    }
    results.len()
}

fn most_visited<'a>(connections: &'a [Connection], current: &'a Connection) -> Vec<&'a str> {
    let mut counter = HashMap::new();
    let mut queue = VecDeque::new();
    let mut iterations = 0;

    queue.push_back(current.from);
    queue.push_back(current.to);
    while let Some(computer) = queue.pop_front() {
        let value = counter.entry(computer).or_insert(0);
        if iterations > 250 {
            break;
        }
        *value += 1;
        iterations += 1;

        let conns = connections.iter().filter_map(|conn| {
            if conn.from == computer {
                Some(conn.to)
            } else if conn.to == computer {
                Some(conn.from)
            } else {
                None
            }
        });

        for conn in conns {
            queue.push_back(conn);
        }
    }

    let mut entries: Vec<_> = counter.clone().into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));

    let mostvisited: Vec<&str> = entries
        .iter()
        .filter_map(|&(key, value)| if value > 10 { Some(key) } else { None })
        .collect();

    mostvisited
}

fn validate_visited<'a>(connections: &'a [Connection], visited: &[&'a str]) -> Vec<&'a str> {
    let mut validated: Vec<&str> = Vec::new();
    for &result in visited {
        for &validateditem in &validated {
            let conns = connections
                .iter()
                .filter(|conn| {
                    (conn.from == result || conn.to == result)
                        && (conn.from == validateditem || conn.to == validateditem)
                })
                .count();
            if conns == 0 {
                return validated;
            }
        }
        validated.push(result);
    }
    validated
}

fn part2(connections: &[Connection]) -> Option<String> {
    let mut counter = HashMap::new();

    for connection in connections {
        counter.insert(connection.from, 0);
        counter.insert(connection.to, 0);
    }

    for conn in connections {
        let mostvisited = most_visited(connections, conn);
        if mostvisited.len() >= 13 {
            let mut validated = validate_visited(connections, &mostvisited);

            if validated.len() > 12 {
                validated.sort();
                let result = validated.join(",");
                return Some(result);
            }
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?;
    let connections: Vec<Connection> = input.lines().map(|line| line.into()).collect();

    let part1 = part1(&connections);
    println!("Part 1: {}", part1);

    let part2 = part2(&connections).unwrap();
    println!("Part 2: {}", part2);

    Ok(())
}
