use {Vector, Point};

use na::{zero, dot};
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
    Rectangle(Point<f32>, Point<f32>, f32),
    Line(Point<f32>, Vector<f32>),
    LineSegment(Point<f32>, Point<f32>),
    Dot(Point<f32>),
}
impl Hitbox {
    pub fn collides<'a>(&'a self, hitbox: &'a Hitbox) -> bool {
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
                let rotation = Rotation2::rotation_between(&(r_epos - r_spos), &Vector::new(1.,0.));
                let rot_circle = rotation.transform_vector(&(c_lpos - r_spos.coords));
                let aabb_width = (r_epos-r_spos).norm();
                let aabb_center = Vector::new(aabb_width/2., r_height/2.);
                let aabb = Aabb(aabb_center, Vector::new(aabb_width, *r_height));
                Hitbox::collides(&aabb, &Circle(rot_circle, *c_radius))
            },
            (&Aabb(ref aabb_pos, ref aabb_sides), r @ &Rectangle(..)) |
            (r @ &Rectangle(..), &Aabb(ref aabb_pos, ref aabb_sides)) => {
                let s_pos = Point::from_coordinates(*aabb_pos);
                let e_pos = Point::from_coordinates(aabb_pos + Vector::new(aabb_sides.x, 0.));
                Hitbox::collides(r, &Rectangle(s_pos, e_pos, aabb_sides.y))
            },
            (&Rectangle(ref r1_spos, ref r1_epos, ref r1_height), &Rectangle(ref r2_spos, ref r2_epos, ref r2_height)) => {
                let perp = Vector::new(r1_epos.y - r1_spos.y, r1_epos.x - r1_spos.x).normalize() * *r1_height;
                let a1 = r1_spos;
                let b1 = a1 + perp;
                let c1 = r1_epos;
                let d1 = c1 + perp;

                let perp = Vector::new(r2_epos.y - r2_spos.y, r2_epos.x - r2_spos.x).normalize() * *r2_height;
                let a2 = r2_spos;
                let b2 = a2 + perp;
                let c2 = r2_epos;
                let d2 = c2 + perp;

                let r1 = &Rectangle(*r1_spos, *r1_epos, *r1_height);
                let r2 = &Rectangle(*r2_spos, *r2_epos, *r2_height);

                Hitbox::collides(r1, &Dot(*a2)) ||
                Hitbox::collides(r1, &Dot(b2)) ||
                Hitbox::collides(r1, &Dot(*c2)) ||
                Hitbox::collides(r1, &Dot(d2)) ||

                Hitbox::collides(r2, &Dot(*a1)) ||
                Hitbox::collides(r2, &Dot(b1)) ||
                Hitbox::collides(r2, &Dot(*c1)) ||
                Hitbox::collides(r2, &Dot(d1))
            },
            (&Rectangle(ref r_spos, ref r_epos, ref r_height), &Dot(ref p)) |
            (&Dot(ref p), &Rectangle(ref r_spos, ref r_epos, ref r_height)) => {
                let perp = Vector::new(r_epos.y - r_spos.y, r_epos.x - r_spos.x).normalize() * *r_height;
                let a = &r_spos.coords;
                let b = &r_epos.coords;
                let c = &(b + perp);
                let d = &(a + perp);

                let which_side = |(a, b): (&Vector<f32>, &Vector<f32>), c: &Vector<f32>| {
                    let diff = b - a;
                    dot(&(c - a), &Vector::new(-diff.y, diff.x))
                };

                let p = &p.coords;

                which_side((a, b), p) >= 0. &&
                which_side((b, c), p) >= 0. &&
                which_side((c, d), p) >= 0. &&
                which_side((d, a), p) >= 0.
            },
            (&Dot(ref p1), &Dot(ref p2)) => p1 == p2,
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
            ref u => unimplemented!("{:?}", u),
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