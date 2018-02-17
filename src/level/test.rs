use {Vector, Point};
use super::Hitbox;
use super::unaligned_level::{line_line_intersection_point, LineIntersectError};
use util::Perp;

fn assert_collides(hitbox1: &Hitbox, hitbox2: &Hitbox) {
    assert!(hitbox1.collides(hitbox2));
    assert!(hitbox2.collides(hitbox1));
    assert!(hitbox1.collides(hitbox1));
    assert!(hitbox2.collides(hitbox2));
}

fn assert_not_collides(hitbox1: &Hitbox, hitbox2: &Hitbox) {
    assert!(!hitbox1.collides(hitbox2));
    assert!(!hitbox2.collides(hitbox1));
}

#[test]
fn circle_circle_collision() {
    let circle1 = Hitbox::Circle(Vector::new(0., 0.), 1.);
    let circle2 = Hitbox::Circle(Vector::new(1.9, 0.), 1.);
    let circle3 = Hitbox::Circle(Vector::new(2.,2.), 1.);

    assert_collides(&circle1, &circle2);
    
    assert_not_collides(&circle1, &circle3);
    assert_not_collides(&circle2, &circle3);
}

#[test]
fn aabb_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0.5, 0.5), Vector::new(1., 1.));
    let aabb2 = Hitbox::Aabb(Vector::new(-0.5, -0.5), Vector::new(1., 1.));
    let aabb3 = Hitbox::Aabb(Vector::new(1.6, 2.6), Vector::new(1., 2.));
    let aabb4 = Hitbox::Aabb(Vector::new(50., 50.), Vector::new(100., 150.));

    assert_collides(&aabb1, &aabb2);
    assert_collides(&aabb1, &aabb4);
    assert_collides(&aabb2, &aabb4);
    assert_collides(&aabb3, &aabb4);

    assert_not_collides(&aabb1, &aabb3);
    assert_not_collides(&aabb2, &aabb3);
}

#[test]
fn crossing_aabb_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0.0, 0.0), Vector::new(10., 1.));
    let aabb2 = Hitbox::Aabb(Vector::new(0.0, 0.0), Vector::new(1., 10.));
    assert_collides(&aabb1, &aabb2);
}

#[test]
fn circle_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let circle1 = Hitbox::Circle(Vector::new(0., 0.), 1.);
    let aabb2 = Hitbox::Aabb(Vector::new(2., 2.), Vector::new(1., 1.));
    let circle2 = Hitbox::Circle(Vector::new(2., 2.), 1.);

    assert_collides(&aabb1, &circle1);
    assert_collides(&aabb2, &circle2);

    assert_not_collides(&aabb1, &aabb2);
    assert_not_collides(&aabb2, &circle1);
    assert_not_collides(&circle2, &aabb1);
    assert_not_collides(&circle2, &circle1);
}

#[test]
fn rectangle_circle_inside() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0.5, 0.5), 0.5);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_really_distant() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(100.,100.), 1.);

    assert_not_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_right_side_touching() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(2.,1.), 1.);
    
    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_left_side_touching() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(-2.,1.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_up_side_touching() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0. ,3.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_lower_side_touching() {
    let rectangle = Hitbox::Rectangle(Point::new(0.,0.), Point::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0., -1.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_close_to_side() {
    let rectangle = Hitbox::Rectangle(Point::new(1.,-1.), Point::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_close_to_side_not_touching() {
    let rectangle = Hitbox::Rectangle(Point::new(1.001,-1.), Point::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert_not_collides(&rectangle, &circle);
}

#[test]
fn rectangle_aabb_partial_overlap() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Point::new(-1., -1.), Point::new(0.5, 0.), 1.);
    
    assert_collides(&rectangle, &aabb);
}

#[test]
fn rectangle_aabb_left() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Point::new(-1., -1.), Point::new(-0.4, 0.), 1.);
    
    assert_not_collides(&rectangle, &aabb);
}

#[test]
fn rectangle_aabb_above_left() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Point::new(-1., -1.), Point::new(0., 3.), 1.);
    
    assert_not_collides(&rectangle, &aabb);
}

#[test]
fn rectangle_aabb_below() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Point::new(-4., -4.), Point::new(1., -4.), 0.2);
    
    assert_not_collides(&rectangle, &aabb);
}

