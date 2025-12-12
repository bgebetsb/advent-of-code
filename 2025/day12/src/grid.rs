use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct Grid {
    pub grid: Vec<Vec<char>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in &self.grid {
            let line: String = line.iter().collect();
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;

        Ok(())
    }
}

impl Grid {
    pub fn rotate(&self) -> Self {
        if self.grid.is_empty() {
            return self.clone();
        }

        let height = self.grid.len();
        let width = self.grid[0].len();

        let new_grid: Vec<Vec<char>> = (0..width)
            .map(|x| (0..height).rev().map(|y| self.grid[y][x]).collect())
            .collect();

        Self { grid: new_grid }
    }
}
