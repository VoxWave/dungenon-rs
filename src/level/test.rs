use {Vector, Point};
use super::Hitbox;
use super::unaligned_level::{line_line_intersection_point, LineIntersectError};

fn assert_collides(hitbox1: &Hitbox, hitbox2: &Hitbox) {
    assert!(hitbox1.collides(hitbox1));
    assert!(hitbox2.collides(hitbox2));
    assert!(hitbox1.collides(hitbox2));
    assert!(hitbox2.collides(hitbox1));
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
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0.5, 0.5), 0.5);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_really_distant() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(100.,100.), 1.);

    assert_not_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_right_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(2.,1.), 1.);
    
    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_left_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(-2.,1.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_up_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0. ,3.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_rectangle_down_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0., -1.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_close_to_side() {
    let rectangle = Hitbox::Rectangle(Vector::new(1.,-1.), Vector::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert_collides(&rectangle, &circle);
}

#[test]
fn rectangle_circle_close_to_side_not_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(1.001,-1.), Vector::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert_not_collides(&rectangle, &circle);
}

#[test]
#[ignore]
fn rectangle_aabb_partial_overlap() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Vector::new(-1., -1.), Vector::new(0.5, 0.), 1.);
    
    assert_collides(&rectangle, &aabb);
}

#[test]
fn rectangle_aabb_left() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Vector::new(-1., -1.), Vector::new(-0.4, 0.), 1.);
    
    assert_collides(&rectangle, &aabb);
}

#[test]
fn rectangle_aabb_above_left() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Vector::new(-1., -1.), Vector::new(0., 3.), 1.);
    
    assert_not_collides(&rectangle, &aabb);
}

#[test]
#[ignore]
fn rectangle_aabb_below() {
    let aabb = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let rectangle = Hitbox::Rectangle(Vector::new(-4., -4.), Vector::new(1., -4.), 0.2);
    
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

// #[test]
// fn line_segment_no_intersections() {
//     let ls1 = Hitbox::LineSegment(Point::new(-1., 0.), Point::new(1., 0.));
//     let ls2 = Hitbox::LineSegment(Point::new(1., 0.), Point::new(-1., 0.));
//     let lsabove = Hitbox::LineSegment(Point::new(0., 2.), Point::new(0., 1.));
//     let lsbelow = Hitbox::LineSegment(Point::new(0.5, -2.), Point::new(0., -1.));
//     let lsabove_2 = Hitbox::LineSegment(Point::new(0., 1.), Point::new(0., 2.));
//     let lsbelow_2 = Hitbox::LineSegment(Point::new(0., -1.), Point::new(0.5, -2.));

//     assert!(!ls1.collides(&lsabove));
//     assert!(!lsabove.collides(&ls1));
//     assert!(!ls1.collides(&lsbelow));
//     assert!(!lsbelow.collides(&ls1));
//     assert!(!ls1.collides(&lsabove_2));
//     assert!(!lsabove_2.collides(&ls1));
//     assert!(!ls1.collides(&lsbelow_2));
//     assert!(!lsbelow_2.collides(&ls1));

//     assert!(!ls2.collides(&lsabove));
//     assert!(!lsabove.collides(&ls2));
//     assert!(!ls2.collides(&lsbelow));
//     assert!(!lsbelow.collides(&ls2));
//     assert!(!ls2.collides(&lsabove_2));
//     assert!(!lsabove_2.collides(&ls2));
//     assert!(!ls2.collides(&lsbelow_2));
//     assert!(!lsbelow_2.collides(&ls2));

//     let left_to_ls1 =
//     let right_to_ls1 = 
//     let left_to_ls1_2 =
//     let right_to_ls1_2 =
// }