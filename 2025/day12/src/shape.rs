use crate::grid::Grid;

#[derive(PartialEq, Debug, Clone)]
pub struct Shape {
    pub coords: Vec<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

impl From<&Grid> for Shape {
    fn from(value: &Grid) -> Self {
        let coords: Vec<(usize, usize)> = value
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, field)| if *field == '#' { Some((x, y)) } else { None })
            })
            .collect();

        let (max_x, max_y) = coords.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });

        Self {
            coords,
            width: max_x + 1,
            height: max_y + 1,
        }
    }
}

pub type ShapeOrientations = Vec<Shape>;
