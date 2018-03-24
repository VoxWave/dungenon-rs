use tile::Tile;
use util::{Error, Grid};
use Vector;
use std::default::Default;

#[derive(Clone)]
pub struct GridLevel<T> {
    tiles: Grid<T>,
}

impl<T> GridLevel<T> {
    pub fn get_width(&self) -> usize {
        self.tiles.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.tiles.get_height()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<&T, Error> {
        if x < self.get_width() && y < self.get_height() {
            Ok(&self.tiles[(x, y)])
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }

    pub fn get_tile_with_vec(&self, pos: &Vector<usize>) -> Result<&T, Error> {
        self.get_tile(pos.x, pos.y)
    }

    pub fn get_tile_with_tuple(&self, (x, y): (usize, usize)) -> Result<&T, Error> {
        self.get_tile(x, y)
    }

    pub fn get_mut_tile(&mut self, x: usize, y: usize) -> Result<&mut T, Error> {
        if x < self.get_width() && y < self.get_height() {
            Ok(&mut self.tiles[(x, y)])
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }

    pub fn get_mut_tile_with_vec(&mut self, pos: &Vector<usize>) -> Result<&mut T, Error> {
        self.get_mut_tile(pos.x, pos.y)
    }

    pub fn get_mut_tile_with_tuple(&mut self, (x, y): (usize, usize)) -> Result<&mut T, Error> {
        self.get_mut_tile(x, y)
    }

    pub fn apply<F>(&mut self, gen: F) -> &mut GridLevel<T>
    where
        F: FnOnce(&mut GridLevel<T>),
    {
        gen(self);
        self
    }
}

impl<T: Default + Clone> GridLevel<T> {
    pub fn fill_with(&mut self, tile: T) {
        let width = self.tiles.get_width();
        let height = self.tiles.get_height();

        for x in 0..width {
            for y in 0..height {
                self.tiles[(x, y)] = tile.clone();
            }
        }
    }

    pub fn new(width: usize, height: usize) -> GridLevel<T> {
        GridLevel {
            tiles: Grid::new(width, height),
        }
    }

    pub fn new_filled_with(tile: T, width: usize, height: usize) -> GridLevel<T> {
        GridLevel {
            tiles: Grid::new_filled_with(tile, width, height),
        }
    }
}

pub fn fill_dead_end_tiles(level: &mut GridLevel<Tile>) -> bool {
    let mut deadends = Vec::new();
    for y in 0..level.get_height() {
        for x in 0..level.get_width() {
            if let Ok(n) = level.get_tile(x, y) {
                if let &Tile::Floor(_) = n {
                    if is_deadend(level, x, y) {
                        deadends.push((x, y));
                    }
                }
            }
        }
    }
    let mut filled_deadend = false;
    for &(x, y) in &deadends {
        if let Ok(tile) = level.get_mut_tile(x, y) {
            *tile = Tile::Wall(0);
            filled_deadend = true;
        }
    }
    filled_deadend
}

pub fn is_deadend(level: &GridLevel<Tile>, x: usize, y: usize) -> bool {
    use util::Direction;
    let mut paths = 0;
    for dir in Direction::get_orthogonal_dirs() {
        let vector = dir.get_vec();
        let coord = match (
            add_isize_to_usize(vector.x, x),
            add_isize_to_usize(vector.y, y),
        ) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };

        if let Ok(&Tile::Floor(_)) = level.get_tile_with_tuple(coord) {
            paths += 1;
        }
    }
    paths < 2
}

pub fn add_isize_to_usize(i: isize, mut u: usize) -> Option<usize> {
    if i < 0 && u != 0 {
        u -= (-i) as usize;
    } else if i >= 0 && u < usize::max_value() {
        u += i as usize;
    } else {
        return None;
    }
    Some(u)
}
