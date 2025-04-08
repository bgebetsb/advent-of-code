fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;

    for (nbr, line) in content.lines().enumerate() {
        let content = line.split(':').skip(1).next().unwrap();
        let games: Vec<_> = content.split(';').collect();
        let mut possible = true;
        for game in games {
            let blocks: Vec<_> = game.split(',').map(|block| block.trim()).collect();
            for block in blocks {
                let input: Vec<_> = block.split(' ').collect();
                let amount: i32 = input[0].parse().unwrap();
                let color = input[1];
                let max = match color {
                    "blue" => 14,
                    "red" => 12,
                    "green" => 13,
                    _ => i32::MAX,
                };

                if amount > max {
                    possible = false;
                }
            }
        }

        if possible {
            total += nbr + 1;
        }
    }

    println!("{}", total);
}
