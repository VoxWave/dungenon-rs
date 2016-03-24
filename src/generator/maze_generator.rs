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

    pub fn generate(&self, level: &mut Level) {
        let mut stack: Vec<Vec2<usize>> = Vec::new();
        stack.push(self.pos);
        'mainloop: while let Some(cur) = stack.pop() {

            match level.get_mut_tile_with_vec(cur) {

                Some(tile) => {
                    if tile == Tile::Void || tile == Tile::Floor {
                        continue 'mainloop
                    }
                    let _:() = self.get_neighbours();
                    match self.get_neighbours() {
                        Some(neighbours) => {
                            self.rand.shuffle(&mut neighbours);
                            while !neighbours.is_empty() {
                                stack.push(neighbours.pop());
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

    fn get_neighbours(level: &mut Level, pos: &Vec2<usize>) -> Option<Vec<Vec2<usize>>> {
        let mut neighbours: Vec<Vec2<usize>> = Vec::new();
        let mut floors = 0;
        for d in Direction::get_orthogonal_dirs() {
            match level[d.get_vec() + pos.clone()] {
                Some(Tile::Floor) => {
                    floors += 1;
                    if floors > 1 {
                        return None;
                    }
                },
                _ => {},
            }
            neighbours.push(d.get_vec() + pos.clone())
        }
        Some(neighbours)
    }

}
