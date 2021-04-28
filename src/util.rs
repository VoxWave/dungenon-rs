use std::default::Default;
use std::ops::{Index, IndexMut};
use Vector;

#[derive(Clone)]
pub struct Grid<T> {
    pub(crate) data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.data.len() / self.width
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            data: vec![Default::default(); width * height],
            width: width,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_filled_with(thing: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            data: vec![thing; width * height],
            width: width,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[x + y * self.width]
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

static ORTHOGONAL: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
static ALL: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::Dl,
    Direction::Dr,
    Direction::Ul,
    Direction::Ur,
];

impl Direction {
    pub fn get_vec(&self) -> Vector<isize> {
        let (x, y) = self.get_tuple();
        Vector::new(x, y)
    }

    pub fn get_tuple(&self) -> (isize, isize) {
        use self::Direction::*;
        match *self {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
            Ur => (1, 1),
            Dr => (1, -1),
            Dl => (-1, -1),
            Ul => (-1, 1),
        }
    }

    pub fn get_orthogonal_dirs() -> &'static [Direction] {
        &ORTHOGONAL
    }
    pub fn get_dirs() -> &'static [Direction] {
        &ALL
    }
}

pub enum Error {
    IndexOutOfBounds,
}
