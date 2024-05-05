use std::ops::{Add, Mul};

use crate::coordinate::Facing;
use crate::vec3::Vec3;

pub struct RayTraceResult<T: Copy> {
    pub plane: Facing,
    pub hit_vector: Vec3<T>,
}

pub trait BoundingBox<T: Copy> {
    fn collided(&self, self_pos: Vec3<T>, bb: Self, bb_pos: Vec3<T>) -> bool;
    fn intersect(&self, self_pos: Vec3<T>, bb: Self, bb_pos: Vec3<T>) -> Option<RayTraceResult<T>>;

    fn scale(&self, multiplier: T) -> Self;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AxisBoundingBox<T: Copy> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T> + PartialOrd<T>> BoundingBox<T>
    for AxisBoundingBox<T>
{
    fn collided(&self, self_pos: Vec3<T>, bb: Self, bb_pos: Vec3<T>) -> bool {
        let n_self = Self {
            min: self.min + self_pos,
            max: self.max + self_pos,
        };
        let n_bb = Self {
            min: bb.min + bb_pos,
            max: bb.max + bb_pos,
        };
        if n_self.min.x() > n_bb.min.x() && n_bb.min.x() < n_self.max.x() {
            return true;
        }
        if n_self.min.y() > n_bb.min.y() && n_bb.min.y() < n_self.max.y() {
            return true;
        }
        if n_self.min.z() > n_bb.min.z() && n_bb.min.z() < n_self.max.z() {
            return true;
        }
        false
    }

    fn intersect(&self, self_pos: Vec3<T>, bb: Self, bb_pos: Vec3<T>) -> Option<RayTraceResult<T>> {
        todo!()
    }

    fn scale(&self, multiplier: T) -> Self {
        Self {
            min: self.min * multiplier,
            max: self.max * multiplier,
        }
    }
}
