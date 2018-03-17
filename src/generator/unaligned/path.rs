use rand::Rng;

use Vector;
use Point;

use level::{UnalignedLevel, Object, Hitbox};

pub fn generate_linear_path(level: &mut UnalignedLevel<String>, points: &Vec<Point<f32>>, path_thickness: f32) {
     for p2 in points.windows(2) {
        let point1 = p2[0];
        let point2 = p2[1];
        let rectangle = Object::new("de way".into() ,Hitbox::Rectangle(point1, point2, path_thickness));
        level.add_unchecked(rectangle);
    }
}

pub fn generate_noisy_path() {

}