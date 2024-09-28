use crate::game::grid::Grid;

pub struct Selection {
    pub x: usize,
    pub y: usize,
    pub choice: PossibleChoice,
}

impl Selection {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            choice: PossibleChoice::new()
        }
    }
}

pub struct PossibleChoice {
    grid: Grid<bool>
}

impl PossibleChoice {
    pub fn new() -> Self {
        Self {
            grid: Grid::splat(false)
        }
    }

    pub fn add(&mut self, x: i32, y: i32) {
        *self.grid.get_mut(x as usize, y as usize) = true;
    }

    pub fn is_available(&self, x: usize, y: usize) -> bool {
        *self.grid.get(x, y)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &bool)> {
        self.grid.iter()
    }
}