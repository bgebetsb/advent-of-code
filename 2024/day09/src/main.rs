use std::fs::read_to_string;
use std::io;
use std::mem;
use utils_2024::*;

#[derive(Clone)]
enum ItemType {
    File(usize, usize),
    Space(usize),
}

fn create_items_part1(chars: &[char]) -> Vec<ItemType> {
    chars
        .iter()
        .enumerate()
        .flat_map(|(total, c)| {
            let amount = c.to_digit(10).unwrap();
            (0..amount).map(move |_| {
                if total % 2 == 0 {
                    ItemType::File(total / 2, 1)
                } else {
                    ItemType::Space(1)
                }
            })
        })
        .collect()
}

fn create_items_part2(chars: &[char]) -> Vec<ItemType> {
    chars
        .iter()
        .enumerate()
        .map(|(total, c)| {
            let amount = c.to_digit(10).unwrap();
            if total % 2 == 0 {
                ItemType::File(total / 2, amount as usize)
            } else {
                ItemType::Space(amount as usize)
            }
        })
        .collect()
}

fn move_one_block(items: &mut [ItemType]) {
    for i in (0..items.len()).rev() {
        for j in 0..i {
            if let (&ItemType::File(_, _), &ItemType::Space(_)) = (&items[i], &items[j]) {
                items.swap(i, j);
                break;
            }
        }
    }
}

fn move_whole_items(items: &mut Vec<ItemType>) {
    for i in (0..items.len()).rev() {
        for j in 0..i {
            if let (&ItemType::File(_, filesize), &ItemType::Space(spacesize)) =
                (&items[i], &items[j])
            {
                if spacesize < filesize {
                    continue;
                }
                let sizediff = spacesize - filesize;
                let originalfile = mem::replace(&mut items[i], ItemType::Space(filesize));
                items[j] = originalfile;
                if sizediff > 0 {
                    items.insert(j + 1, ItemType::Space(sizediff));
                }
                break;
            }
        }
    }
}

fn calculate_result(items: &[ItemType]) -> usize {
    let mut result = 0;
    let mut realindex = 0;

    for item in items {
        if let ItemType::File(index, width) = item {
            for _ in 0..*width {
                result += realindex * index;
                realindex += 1;
            }
        } else if let ItemType::Space(width) = item {
            realindex += width;
        }
    }
    result
}

fn main() -> Result<(), io::Error> {
    let chars = read_to_string("input.txt")?.get_chars_trimmed();

    let mut part1_items = create_items_part1(&chars);
    move_one_block(&mut part1_items);
    println!("Part 1: {}", calculate_result(&part1_items));

    let mut part2_items = create_items_part2(&chars);
    move_whole_items(&mut part2_items);
    println!("Part 2: {}", calculate_result(&part2_items));

    Ok(())
}
