#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Types {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum CardsPartTwo {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Cards {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

impl From<char> for CardsPartTwo {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

pub fn get_type(input: &[Cards]) -> Types {
    let mut sorted = input.to_vec();
    sorted.sort();

    let mut pair1 = 0;
    let mut pair2 = 0;

    let mut current_count = 1;
    let mut prev = sorted[0];

    for i in 1..sorted.len() {
        if sorted[i] == prev {
            current_count += 1;
        }

        if sorted[i] != prev || i + 1 == sorted.len() {
            if current_count > 1 {
                if pair1 != 0 {
                    pair2 = current_count;
                } else {
                    pair1 = current_count;
                }
            }
            current_count = 1;
            prev = sorted[i];
        }
    }

    let mut pairs = [pair1, pair2];
    pairs.sort();

    match pairs {
        [_, 5] => Types::FiveOfAKind,
        [_, 4] => Types::FourOfAKind,
        [2, 3] => Types::FullHouse,
        [_, 3] => Types::ThreeOfAKind,
        [2, 2] => Types::TwoPair,
        [_, 2] => Types::OnePair,
        [_, _] => Types::HighCard,
    }
}

pub fn get_type_part_two(input: &[CardsPartTwo]) -> Types {
    let jacks_count = input
        .iter()
        .filter(|&card| *card == CardsPartTwo::Jack)
        .count();

    if jacks_count >= 4 {
        return Types::FiveOfAKind;
    }

    let mut sorted: Vec<_> = input
        .iter()
        .filter(|&card| *card != CardsPartTwo::Jack)
        .collect();
    sorted.sort();

    let mut current_count = 1;
    let mut prev = sorted[0];

    let mut pairs = vec![0];
    for i in 1..sorted.len() {
        if sorted[i] == prev {
            current_count += 1;
        }

        if sorted[i] != prev {
            pairs.push(current_count);
            current_count = 1;
            prev = sorted[i];
        }

        if i + 1 == sorted.len() {
            pairs.push(current_count);
        }
    }

    pairs.sort_by(|a, b| b.cmp(a));

    let pairs = [pairs[0] + jacks_count, pairs[1]];

    match pairs {
        [5, _] => Types::FiveOfAKind,
        [4, _] => Types::FourOfAKind,
        [3, 2] => Types::FullHouse,
        [3, _] => Types::ThreeOfAKind,
        [2, 2] => Types::TwoPair,
        [2, _] => Types::OnePair,
        [_, _] => Types::HighCard,
    }
}
