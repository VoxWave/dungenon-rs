use rand::Rng;

use crate::Vector;
use crate::Point;

use crate::level::{Hitbox, Object, UnalignedLevel};

pub fn generate_linear_path(
    level: &mut UnalignedLevel<String>,
    points: &[Point<f32>],
    path_thickness: f32,
) {
    for p2 in points.windows(2) {
        let point1 = p2[0];
        let point2 = p2[1];
        let rectangle = Object::new(
            "de way".into(),
            Hitbox::Rectangle(point1, point2, path_thickness),
            Point::new(0., 0.)
        );
        level.add(rectangle);
    }
}

pub fn generate_noisy_path() {}
