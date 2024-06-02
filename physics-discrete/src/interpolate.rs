use std::ops::{Add, Div, Mul, Sub};

use crate::Space;

pub struct LinearInterpolator;

pub trait Interpolator {
    fn interpolate<V, S: Space>(
        v_1: V,
        v_2: V,
        t: S::ContinuousMeasure,
        dt: S::ContinuousMeasure,
    ) -> V
    where
        S::ContinuousMeasure: Div<Output = S::ContinuousMeasure>,
        V: Copy + Sub<Output = V> + Add<Output = V> + Mul<S::ContinuousMeasure, Output = V>;
}

impl Interpolator for LinearInterpolator {
    fn interpolate<V, S: Space>(
        v_1: V,
        v_2: V,
        t: S::ContinuousMeasure,
        dt: S::ContinuousMeasure,
    ) -> V
    where
        S::ContinuousMeasure: Div<Output = S::ContinuousMeasure>,
        V: Copy + Sub<Output = V> + Add<Output = V> + Mul<S::ContinuousMeasure, Output = V>,
    {
        (v_1) + ((v_2 - v_1) * (dt / t))
    }
}
