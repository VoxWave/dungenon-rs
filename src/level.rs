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
