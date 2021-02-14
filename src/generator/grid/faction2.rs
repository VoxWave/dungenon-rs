use std::{mem, ops::Deref};

use rand::{rngs::OsRng, Rng};

use rayon::prelude::*;

use tile::Faction;

use level::GridLevel;

// const LEHMER_MULT0: u128 = 0x9cec0193f9cb55c4acce1fe16e62b05f;
// const LEHMER_MULT1: u128 = 0x82163e3e925f6e050dfa28d05eb25d83;
// const LEHMER_MULT2: u128 = 0xdd72c15464faa3388de458ec1e58452b;
// const LEHMER_MULT3: u128 = 0xb68e8e143dce210d5e40e89d3033fd65;
// const LEHMER_MULT4: u128 = 0x12e15e35b500f16e2e714eb2b37916a5;
// type Seed = u128;
// const BIT_LENGTH: Seed = 64;

// const LEHMER_MULT0: u64 = 9954494205990559033;
// const LEHMER_MULT1: u64 = 8009960310945691199;
// const LEHMER_MULT2: u64 = 7836174002681351413;
// const LEHMER_MULT3: u64 = 10414503560430172271;
// const LEHMER_MULT4: u64 = 2038836139019627173;
// type Seed = u64;
// const BIT_LENGTH: Seed = 32;

const LEHMER_MULT0: u32 = 3566928163;
const LEHMER_MULT1: u32 = 3999046367;
const LEHMER_MULT2: u32 = 3664638667;
const LEHMER_MULT3: u32 = 4211219281;
const LEHMER_MULT4: u32 = 2710379303;
type Seed = u32;
const BIT_LENGTH: Seed = 16;

struct StaticVec<T> {
    i: usize,
    data: [T; 9],
}

impl<T: Default + Copy> StaticVec<T> {
    fn new() -> StaticVec<T> {
        Self {
            i: 0,
            data: [T::default(); 9],
        }
    }

    fn push(&mut self, val: T) {
        if self.i < 9 {
            self.data[self.i] = val;
            self.i += 1;
        }
    }

    fn clear(&mut self) {
        self.i = 0;
    }

    fn is_empty(&self) -> bool {
        self.i == 0
    }
}

impl<T> Deref for StaticVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[0..self.i]
    }
}

pub struct FactionGen {
    seed: Seed,
}

impl FactionGen {
    pub fn new() -> Self {
        Self {
            seed: OsRng::new().unwrap().gen::<Seed>() << 1 | 1,
        }
    }
    pub fn generate(&mut self, level: &mut GridLevel<Faction>, buffer: &mut GridLevel<Faction>) {
        self.seed = tick(
            self.seed,
            level.get_width(),
            level.get_height(),
            &level.tiles.data[..],
            &mut buffer.tiles.data[..],
        );
        mem::swap(level, buffer);
    }
}

fn select(deck: &[usize], n: usize) -> usize {
    deck[n % deck.len()]
}

fn index(width: usize, _height: usize, x: i64, y: i64) -> usize {
    (x + y * width as i64) as usize
}

fn mix(a: Seed, b: usize, mult: Seed) -> Seed {
    a ^ ((b << 1 | 1) as Seed).wrapping_mul(mult)
}

const FACTOR: usize = 4;

pub fn tick(
    seed: Seed,
    width: usize,
    height: usize,
    prev: &[Faction],
    next: &mut [Faction],
) -> Seed {
    let corrected = (width / FACTOR) * FACTOR;
    let rows = (512 as f32 / width as f32).ceil() as usize;
    next.par_chunks_mut(width * rows)
        .enumerate()
        .for_each(|(y, chunk)| {
            chunk.chunks_mut(width).enumerate().for_each(|(yy, chunk)| {
                let yyy = y * rows + yy;
                row_tick(seed, yyy, width, height, corrected, prev, chunk)
            });
        });
    seed.wrapping_mul(LEHMER_MULT4)
}

