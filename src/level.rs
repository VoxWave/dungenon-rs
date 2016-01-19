use tile::Tile;
use util::Grid;
use std::ops::{Index};

pub struct Level {
	tiles: Grid<Option<Tile>>,
}

impl Level {
	pub fn new(width: usize, height: usize) -> Level {
		Level{
			tiles: Grid::new_filled_with(Some(Tile::Void) ,width, height),
		}
	}

	pub fn new_filled_with(tile: Option<Tile>, width: usize, height: usize) -> level {
		Level{
			tiles: Grid::new_filled_with(tile, width, height),
		}
	}

	pub fn fill_with(&mut self, tile: Tile) {
		let width = tiles.get_width();
		let height = tiles.get_height();

		for x in 0..width {
			for y in 0..height {
				self.tiles[(x, y)] = tile;
			}
		}
	}

	pub fn get_width(&self) -> usize {
		self.tiles.get_width()
	}

	pub fn get_height(&self) -> usize {
		self.tiles.get_height()
	}

	pub fn get_mut_tile(&mut self, width: usize, height: usize) -> Option<&mut Tile> {
		if x < self.get_width() && y < self.get_height() {
			self.tiles[(x, y)]
		} else {
			None
		}
	}

}

static NONE: Option<Tile> = None;

impl Index<(usize, usize)> for Level {
	type Output= Option<Tile>;

    pub fn index(&self, (x, y): (usize, usize)) -> &Option<Tile>{
		if x < self.get_width() && y < self.get_height() {
			&self.tiles[(x, y)]
		} else {
			&NONE
		}
    }
}
