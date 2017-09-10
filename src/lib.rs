extern crate nalgebra as na;
extern crate rand;
extern crate rayon;
extern crate smallvec;
extern crate poisson;

pub mod generator;
pub mod level;
pub mod tile;
pub mod util;

type Vector<T> = na::Vector2<T>;
type Point<T> = na::Point2<T>;
