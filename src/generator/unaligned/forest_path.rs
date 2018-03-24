use rand::Rng;

use Point;

use super::forest::generate_forest;
use super::path::generate_linear_path;

use level::{Hitbox, UnalignedLevel};

pub fn generate_linear_forest_path<R: Rng>(
    level: &mut UnalignedLevel<String>,
    points: &[Point<f32>],
    rands: (&mut R, &mut R),
    path_thickness: f32,
    tree_radius: (f32, f32),
    forest_area: ((f32, f32), (f32, f32)),
) {
    generate_linear_path(level, points, path_thickness);
    generate_forest(level, 100, tree_radius, rands, forest_area)
}

pub fn generate_noisy_forest_path<R: Rng>(
    level: &mut UnalignedLevel<String>,
    points: &Vec<Point<f32>>,
    rand: R,
    path_thickness: f32,
) {

}
