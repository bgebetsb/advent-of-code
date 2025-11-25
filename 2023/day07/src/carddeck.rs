use crate::cards::{Cards, CardsPartTwo, get_type, get_type_part_two};

#[derive(Debug, PartialEq, Eq)]
pub struct CardDeck(pub Vec<Cards>);

#[derive(Debug, PartialEq, Eq)]
pub struct CardDeckPartTwo(pub Vec<CardsPartTwo>);

impl Ord for CardDeck {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let own_type = get_type(&self.0);
        let other_type = get_type(&other.0);

        if own_type != other_type {
            return own_type.cmp(&other_type);
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return self.0[i].cmp(&other.0[i]);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for CardDeck {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardDeckPartTwo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let own_type = get_type_part_two(&self.0);
        let other_type = get_type_part_two(&other.0);

        if own_type != other_type {
            return own_type.cmp(&other_type);
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return self.0[i].cmp(&other.0[i]);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for CardDeckPartTwo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
