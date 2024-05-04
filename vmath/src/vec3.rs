use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use num_traits::{One, Pow, Zero};

use crate::coordinate;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec3<T: Copy>([T; 3]);
impl<T: Copy> Vec3<T> {
    #[inline]
    pub fn x(&self) -> T {
        self.0[coordinate::X]
    }

    #[inline]
    pub fn y(&self) -> T {
        self.0[coordinate::Y]
    }

    #[inline]
    pub fn z(&self) -> T {
        self.0[coordinate::Z]
    }
    #[inline]
    pub fn set_x(&mut self, v: T) {
        self.0[coordinate::X] = v
    }

    #[inline]
    pub fn set_y(&mut self, v: T) {
        self.0[coordinate::Y] = v
    }

    #[inline]
    pub fn set_z(&mut self, v: T) {
        self.0[coordinate::Z] = v
    }
}

impl<T: Add<T, Output = T> + Copy> Add for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self([self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()])
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self([self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()])
    }
}

impl<T: Mul<T, Output = T> + Sub<T, Output = T> + Copy> Mul for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self([self.y() * rhs, self.z() * rhs, self.x() * rhs])
    }
}

impl<T: Add<T, Output = T> + Copy> AddAssign for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0[coordinate::X] = self.0[coordinate::X] + rhs.0[coordinate::X];
        self.0[coordinate::Y] = self.0[coordinate::Y] + rhs.0[coordinate::Y];
        self.0[coordinate::Z] = self.0[coordinate::Z] + rhs.0[coordinate::Z];
    }
}

impl<T: Sub<T, Output = T> + Copy> SubAssign for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0[coordinate::X] = self.0[coordinate::X] - rhs.0[coordinate::X];
        self.0[coordinate::Y] = self.0[coordinate::Y] - rhs.0[coordinate::Y];
        self.0[coordinate::Z] = self.0[coordinate::Z] - rhs.0[coordinate::Z];
    }
}

impl<T: Mul<T, Output = T> + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0[coordinate::X] = self.0[coordinate::X] * rhs;
        self.0[coordinate::Y] = self.0[coordinate::Y] * rhs;
        self.0[coordinate::Z] = self.0[coordinate::Z] * rhs;
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Vec3<T> {
    #[inline]
    pub fn length_sqr(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl<
        T: Mul<T, Output = T>
            + Add<T, Output = T>
            + Pow<T, Output = T>
            + Div<T, Output = T>
            + One
            + Copy,
    > Vec3<T>
{
    #[inline]
    pub fn length(&self) -> T {
        T::pow(self.length_sqr(), self.x())
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        *self * (T::one() / self.length())
    }
}

impl<T: Zero + Copy> Default for Vec3<T> {
    #[inline]
    fn default() -> Self {
        Self([T::zero(); 3])
    }
}

#[inline]
pub fn new_zero<T: Zero + Copy>() -> Vec3<T> {
    Vec3::default()
}

#[inline]
pub fn new<T: Zero + Copy>(x: T, y: T, z: T) -> Vec3<T> {
    let mut v: Vec3<T> = Vec3::default();
    v.set_x(x);
    v.set_y(y);
    v.set_z(z);
    v
}
