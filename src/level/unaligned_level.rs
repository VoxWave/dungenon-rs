use {Vector, Point};

use na::zero;
use na::geometry::Rotation2;
use alga::linear::Transformation;

use rayon::iter::{ParallelIterator};
use rayon::prelude::*;

pub struct UnalignedLevel<T> {
    objects: Vec<Object<T>>,
}

impl<T> UnalignedLevel<T> {
    pub fn new() -> UnalignedLevel<T> {
        UnalignedLevel{
            objects: Vec::new(),
        }
    }

    /// Adds an `Object` to the level without checking for collision with other objects
    pub fn add_unchecked(&mut self, obj: Object<T>) {
        self.objects.push(obj);
    }
}

impl<T: Sync> UnalignedLevel<T> {
    
    ///Adds an `Object` to the level if it doesn't collide with other objects
    pub fn add(&mut self, obj: Object<T>) -> bool {
        if self.collides(&obj.hitbox) {
            false
        } else {
            self.add_unchecked(obj);
            true
        }
    }


    pub fn collides(&self, hitbox: &Hitbox) -> bool {
        (&self.objects).par_iter().any(|o| o.collides(hitbox))
    }
} 

pub struct Object<T> {
    pub value: T,
    pub hitbox: Hitbox,
}
impl<T> Object<T> {
    pub fn new(value: T, hitbox: Hitbox) -> Self {
        Object{
            value,
            hitbox,
        }
    }

    pub fn collides(&self, hitbox: &Hitbox) -> bool {
        self.hitbox.collides(hitbox)
    }
}
#[derive(Debug)]
pub enum Hitbox {
    Circle(Vector<f32>, f32),
    ///First vector denotes the center of the AABB and the second vector denotes the dimensions(width, height) of the AABB
    Aabb(Vector<f32>, Vector<f32>),
    Rectangle(Vector<f32>, Vector<f32>, f32),
}
impl Hitbox {
    pub fn collides(&self, hitbox: &Hitbox) -> bool {
        use self::Hitbox::*;
        match (self, hitbox) {
            (&Circle(ref a_lpos, ref a_radius), &Circle(ref b_lpos, ref b_radius)) => {
                ((*a_lpos) - (*b_lpos)).norm_squared() <= (*a_radius + *b_radius).powi(2)
            },
            (&Circle(ref c_lpos, ref c_radius), &Aabb(ref a_lpos, ref a_sides)) |
            (&Aabb(ref a_lpos, ref a_sides), &Circle(ref c_lpos, ref c_radius)) => {
                let width = a_sides.x.abs() / 2.;
                let height = a_sides.y.abs() / 2.;
                let aabb_center = *a_lpos;
                let circle_center = *c_lpos;
                let mut ca = aabb_center - circle_center;
                if ca != zero() {
                    ca = ca.normalize();
                }
                let outer = circle_center + *c_radius * ca;
                point_in_aabb(Point::from_coordinates(outer), (Point::from_coordinates(aabb_center), width, height))
            },
            (&Circle(ref c_lpos, ref c_radius), &Rectangle(ref r_spos, ref r_epos, ref r_height)) |
            (&Rectangle(ref r_spos, ref r_epos, ref r_height), &Circle(ref c_lpos, ref c_radius)) => {
                let rotation = Rotation2::rotation_between(&(r_epos-r_spos),&Vector::new(1.,0.));
                let rot_circle = rotation.transform_vector(&(c_lpos-r_spos));
                let aabb_width = (r_epos-r_spos).norm();
                let aabb_center = Vector::new(aabb_width/2., r_height/2.);
                let aabb = Aabb(aabb_center, Vector::new(aabb_width, *r_height));
                Hitbox::collides(&aabb, &Circle(rot_circle, *c_radius))
            },

            (&Aabb(ref aabb_pos, ref aabb_sides), &Rectangle(ref r_spos, ref r_epos, ref r_height)) |
            (&Rectangle(ref r_spos, ref r_epos, ref r_height), &Aabb(ref aabb_pos, ref aabb_sides)) => {
                
            },

            (&Aabb(ref a1_lpos, ref a1_sides), &Aabb(ref a2_lpos, ref a2_sides)) => {
                let a1_center = *a1_lpos;
                let a2_center = *a2_lpos;
                let a1_width = a1_sides.x.abs() / 2.;
                let a1_height = a1_sides.y.abs() / 2.;
                let a2_width = a2_sides.x.abs() / 2.;
                let a2_height = a2_sides.y.abs() / 2.;
                (a1_center.x - a1_width) <= (a2_center.x + a2_width) &&
                (a1_center.x + a1_width) >= (a2_center.x - a2_width) &&
                (a1_center.y - a1_height) <= (a2_center.y + a2_height) &&
                (a1_center.y + a1_height) >= (a2_center.y - a2_height)
            },

            _ => unimplemented!(),
        }
    }
}

fn point_in_aabb(point: Point<f32>, (center, width, height): (Point<f32>, f32, f32)) -> bool {
    point.x >= (center.x - width) &&
    point.x <= (center.x + width) &&
    point.y >= (center.y - height) &&
    point.y <= (center.y + height)
}