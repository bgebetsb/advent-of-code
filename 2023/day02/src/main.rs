fn process_line(line: &str) -> (bool, usize) {
    let content = line.split(':').nth(1).unwrap();

    let blocks: Vec<_> = content
        .split([';', ','])
        .map(|block| block.trim())
        .collect();

    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
    for block in blocks {
        let input: Vec<_> = block.split(' ').collect();
        let amount: usize = input[0].parse().unwrap();
        let color = input[1];
        match color {
            "blue" => {
                max_blue = max_blue.max(amount);
            }
            "red" => {
                max_red = max_red.max(amount);
            }
            "green" => {
                max_green = max_green.max(amount);
            }
            _ => panic!("Invalid input"),
        }
    }

    let possible = max_red <= 12 && max_green <= 13 && max_blue <= 14;

    (possible, max_red * max_green * max_blue)
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    for (nbr, line) in content.lines().enumerate() {
        let (possible, part2_result) = process_line(line);

        if possible {
            part1 += nbr + 1;
        }

        part2 += part2_result;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
