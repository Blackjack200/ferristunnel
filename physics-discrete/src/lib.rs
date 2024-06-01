use vmath::vec3::Vec3;

pub type ContinuousSpaceMeasure = f64;
pub type ContinuousTimeMeasure = f64;
pub type DiscreteTimeMeasure = i64;

type Position = Vec3<ContinuousSpaceMeasure>;

pub trait TimeSpace {
    fn tick() -> ContinuousSpaceMeasure;
}

pub trait DiscreteMovementObject {
    fn current_tick(&self) -> &DiscreteTimeMeasure;

    fn position(&mut self) -> &mut Position;
    fn delta_position(&mut self) -> &mut Position;

    fn yaw(&mut self) -> &mut ContinuousSpaceMeasure;
    fn delta_yaw(&mut self) -> &mut ContinuousSpaceMeasure;

    fn pitch(&mut self) -> &mut ContinuousSpaceMeasure;
    fn delta_pitch(&mut self) -> &mut ContinuousSpaceMeasure;

    fn next(&self);
}

pub trait DiscreteMovementInterpolator {
    fn interpolate_vector<T: TimeSpace>(
        v_1: Position,
        v_2: Position,
        tck: DiscreteTimeMeasure,
        dt: ContinuousTimeMeasure,
    ) -> Position;
    fn interpolate_scalar<T: TimeSpace>(
        v_1: ContinuousSpaceMeasure,
        v_2: ContinuousSpaceMeasure,
        tck: DiscreteTimeMeasure,
        dt: ContinuousTimeMeasure,
    ) -> ContinuousSpaceMeasure;
}

pub trait Computer {}
