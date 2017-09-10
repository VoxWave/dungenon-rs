pub struct UnalignedLevel<T> {
    objects: Vec<Object<T>>,
}

pub struct Object<T> {
    value: T,
    hitbox: HitBox,
}

pub enum Hitbox {
    Circle(Vector, f32),
    Aabb(Vector, Vector),
}