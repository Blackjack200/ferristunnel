use io::Write;
use std::io;
use std::io::{Error, ErrorKind, Read};

use byteorder::{ReadBytesExt, WriteBytesExt};

impl<T: Read + Sized> ReaderExt for T {}

pub trait ReaderExt: Read + Sized {
    #[inline]
    fn read_bool(&mut self) -> io::Result<bool> {
        self.read_u8().map(|u| u == 1)
    }

    #[inline]
    fn read_vu32(&mut self) -> io::Result<u32> {
        read_variable(self).map(|v: u64| v as u32)
    }

    #[inline]
    fn read_vu64(&mut self) -> io::Result<u64> {
        read_variable(self)
    }

    #[inline]
    fn read_vi32(&mut self) -> io::Result<i32> {
        read_variable(self).map(|v: u64| v as i32)
    }

    #[inline]
    fn read_vi64(&mut self) -> io::Result<i64> {
        read_variable(self).map(|v: u64| v as i64)
    }
}

#[inline]
fn read_variable<T: Into<u64> + From<u64>>(r: &mut impl Read) -> io::Result<T> {
    let mut v = 0u64;
    let mut shift = 0u64;
    while shift < 64 {
        let vx = r.read_u8()? as u64;
        let val = vx & 0b01111111u64;
        v |= val << shift;
        let last = (vx & 0b10000000u64) == 0;
        if last {
            return Ok(v.into());
        }
        shift += 7;
    }
    Err(Error::new(ErrorKind::InvalidData, "varint has no ending."))
}

#[inline]
fn write_variable<T: PartialOrd + Into<u64>>(r: &mut impl Write, v: T) -> io::Result<()> {
    let mut remainder = v.into();
    while remainder > 0b01111111u8 as u64 {
        let data: u8 = (remainder & 0b01111111u8 as u64) as u8;
        remainder >>= 7;
        r.write_u8(data | 0b10000000u8)?
    }
    r.write_u8(remainder as u8)
}

impl<T: Write + Sized> WriterExt for T {}

pub trait WriterExt: Write + Sized {
    #[inline]
    fn write_bool(&mut self, v: bool) -> io::Result<()> {
        self.write_u8(v as u8)
    }

    #[inline]
    fn write_vu32(&mut self, v: u32) -> io::Result<()> {
        write_variable(self, v)
    }

    #[inline]
    fn write_vu64(&mut self, v: u64) -> io::Result<()> {
        write_variable(self, v)
    }

    #[inline]
    fn write_vi32(&mut self, v: i32) -> io::Result<()> {
        write_variable(self, v as u64)
    }

    #[inline]
    fn write_vi64(&mut self, v: i64) -> io::Result<()> {
        write_variable(self, v as u64)
    }
}

pub trait BinaryStream {
    fn read(&mut self, out: &mut impl Read) -> io::Result<()>;
    fn write(&self, out: &mut impl Write) -> io::Result<()>;
}

#[derive(Debug, Default, Clone)]
pub struct Vu32LenByteSlice(pub Vec<u8>);

pub trait EnumBinaryStream {
    fn read(out: &mut impl Read) -> io::Result<Self>
        where
            Self: Sized;
    fn write(&self, out: &mut impl Write) -> io::Result<()>;
}

impl Vu32LenByteSlice {
    #[inline]
    pub fn from<T: Into<Vec<u8>>>(v: T) -> Self {
        Self(v.into())
    }

    #[inline]
    pub fn read(out: &mut impl Read) -> io::Result<Self> {
        let mut v = Self::default();
        v.0.clear();
        let len = out.read_vu32()?;
        for _ in 0..len {
            v.0.push(out.read_u8()?)
        }
        Ok(v)
    }

    #[inline]
    pub fn write(&self, out: &mut impl Write) -> io::Result<()> {
        out.write_vu32(self.0.len() as u32)?;
        for elem in self.0.iter() {
            out.write_u8(*elem)?
        }
        Ok(())
    }
}
