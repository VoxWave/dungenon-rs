use level::{add_isize_to_usize, GridLevel};

use Vector;

use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use tile::Tile;
use util::{Direction};

pub struct MazeGen {
    pub pos: Vector<usize>,
    rand: XorShiftRng,
}

impl MazeGen {
    pub fn new(x: usize, y: usize) -> MazeGen {
        MazeGen{
            pos: Vector::new(x,y),
            rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
        }
    }

    pub fn generate(&mut self, level: &mut GridLevel<Tile>) {
        use util::Error;
        let mut stack: Vec<Vector<usize>> = Vec::new();
        stack.push(self.pos);
        'mainloop: while let Some(cur) = stack.pop() {
            let neighbours = Self::get_neighbours(level, &cur);
            match level.get_mut_tile_with_vec(&cur) {

                Ok(tile) => {
                    match *tile {
                        Tile::Void(_) | Tile::Floor(_) => {continue 'mainloop},
                        _ => {},
                    }
                    match neighbours {
                        Some(mut neighbours) => {
                            self.rand.shuffle(&mut neighbours);
                            while let Some(p) = neighbours.pop() {
                                stack.push(p);
                            }
                            *tile = Tile::Floor(0);
                        },
                        None => continue 'mainloop
                    }
                },

                Err(Error::IndexOutOfBounds) => continue 'mainloop
            }

        }
    }

    fn get_neighbours(level: &GridLevel<Tile>, pos: &Vector<usize>) -> Option<Vec<Vector<usize>>> {
        let mut neighbours: Vec<Vector<usize>> = Vec::new();
        let mut floors = 0;
        for d in Direction::get_orthogonal_dirs() {
            let pos = pos.clone();
            let dvec = d.get_vec();
            let coord = match (add_isize_to_usize(dvec.x, pos.x), add_isize_to_usize(dvec.y, pos.y)) {
                (Some(x), Some(y)) => (x,y),
                _ => continue,
            };
            match level.get_tile_with_tuple(coord) {
                Ok(&Tile::Floor(_)) => {
                    floors += 1;
                    if floors > 1 {
                        return None;
                    }
                },
                _ => {},
            }
            neighbours.push(pos);
        }
        Some(neighbours)
    }

}
