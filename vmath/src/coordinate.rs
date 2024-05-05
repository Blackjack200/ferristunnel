use crate::vec3::Vec3;

pub enum Facing {
    Up,
    Down,
    North,
    East,
    West,
    South,
}

pub trait Space<T: Copy>: Sized {
    fn direction(f: Facing) -> Option<Vec3<T>>;
}
