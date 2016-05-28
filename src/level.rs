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
            match level[(x,y)] {
                Some(n) => {
                    match n {
                        Tile::Floor => {
                            if is_deadend(level, x, y) {
                                deadends.push((x,y));
                            }
                        },
                        _ => {},
                    }
                },
                None => {},
            }
        }
    }

    for (x,y) in deadends {
        match level.get_mut_tile(x, y) {
            Some(tile) => {
                *tile = Tile::Floor;
            }
        }
    }
    deadends.is_empty()
}

pub fn is_deadend(level: &mut Level, x: usize, y: usize) -> bool {
    use util::Direction;
    let mut paths = 0;
    for dir in Direction::get_orthogonal_dirs() {
        
        match level[&Direction::get_vec(dir)] {
            Some(tile) => {
                match tile {
                    Tile::Floor => paths+=1,
                    _ => {},
                }
            }
            None => {}
        }
	}
    paths < 2
}
