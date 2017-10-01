use Vector;
use super::Hitbox;

#[test]
fn circle_circle_collision() {

    let circle1 = Hitbox::Circle(Vector::new(0., 0.), 1.);
    let circle2 = Hitbox::Circle(Vector::new(1.9, 0.), 1.);
    let circle3 = Hitbox::Circle(Vector::new(2.,2.), 1.);

    assert!(circle1.collides(&circle2));
    assert!(circle2.collides(&circle1));
    
    assert!(!circle3.collides(&circle1));
    assert!(!circle3.collides(&circle2));
    assert!(!circle1.collides(&circle3));
    assert!(!circle2.collides(&circle3));
}

#[test]
fn aabb_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0.5, 0.5), Vector::new(1., 1.));
    let aabb2 = Hitbox::Aabb(Vector::new(-0.5, -0.5), Vector::new(1., 1.));
    let aabb3 = Hitbox::Aabb(Vector::new(1.6, 2.6), Vector::new(1., 2.));
    let aabb4 = Hitbox::Aabb(Vector::new(50., 50.), Vector::new(100., 150.));

    assert!(aabb1.collides(&aabb2));
    assert!(aabb2.collides(&aabb1));

    assert!(!aabb3.collides(&aabb1));
    assert!(!aabb3.collides(&aabb2));
    assert!(!aabb1.collides(&aabb3));
    assert!(!aabb2.collides(&aabb3));

    assert!(aabb4.collides(&aabb1));
    assert!(aabb1.collides(&aabb4));
    assert!(aabb4.collides(&aabb2));
    assert!(aabb2.collides(&aabb4));
    assert!(aabb4.collides(&aabb3));
    assert!(aabb3.collides(&aabb4));
}

#[test]
fn crossing_aabb_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0.0, 0.0), Vector::new(10., 1.));
    let aabb2 = Hitbox::Aabb(Vector::new(0.0, 0.0), Vector::new(1., 10.));
    assert!(aabb1.collides(&aabb2));
    assert!(aabb2.collides(&aabb1));
}

#[test]
fn circle_aabb_collision() {
    let aabb1 = Hitbox::Aabb(Vector::new(0., 0.), Vector::new(1., 1.));
    let circle1 = Hitbox::Circle(Vector::new(0., 0.), 1.);
    let aabb2 = Hitbox::Aabb(Vector::new(2., 2.), Vector::new(1., 1.));
    let circle2 = Hitbox::Circle(Vector::new(2., 2.), 1.);

    assert!(aabb1.collides(&circle1));
    assert!(circle1.collides(&aabb1));

    assert!(!aabb2.collides(&aabb1));
    assert!(!aabb2.collides(&circle1));
    assert!(!aabb1.collides(&aabb2));
    assert!(!circle1.collides(&aabb2));

    assert!(!circle2.collides(&aabb1));
    assert!(!circle2.collides(&circle1));
    assert!(!aabb1.collides(&circle2));
    assert!(!circle1.collides(&circle2));

    assert!(circle2.collides(&aabb2));
    assert!(aabb2.collides(&circle2));
}

#[test]
fn rectangle_circle_inside() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0.5, 0.5), 0.5);

    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_really_distant() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(100.,100.), 1.);

    assert!(!rectangle.collides(&circle));
    assert!(!circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_right_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(2.,1.), 1.);
    
    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_left_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(-2.,1.), 1.);

    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_up_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0. ,3.), 1.);

    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_rectangle_down_side_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(0.,0.), Vector::new(1.,1.), 2f32.sqrt());
    let circle = Hitbox::Circle(Vector::new(0., -1.), 1.);

    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_close_to_side() {
    let rectangle = Hitbox::Rectangle(Vector::new(1.,-1.), Vector::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert!(rectangle.collides(&circle));
    assert!(circle.collides(&rectangle));
}

#[test]
fn rectangle_circle_close_to_side_not_touching() {
    let rectangle = Hitbox::Rectangle(Vector::new(1.001,-1.), Vector::new(2.,-1.), 2.);
    let circle = Hitbox::Circle(Vector::new(0., 0.), 1.);

    assert!(!rectangle.collides(&circle));
    assert!(!circle.collides(&rectangle));
}