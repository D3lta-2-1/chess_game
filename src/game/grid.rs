pub const BOARD_SIZE: usize = 8;

pub struct Grid<T> {
    grid: [[T; BOARD_SIZE]; BOARD_SIZE]
}

impl<T: Copy> Grid<T> {
    pub fn splat(value: T) -> Self {
        Self {
            grid: [[value; BOARD_SIZE]; BOARD_SIZE]
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self{
            grid: self.grid.clone(),
        }
    }
}

impl<T: Copy> Copy for Grid<T> {}

impl<T> Grid<T> {
    pub fn from(grid: [[T; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self {
            grid
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        let ys = self.grid.iter().enumerate();
        let xs = ys.map(|(y, xs)| xs.iter().enumerate().map(move |(x, piece)| (x, y, piece) ));
        xs.flatten()
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[y][x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.grid[y][x]
    }
}