use std::io;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use byteorder::{LittleEndian, ReadBytesExt};
use num_traits::{One, Pow, Zero};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl bstream::BinaryStream for Vec3<f32> {
    fn read(&mut self, out: &mut impl io::Read) -> io::Result<()> {
        self.x = ReadBytesExt::read_f32::<LittleEndian>(out)?;
        self.y = ReadBytesExt::read_f32::<LittleEndian>(out)?;
        self.z = ReadBytesExt::read_f32::<LittleEndian>(out)?;
        Ok(())
    }
    fn write(&self, out: &mut impl io::Write) -> io::Result<()> {
        byteorder::WriteBytesExt::write_f32::<LittleEndian>(out, self.x)?;
        byteorder::WriteBytesExt::write_f32::<LittleEndian>(out, self.y)?;
        byteorder::WriteBytesExt::write_f32::<LittleEndian>(out, self.z)?;
        Ok(())
    }
}

impl<T: Copy> Vec3<T> {
    #[inline]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub fn y(&self) -> T {
        self.y
    }

    #[inline]
    pub fn z(&self) -> T {
        self.z
    }
}

impl<T: Add<T, Output = T> + Copy> Add for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl<T: Mul<T, Output = T> + Sub<T, Output = T> + Copy> Mul for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y() * rhs.z() - self.z() * rhs.y(),
            y: self.z() * rhs.x() - self.x() * rhs.z(),
            z: self.x() * rhs.y() - self.y() * rhs.x(),
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}

impl<T: Add<T, Output = T> + Copy> AddAssign for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: Sub<T, Output = T> + Copy> SubAssign for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<T: Mul<T, Output = T> + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
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
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

#[inline]
pub fn new_zero<T: Zero + Copy>() -> Vec3<T> {
    Vec3::default()
}

#[inline]
pub fn new<T: Zero + Copy>(x: T, y: T, z: T) -> Vec3<T> {
    let mut v: Vec3<T> = Vec3::default();
    v.x = x;
    v.y = y;
    v.z = z;
    v
}
