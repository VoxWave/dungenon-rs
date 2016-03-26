use level::Level;

use na::Vec2;

use rand::os::OsRng;
use rand::XorShiftRng;
use rand::Rand;
use rand::Rng;

use tile::Tile;
use util::{Direction};

pub struct MazeGen {
    pub pos: Vec2<usize>,
    rand: XorShiftRng,
}

impl MazeGen {
    pub fn new(x: usize, y: usize) -> MazeGen {
        MazeGen{
            pos: Vec2::new(x,y),
            rand: XorShiftRng::rand(&mut OsRng::new().unwrap()),
        }
    }

    pub fn generate(&mut self, level: &mut Level) {
        let mut stack: Vec<Vec2<usize>> = Vec::new();
        stack.push(self.pos);
        'mainloop: while let Some(cur) = stack.pop() {
            let neighbours = Self::get_neighbours(level, &cur);
            match level.get_mut_tile_with_vec(&cur) {

                Some(tile) => {
                    if *tile == Tile::Void || *tile == Tile::Floor {
                        continue 'mainloop
                    }
                    match neighbours {
                        Some(mut neighbours) => {
                            self.rand.shuffle(&mut neighbours);
                            while let Some(p) = neighbours.pop() {
                                stack.push(p);
                            }
                            *tile = Tile::Floor;
                        },
                        None => continue 'mainloop
                    }
                },

                None => continue 'mainloop
            }

        }
    }

    fn get_neighbours(level: &Level, pos: &Vec2<usize>) -> Option<Vec<Vec2<usize>>> {
        let mut neighbours: Vec<Vec2<usize>> = Vec::new();
        let mut floors = 0;
        for d in Direction::get_orthogonal_dirs() {
            let mut pos = pos.clone();
            let dvec = d.get_vec();
            pos.x = (pos.x as isize + dvec.x) as usize;
            pos.y = (pos.y as isize + dvec.y) as usize;
            match level[pos] {
                Some(Tile::Floor) => {
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
