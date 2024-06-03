use std::ops::AddAssign;

use num_traits::One;

use vmath::vec3::Vec3;

pub mod interpolate;
pub mod minecraft;

pub trait Space {
    type ContinuousMeasure;
    type DiscreteTimeMeasure;
    type Position;
    fn per_tick() -> Self::ContinuousMeasure;
    fn per_second() -> Self::DiscreteTimeMeasure;
}

pub trait Object<S: Space> {
    fn current_tick(&self) -> S::DiscreteTimeMeasure;

    fn position(&mut self) -> &mut S::Position;
    fn delta_position(&mut self) -> &mut S::Position;

    fn yaw(&mut self) -> &mut S::ContinuousMeasure;
    fn delta_yaw(&mut self) -> &mut S::ContinuousMeasure;

    fn pitch(&mut self) -> &mut S::ContinuousMeasure;
    fn delta_pitch(&mut self) -> &mut S::ContinuousMeasure;

    fn end_previous_tick(&mut self);
}

#[derive(Default)]
pub struct MovingEntity<S: Space> {
    tick: S::DiscreteTimeMeasure,
    pos: S::Position,
    delta_pos: S::Position,
    yaw: S::ContinuousMeasure,
    delta_yaw: S::ContinuousMeasure,
    pitch: S::ContinuousMeasure,
    delta_pitch: S::ContinuousMeasure,
}

impl<S: Space> Object<S> for MovingEntity<S>
where
    S::Position: Copy + Default + AddAssign,
    S::DiscreteTimeMeasure: Copy + One + AddAssign,
{
    fn current_tick(&self) -> S::DiscreteTimeMeasure {
        self.tick
    }

    fn position(&mut self) -> &mut S::Position {
        &mut self.pos
    }

    fn delta_position(&mut self) -> &mut S::Position {
        &mut self.delta_pos
    }

    fn yaw(&mut self) -> &mut S::ContinuousMeasure {
        &mut self.yaw
    }

    fn delta_yaw(&mut self) -> &mut S::ContinuousMeasure {
        &mut self.delta_yaw
    }

    fn pitch(&mut self) -> &mut S::ContinuousMeasure {
        &mut self.pitch
    }

    fn delta_pitch(&mut self) -> &mut S::ContinuousMeasure {
        &mut self.delta_pitch
    }

    fn end_previous_tick(&mut self) {
        self.pos += self.delta_pos;
        self.tick += S::DiscreteTimeMeasure::one();
        self.delta_pos = S::Position::default();
    }
}
