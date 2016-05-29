use tile::Tile;
use util::Grid;
use std::ops::{Index};
use na::Vec2;

pub struct Level {
    tiles: Grid<Option<Tile>>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Level {
        Level{
            tiles: Grid::new_filled_with(Some(Tile::Void) ,width, height),
        }
    }

    pub fn new_filled_with(tile: Option<Tile>, width: usize, height: usize) -> Level {
        Level{
            tiles: Grid::new_filled_with(tile, width, height),
        }
    }

    pub fn fill_with(&mut self, tile: Tile) {
        let width = self.tiles.get_width();
        let height = self.tiles.get_height();

        for x in 0..width {
            for y in 0..height {
                self.tiles[(x, y)] = Some(tile.clone());
            }
        }
    }

    pub fn get_width(&self) -> usize {
        self.tiles.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.tiles.get_height()
    }

    pub fn get_mut_tile(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.get_width() && y < self.get_height() {
            self.tiles[(x, y)].as_mut()
        } else {
            None
        }
    }

    pub fn get_mut_tile_with_vec(&mut self, pos: &Vec2<usize>) -> Option<&mut Tile> {
        self.get_mut_tile(pos.x, pos.y)
    }

    pub fn apply<F>(&mut self, gen: F) -> &mut Level where F: FnOnce(&mut Level) {
        gen(self);
        self
    }

}

static NONE: Option<Tile> = None;

impl Index<(usize, usize)> for Level {
    type Output= Option<Tile>;

    fn index(&self, (x, y): (usize, usize)) -> &Option<Tile>{
        if x < self.get_width() && y < self.get_height() {
            &self.tiles[(x, y)]
        } else {
            &NONE
        }
    }
}

impl Index<Vec2<usize>> for Level {
    type Output= Option<Tile>;

    fn index(&self, vec: Vec2<usize>) -> &Option<Tile>{
        if vec.x < self.get_width() && vec.y < self.get_height() {
            &self.tiles[(vec.x, vec.y)]
        } else {
            &NONE
        }
    }
}

pub fn fill_dead_ends(level: &mut Level) -> bool {
    let mut deadends = Vec::new();
    for y in 0..level.get_height() {
        for x in 0..level.get_width() {
            if let Some(ref n) = level[(x,y)] {
                if let &Tile::Floor = n {
                    if is_deadend(level, x, y) {
                        deadends.push((x,y));
                    }
                }
            }
        }
    }

    for &(x,y) in &deadends {
        if let Some(tile) = level.get_mut_tile(x, y) {
            *tile = Tile::Floor;
        }
    }
    deadends.is_empty()
}

pub fn is_deadend(level: &Level, x: usize, y: usize) -> bool {
    use util::Direction;
    let mut paths = 0;
    for dir in Direction::get_orthogonal_dirs() {
        let vector = dir.get_vec();
        let coord = match (add_isize_to_usize(vector.x, x), add_isize_to_usize(vector.y, y)) {
            (Some(x), Some(y)) => (x,y),
            _ => continue,
        };

        if let Some(Tile::Floor) = level[coord] {
            paths += 1;
        }
	}
    paths < 2
}

fn add_isize_to_usize(i: isize, mut u: usize,) -> Option<usize> {
    if i < 0 && u != 0 {
        u -= (-i) as usize;
    } else if i >= 0 && u < usize::max_value() {
        u += i as usize;
    } else {
        return None
    }
    Some(u)
}