type Deck<T> = StaticVec<T>;

fn row_tick(
    seed: Seed,
    y: usize,
    width: usize,
    height: usize,
    corrected: usize,
    prev: &[Faction],
    chunk: &mut [Faction],
) {
    let start_y = if y == 0 { 0 } else { -1 };
    let end_y = if y == height - 1 { 0 } else { 1 };
    (&mut chunk[0..corrected])
        .par_chunks_mut(512)
        .for_each(|c| inner_tick(seed, y, start_y, end_y, width, height, corrected, prev, c));
    let mut deck = Deck::new();
    let mut seed = seed;
    for x in corrected..width {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let xx = x as i64 + dx;
                let yy = y as i64 + dy;
                let idx = index(width, height, xx, yy);
                if let Some(Faction::Faction(f)) = prev.get(idx) {
                    deck.push(*f);
                }
            }
        }
        if !deck.is_empty() {
            seed = seed.wrapping_mul(LEHMER_MULT0);
            let f = select(&*deck, (seed >> BIT_LENGTH) as usize);
            chunk[x] = Faction::Faction(f);
            deck.clear();
        }
    }
}

pub fn inner_tick(
    seed: Seed,
    y: usize,
    start_y: i64,
    end_y: i64,
    width: usize,
    height: usize,
    corrected: usize,
    prev: &[Faction],
    chunk: &mut [Faction],
) {
    let mut deck0 = Deck::new();
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();
    let mut deck3 = Deck::new();
    let mut rngs: [Seed; 4] = [
        mix(seed, y, LEHMER_MULT0) << 1 | 1,
        mix(seed, y, LEHMER_MULT1) << 1 | 1,
        mix(seed, y, LEHMER_MULT2) << 1 | 1,
        mix(seed, y, LEHMER_MULT3) << 1 | 1,
    ];
    let len = chunk.len();
    let mut calc = |x, deck: &mut Deck<_>, n| {
        if !deck.is_empty() {
            let f = select(&*deck, n);
            if x < chunk.len() {
                chunk[x] = Faction::Faction(f);
            } else {
                debug_assert!(false);
                unsafe { ::std::hint::unreachable_unchecked() }
            }
            deck.clear();
        }
    };
    for x in (0..len).step_by(FACTOR) {
        let start_x = if x == 0 { 0 } else { -1 };
        let end_x = FACTOR as i64 - if x == corrected - FACTOR { 1 } else { 0 };
        // x x x x x x
        // x o o o o x
        // x x x x x x
        for dy in start_y..=end_y {
            for dx in start_x..=end_x {
                let xx = x as i64 + dx;
                let yy = y as i64 + dy;
                let idx = index(width, height, xx, yy);
                if idx < prev.len() {
                    if let Faction::Faction(f) = prev[idx] {
                        if dx <= 1 {
                            deck0.push(f);
                        }
                        if 0 <= dx && dx <= 2 {
                            deck1.push(f);
                        }
                        if 1 <= dx && dx <= 3 {
                            deck2.push(f);
                        }
                        if 2 <= dx {
                            deck3.push(f);
                        }
                    }
                } else {
                    debug_assert!(false);
                    unsafe { ::std::hint::unreachable_unchecked() }
                }
            }
        }
        rngs = [
            rngs[0].wrapping_mul(LEHMER_MULT0),
            rngs[1].wrapping_mul(LEHMER_MULT1),
            rngs[2].wrapping_mul(LEHMER_MULT2),
            rngs[3].wrapping_mul(LEHMER_MULT3),
        ];
        calc(x + 0, &mut deck0, (rngs[0] >> BIT_LENGTH) as usize);
        calc(x + 1, &mut deck1, (rngs[1] >> BIT_LENGTH) as usize);
        calc(x + 2, &mut deck2, (rngs[2] >> BIT_LENGTH) as usize);
        calc(x + 3, &mut deck3, (rngs[3] >> BIT_LENGTH) as usize);
    }
}
