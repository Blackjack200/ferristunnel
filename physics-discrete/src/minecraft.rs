use crate::*;

#[derive(Default)]
pub struct MinecraftSpace;

impl Space for MinecraftSpace {
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