#[test]
fn lines_intersecting() {
    let p1 = Point::new(-1., -1.);
    let v1 = Vector::new(1., 1.);
    let p2 = Point::new(1., -1.);
    let v2 = Vector::new(-1., 1.);
    assert_eq!(line_line_intersection_point(&p1,&v1,&p2,&v2), Ok(Point::new(0., 0.)));
    assert_eq!(line_line_intersection_point(&p2,&v2,&p1,&v1), Ok(Point::new(0., 0.)));
    let line1 = Hitbox::Line(p1, v1);
    let line2 = Hitbox::Line(p2, v2);
    assert_collides(&line1, &line2);
}

#[test]
fn lines_intersecting_weird() {
    let p1 = Point::new(-1., -1.);
    let v1 = Vector::new(1., 1.);
    let p2 = Point::new(1., -1.);
    let v2 = Vector::new(-2.8459832, 2.87654942);
    match line_line_intersection_point(&p1,&v1,&p2,&v2) {
        Ok(_) => {},
        _ => panic!("lines didn't intersect even thought they should"),
    }
    match line_line_intersection_point(&p2,&v2,&p1,&v1) {
        Ok(_) => {},
        _ => panic!("lines didn't intersect even thought they should"),
    }
    let line1 = Hitbox::Line(p1, v1);
    let line2 = Hitbox::Line(p2, v2);
    assert_collides(&line1, &line2);
}

#[test]
fn lines_not_intersecting() {
    let p1 = Point::new(-1., -1.);
    let v1 = Vector::new(1., 1.);
    let p2 = Point::new(-1., 1.);
    assert_eq!(line_line_intersection_point(&p1,&v1,&p2,&v1), Err(LineIntersectError::NoCollision));
    assert_eq!(line_line_intersection_point(&p2,&v1,&p1,&v1), Err(LineIntersectError::NoCollision));
    let line1 = Hitbox::Line(p1, v1.clone());
    let line2 = Hitbox::Line(p2, v1);
    assert_not_collides(&line1, &line2);
}

#[test]
fn lines_equal() {
    let p1 = Point::new(0., 0.);
    let v1 = Vector::new(1., 1.);
    let p2 = Point::new(2., 2.);
    let v2 = Vector::new(3., 3.);
    assert_eq!(line_line_intersection_point(&p1,&v1,&p2,&v2), Err(LineIntersectError::Infinite));
    assert_eq!(line_line_intersection_point(&p2,&v2,&p1,&v1), Err(LineIntersectError::Infinite));
    let line1 = Hitbox::Line(p1, v1);
    let line2 = Hitbox::Line(p2, v2);
    assert_collides(&line1, &line2);
}
#[test]
fn line_segment_intersection() {
    let ls1 = Hitbox::LineSegment(Point::new(-1., 0.), Point::new(1., 0.));
    let ls2 = Hitbox::LineSegment(Point::new(0., -1.), Point::new(0., 1.)); 
    let ls1_2 = Hitbox::LineSegment(Point::new(1., 0.), Point::new(-1., 0.));
    let ls2_2 = Hitbox::LineSegment(Point::new(0., 1.), Point::new(0., -1.));

    assert_collides(&ls1, &ls2);
    assert_collides(&ls1, &ls1_2);
    assert_collides(&ls1, &ls2_2);
    assert_collides(&ls2, &ls1_2);
    assert_collides(&ls2, &ls2_2);
    assert_collides(&ls1_2, &ls2_2);
}

#[test]
fn line_segment_no_intersections_above_and_below() {
    let ls1 = Hitbox::LineSegment(Point::new(-1., 0.25), Point::new(1., 0.));
    let ls2 = Hitbox::LineSegment(Point::new(1., 0.), Point::new(-1., 0.25));
    let lsabove = Hitbox::LineSegment(Point::new(0., 2.), Point::new(0., 1.));
    let lsbelow = Hitbox::LineSegment(Point::new(0.5, -2.), Point::new(0., -1.));
    let lsabove_2 = Hitbox::LineSegment(Point::new(0., 1.), Point::new(0., 2.));
    let lsbelow_2 = Hitbox::LineSegment(Point::new(0., -1.), Point::new(0.5, -2.));

    assert_not_collides(&ls1, &lsabove);
    assert_not_collides(&ls1, &lsbelow);
    assert_not_collides(&ls1, &lsabove_2);
    assert_not_collides(&ls1, &lsbelow_2);

    assert_not_collides(&ls2, &lsabove);
    assert_not_collides(&ls2, &lsbelow);
    assert_not_collides(&ls2, &lsabove_2);
    assert_not_collides(&ls2, &lsbelow_2);
}

