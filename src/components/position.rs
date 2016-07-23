use nalgebra as na;

use ecs::{Component, VecStorage, HashMapStorage, MaskedStorage, NullStorage};

pub struct Position1(pub na::Point1<f32>); // TODO: Make this generic

impl Position1 {
    pub fn new(x: f32) -> Position1 {
        Position1( na::Point1::new(x))
    }
}
impl Component for Position1 {
    type Storage = VecStorage<Position1>;
}

pub struct Position2(pub na::Point2<f32>); // TODO: Make this generic
impl Component for Position2 {
    type Storage = VecStorage<Position2>;
}

pub struct Position3(pub na::Point3<f32>); // TODO: Make this generic
impl Component for Position3 {
    type Storage = VecStorage<Position3>;
}

pub struct Position4(pub na::Point4<f32>); // TODO: Make this generic
impl Component for Position4 {
    type Storage = VecStorage<Position4>;
}
