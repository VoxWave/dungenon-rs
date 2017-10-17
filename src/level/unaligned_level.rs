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
    Line(Point<f32>, Vector<f32>),
    LineSegment(Point<f32>, Point<f32>),
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
                //TODO: slope breaks when vertical
                let slope = (r_epos.y - r_spos.y)/(r_epos.x - r_spos.x);
                let left_epos = Vector::new(r_spos.y-r_epos.y, r_epos.x-r_spos.x).normalize()* *r_height;
                let aabb_left = aabb_pos.x - aabb_sides.x/2.;
                let aabb_left_rectangle_bottom_intersect = (aabb_left - r_spos.x)*slope+r_spos.y;
                let aabb_left_rectangle_top_intersect = (aabb_left - (r_spos.x + left_epos.x))*slope+r_spos.y+left_epos.y;
                //TODO: slope2 breaks when vertical
                let slope_2 = (left_epos.y - r_spos.y)/(left_epos.x - r_spos.x);
                let aabb_left_rectangle_left_intersect = (aabb_left - r_spos.x)*slope_2+r_spos.y;
                let aabb_left_rectangle_right_intersect = (aabb_left - (r_spos.x + r_epos.x))*slope_2+r_spos.y+r_epos.y;
                let aabb_bottom = aabb_pos.y - aabb_sides.y/2.;
                let aabb_top = aabb_pos.y + aabb_sides.y/2.;
                if aabb_left_rectangle_bottom_intersect >= aabb_bottom && aabb_left_rectangle_bottom_intersect <= aabb_top {
                    true
                } else if aabb_left_rectangle_top_intersect >= aabb_bottom && aabb_left_rectangle_top_intersect <= aabb_top {
                    true
                } else if aabb_left_rectangle_left_intersect >= aabb_bottom && aabb_left_rectangle_left_intersect <= aabb_top {
                    true
                } else if aabb_left_rectangle_right_intersect >= aabb_bottom && aabb_left_rectangle_right_intersect <= aabb_top {
                    true
                } else {
                    false
                }
                
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

            (&Line(ref p1, ref v1), &Line(ref p2, ref v2)) => {
                match line_line_intersection_point(p1, v1, p2, v2) {
                    Ok(_) => true,
                    Err(LineIntersectError::Infinite) => true,
                    Err(LineIntersectError::NoCollision) => false,
                }
            },
            (&LineSegment(ref a1, ref a2), &LineSegment(ref b1, ref b2)) => {
                match line_line_intersection_point(a1, &(a2 - a1), b1, &(b2 - b1)) {
                    Ok(p) => {
                        p.x >= a1.x.min(a2.x) &&
                        p.x <= a1.x.max(a2.x) &&
                        p.y >= a1.y.min(a2.y) &&
                        p.y <= a1.y.max(a2.y) &&
                        p.x >= b1.x.min(b2.x) &&
                        p.x <= b1.x.max(b2.x) &&
                        p.y >= b1.y.min(b2.y) &&
                        p.y <= b1.y.max(b2.y)
                    },
                    Err(LineIntersectError::Infinite) => {
                        b1.x >= a1.x.min(a2.x) &&
                        b1.x <= a1.x.max(a2.x) &&
                        b1.y >= a1.y.min(a2.y) &&
                        b1.y <= a1.y.max(a2.y) ||

                        b2.x >= a1.x.min(a2.x) &&
                        b2.x <= a1.x.max(a2.x) &&
                        b2.y >= a1.y.min(a2.y) &&
                        b2.y <= a1.y.max(a2.y) ||

                        a1.x >= b1.x.min(b2.x) &&
                        a1.x <= b1.x.max(b2.x) &&
                        a1.y >= b1.y.min(b2.y) &&
                        a1.y <= b1.y.max(b2.y) ||

                        a1.x >= b1.x.min(b2.x) &&
                        a1.x <= b1.x.max(b2.x) &&
                        a1.y >= b1.y.min(b2.y) &&
                        a1.y <= b1.y.max(b2.y)
                    },
                    Err(LineIntersectError::NoCollision) => {
                        false
                    },
                }
            },

            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LineIntersectError {
    Infinite, NoCollision,
}

pub fn line_line_intersection_point(p1: &Point<f32>, v1: &Vector<f32>, p2: &Point<f32>, v2: &Vector<f32>) -> Result<Point<f32>, LineIntersectError> {
    let denominator_det = (v1.x*(-v2.y)) - ((-v2.x)* v1.y);
    let numerator_det = ((p2.x - p1.x)*(-v2.y)) - ((-v2.x)* (p2.y - p1.y));
    if denominator_det == 0. {
        if numerator_det == 0. {
            Err(LineIntersectError::Infinite)
        } else {
            Err(LineIntersectError::NoCollision)
        }
    } else {
        let x = numerator_det/denominator_det;
        Ok(p1 + (v1*x))
    }
}

fn point_in_aabb(point: Point<f32>, (center, width, height): (Point<f32>, f32, f32)) -> bool {
    point.x >= (center.x - width) &&
    point.x <= (center.x + width) &&
    point.y >= (center.y - height) &&
    point.y <= (center.y + height)
}