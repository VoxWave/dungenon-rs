use std::ops::{Index, IndexMut};
use std::default::Default;
use na::Vec2;

pub struct Grid<T> {
	data: Vec<T>,
	width: usize,
}

impl<T: Default> Grid<T> {
	pub fn new(width:usize, height:usize) -> Grid<T> {
		Grid{
			data: vec![Default::default(), width * height],
			width: width,
		}
	}
	pub fn new_filled_with(thing: T, width: usize, height: usize) -> Grid<T> {
		Grid{
			data: vec![thing, width * height],
			width: width,
		}
	}

	pub fn get_width(&self) -> usize {
		self.width
	}

	pub fn get_height(&self) -> usize {
		self.data.len / self.width
	}
}

impl<T> Index<(usize, usize)> for Grid<T> {
	type Output= T;

    pub fn index(&self, (x, y): (usize, usize)) -> &T{
    	&self.data[x+y*self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
	pub fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
		&mut self.data[x+y*self.width]
	}
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
	Ur,
	Dr,
	Dl,
	Ul,
}

impl Direction {
	pub fn get_vec(&self) -> Vec2<usize> {
		let vec = match self {
			Up => Vec2::new(0,1),
			Down => Vec2::new(0,-1),
			Left => Vec2::new(-1,0),
			Right => Vec2::new(1,0),
			Ur => Vec2::new(1,1),
			Dr => Vec2::new(1,-1),
			Dl => Vec2::new(-1,-1),
			Ul => Vec2::new(-1,1),
		};
		vec
	}

	pub fn get_orthogonal_dir_vec2() {

	}
}
