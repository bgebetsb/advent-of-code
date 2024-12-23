#[derive(Debug, PartialEq)]
pub struct Connection<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

impl Connection<'_> {
    pub fn connected(&self, other: &str) -> bool {
        self.from == other || self.to == other
    }

    pub fn three_connected<'a>(
        &'a self,
        second: &Connection<'a>,
        third: &Connection<'a>,
    ) -> Option<[&'a str; 3]> {
        let mut items = [
            self.from,
            self.to,
            second.from,
            second.to,
            third.from,
            third.to,
        ];
        items.sort_unstable();
        for item in items.chunks_exact(2) {
            if item[0] != item[1] {
                return None;
            }
        }
        let result = [items[0], items[2], items[4]];
        if result[0] == result[1] || result[1] == result[2] {
            return None;
        }
        Some(result)
    }
}

impl<'a> From<&'a str> for Connection<'a> {
    fn from(value: &'a str) -> Self {
        let splitted: Vec<&str> = value.split("-").collect();

        Self {
            from: splitted[0],
            to: splitted[1],
        }
    }
}