#[test]
fn line_segment_no_intersections_left_and_right() {
    let ls1 = Hitbox::LineSegment(Point::new(-1., 0.25), Point::new(1., 0.));
    let ls2 = Hitbox::LineSegment(Point::new(1., 0.), Point::new(-1., 0.25));
    let lsright = Hitbox::LineSegment(Point::new(2., 0.), Point::new(1.5, 0.25));
    let lsleft = Hitbox::LineSegment(Point::new(-2., 0.), Point::new(-1.5, 0.25));
    let lsright_2 = Hitbox::LineSegment(Point::new(1.5, 0.25), Point::new(2., 0.));
    let lsleft_2 = Hitbox::LineSegment(Point::new(-1.5, 0.25), Point::new(-2., 0.));

    assert_not_collides(&ls1, &lsright);
    assert_not_collides(&ls1, &lsleft);
    assert_not_collides(&ls1, &lsright_2);
    assert_not_collides(&ls1, &lsleft_2);

    assert_not_collides(&ls2, &lsright);
    assert_not_collides(&ls2, &lsleft);
    assert_not_collides(&ls2, &lsright_2);
    assert_not_collides(&ls2, &lsleft_2);
}

#[test]
fn rectangle_point_collisions() {
    let point1 = Hitbox::Dot(Point::new(0., 0.));
    let point2 = Hitbox::Dot(Point::new(1., 1.));
    let point3 = Hitbox::Dot(Point::new(-1., -1.));
    let point4 = Hitbox::Dot(Point::new(1., -1.));
    let point5 = Hitbox::Dot(Point::new(-1., 1.));

    let rectangle = Hitbox::Rectangle(Point::new(-1., -1.), Point::new(1., -1.), 2.);

    assert_collides(&rectangle, &point1);
    assert_collides(&rectangle, &point2);
    assert_collides(&rectangle, &point3);
    assert_collides(&rectangle, &point4);
    assert_collides(&rectangle, &point5);
}

#[test]
fn rectangle_point_no_collisions() {
    let point1 = Hitbox::Dot(Point::new(1., 0.));
    let point2 = Hitbox::Dot(Point::new(-1., 0.));
    let point3 = Hitbox::Dot(Point::new(0., 1.));
    let point4 = Hitbox::Dot(Point::new(0., -1.));

    let rectangle = Hitbox::Rectangle(Point::new(-0.5, -0.5), Point::new(0.5, -0.5), 1.);

    assert_not_collides(&rectangle, &point1);
    assert_not_collides(&rectangle, &point2);
    assert_not_collides(&rectangle, &point3);
    assert_not_collides(&rectangle, &point4);
}
#[test]
fn rectangle_collides_with_its_points() {
    let spos = Point::new(-1., -1.);
    let epos = Point::new(-0.5, 0.);
    let thickness = 1.;

    let rectangle = Hitbox::Rectangle(spos, epos, thickness);

    let perp = (epos - spos).perpendicular().normalize() * thickness;
    let a1 = spos;
    let b1 = a1 + perp;
    let c1 = epos;
    let d1 = c1 + perp;

    assert_collides(&rectangle, &Hitbox::Dot(a1));
    assert_collides(&rectangle, &Hitbox::Dot(b1));
    assert_collides(&rectangle, &Hitbox::Dot(c1));
    assert_collides(&rectangle, &Hitbox::Dot(d1));
}


#[test]
fn rectangle_rectangle_cross_collides() {
    let rectangle1 = Hitbox::Rectangle(Point::new(2., -2.), Point::new(-2., 2.), 0.2);
    let rectangle2 = Hitbox::Rectangle(Point::new(2., 2.), Point::new(-2., -2.), 0.2);

    assert_collides(&rectangle1, &rectangle2);
}

