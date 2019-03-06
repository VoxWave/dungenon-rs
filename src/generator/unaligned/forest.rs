use crate::{Vector, Point};

use crate::level::{Object, UnalignedLevel};

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use poisson::{Builder, Type};
use poisson::algorithm::Bridson;

use kolli_desu::shapes::Circle;

/// Generates a forest full of trees.
/// ´level´ is the level that the forest is generate in.
/// ´tries´ is the number of trees the algorithm tries to generate.
/// ´tree_radius´ is a tuple of the minimum and maximum radiuses.
/// ´rands´ is a tuple of two random number generators needed for the algorithm.
/// ´area´ is a tuple of tuples forming an AABB to denote the area in which the forest is generated.
pub fn generate_forest<R: Rng>(
    level: &mut UnalignedLevel<String>,
    tries: usize,
    tree_radius: (f32, f32),
    rands: (&mut R, &mut R),
    area: ((f32, f32), (f32, f32)),
) {
    let ((min_x, min_y), (max_x, max_y)) = area;
    let (min_r, max_r) = tree_radius;

    let scaler = Vector::new(max_x - min_x, max_y - min_y);
    let min_corner = Vector::new(min_x, min_y);

    let (rand_x, rand_y) = rands;

    let min_side = f32::min(scaler.x, scaler.y);
    let tree_size = Range::new(min_r, max_r);
    let poisson_gen =
        Builder::<_, Vector<f32>>::with_samples(tries, max_r / min_side, Type::Normal)
            .build(rand_y, Bridson);

    for v in poisson_gen {
        let hitbox = Circle::new(
            v.component_mul(&scaler).coords,
            tree_size.ind_sample(rand_x),
        );
        let object = Object::new("tree".to_owned(), Box::new(hitbox), min_corner.coords);
        level.add(object);
    }
}
