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
