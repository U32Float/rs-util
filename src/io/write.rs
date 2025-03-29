use std::io::Write;

use anyhow::{Ok, Result};

// -----------------------------------------------------------------------------

pub trait WriteExt {
    fn write_u8(&mut self, value: u8) -> Result<usize>;
    fn write_u16(&mut self, value: u16) -> Result<usize>;
    fn write_u32(&mut self, value: u32) -> Result<usize>;
    fn write_u64(&mut self, value: u64) -> Result<usize>;

    fn write_i8(&mut self, value: i8) -> Result<usize>;
    fn write_i16(&mut self, value: i16) -> Result<usize>;
    fn write_i32(&mut self, value: i32) -> Result<usize>;
    fn write_i64(&mut self, value: i64) -> Result<usize>;

    fn write_f32(&mut self, value: f32) -> Result<usize>;
    fn write_f64(&mut self, value: f64) -> Result<usize>;

    fn write_slice(&mut self, value: &[u8]) -> Result<usize>;

    fn write_pascal_str(&mut self, value: &str) -> Result<usize> {
        let bytes = value.as_bytes();
        self.write_u32(bytes.len() as u32)?;
        let n = self.write_slice(bytes)?;
        Ok(n)
    }
}

impl<W: Write> WriteExt for W {
    #[inline(always)]
    fn write_u8(&mut self, value: u8) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_u16(&mut self, value: u16) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_u32(&mut self, value: u32) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_u64(&mut self, value: u64) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_i8(&mut self, value: i8) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_i16(&mut self, value: i16) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_i32(&mut self, value: i32) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_i64(&mut self, value: i64) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_f32(&mut self, value: f32) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_f64(&mut self, value: f64) -> Result<usize> {
        let bytes = value.to_be_bytes();
        let n = self.write(&bytes)?;
        Ok(n)
    }

    #[inline(always)]
    fn write_slice(&mut self, value: &[u8]) -> Result<usize> {
        let n = self.write(value)?;
        Ok(n)
    }
}
