use std::ops::AddAssign;

use num_traits::One;

use crate::*;

#[derive(Default)]
pub struct MinecraftTimeSpace;

impl Space for MinecraftTimeSpace {
    type ContinuousMeasure = f32;
    type DiscreteTimeMeasure = u64;
    type Position = Vec3<Self::ContinuousMeasure>;

    fn per_tick() -> Self::ContinuousMeasure {
        1.0 / 20.0
    }

    fn per_second() -> Self::DiscreteTimeMeasure {
        20
    }
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
