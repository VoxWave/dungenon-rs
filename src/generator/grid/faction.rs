use rand::SeedableRng;
use rand::XorShiftRng;
use rand::{rngs::OsRng, seq::SliceRandom, RngCore};
use std::mem;

use rayon::prelude::*;
use smallvec::SmallVec;

use tile::Faction;

use util::{Direction, Error};

use level::{add_isize_to_usize, GridLevel};

pub struct FactionGen {
    rand: XorShiftRng,
}

impl FactionGen {
    pub fn new() -> FactionGen {
        FactionGen {
            rand: XorShiftRng::from_rng(&mut OsRng::new().unwrap()).unwrap(),
        }
    }
    #[inline]
    pub fn generate(&mut self, level: &mut GridLevel<Faction>, buffer: &mut GridLevel<Faction>) {
        let lovel = SuperUnsafe(buffer as *mut GridLevel<Faction>);
        let width = level.get_width();
        let height = level.get_height();
        let number = self.rand.next_u32();
        {
            let level_ref = &*level;
            (0..width).into_par_iter()
                .map(|x| (x, (0..height).into_par_iter()))
                .flat_map(|(x, ys)| {
                    ys.flat_map(move |y| {
                        let x_seed = (x as u64 ^ (x as u64 >> 32)) as u32;
                        let y_seed = (y as u64 ^ (y as u64 >> 32)) as u32;
                        let [a0, a1, a2, a3] = number.to_be_bytes();
                        let [b0, b1, b2, b3] = x_seed.to_be_bytes();
                        let [c0, c1, c2, c3] = y_seed.to_be_bytes();
                        let mut rand = XorShiftRng::from_seed([
                            a0, a1, a2, a3,//
                            b0, b1, b2, b3,//
                            c0, c1, c2, c3,//
                            1, 1, 1, 1//
                        ]);
                        rand.next_u32();
                        rand.next_u32();
                        let mut deck = SmallVec::<[_; 9]>::new();
                        match level_ref.get_tile(x, y) {
                            Ok(tile) => {
                                match *tile {
                                    Faction::Faction(_) => {
                                        deck.push(tile.clone());
                                    },
                                    Faction::Void => return None,
                                    _ => {},
                                }
                            },
                            Err(Error::IndexOutOfBounds) => {
                                unreachable!("Generate method indexed out of bounds while simulating a step. This should never happen unless the programmer is not very bright.");
                            }
                        }
                        Self::get_faction_neighbours(x, y, &mut deck, level_ref);
                        deck.choose(&mut rand)
                            .map(|f| ((x, y), f.clone()))
                    })
                })
                .for_each(|((x, y), f)| {
                    //this should be safe, right?
                    if let Ok(tile) = unsafe{ &mut *lovel.0 }.get_mut_tile(x, y) {
                        *tile = f;
                    }
                });
        }
        mem::swap(level, buffer);
    }

    fn get_faction_neighbours(
        x: usize,
        y: usize,
        deck: &mut SmallVec<[Faction; 9]>,
        level: &GridLevel<Faction>,
    ) {
        for d in Direction::get_dirs() {
            let (ix, iy) = d.get_tuple();
            let coord = match (add_isize_to_usize(ix, x), add_isize_to_usize(iy, y)) {
                (Some(x), Some(y)) => (x, y),
                _ => continue,
            };
            match level.get_tile_with_tuple(coord) {
                Ok(f @ &Faction::Faction(_)) => deck.push(f.clone()),
                _ => {}
            }
        }
    }
}
//this struct is unsafe. Use it with great caution.
struct SuperUnsafe(*mut GridLevel<Faction>);
unsafe impl Sync for SuperUnsafe {}
