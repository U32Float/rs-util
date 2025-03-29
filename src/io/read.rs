use anyhow::Result;
use std::io::{BufRead, Read};

// -----------------------------------------------------------------------------

pub trait ReadExt {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;

    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i64(&mut self) -> Result<i64>;

    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]>;
    fn read_vec(&mut self, n: usize) -> Result<Vec<u8>>;

    fn read_pascal_string(&mut self) -> Result<String> {
        let len = self.read_u32()? as usize;
        self.read_utf8(len)
    }

    fn read_utf8(&mut self, n: usize) -> Result<String> {
        let data = self.read_vec(n)?;
        Ok(String::from_utf8(data)?)
    }

    fn matches_signature(&mut self, signature: &[u8; 4]) -> bool {
        self.read_array().ok() == Some(*signature)
    }
}

impl<R: Read> ReadExt for R {
    #[inline(always)]
    fn read_u8(&mut self) -> Result<u8> {
        let mut data = [0; 1];
        self.read_exact(&mut data)?;
        Ok(data[0])
    }

    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16> {
        let mut data = [0; 2];
        self.read_exact(&mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;
        Ok(u32::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_u64(&mut self) -> Result<u64> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;
        Ok(u64::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_i8(&mut self) -> Result<i8> {
        let mut data = [0; 1];
        self.read_exact(&mut data)?;
        Ok(i8::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_i16(&mut self) -> Result<i16> {
        let mut data = [0; 2];
        self.read_exact(&mut data)?;
        Ok(i16::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_i32(&mut self) -> Result<i32> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;
        Ok(i32::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_i64(&mut self) -> Result<i64> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;
        Ok(i64::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_f32(&mut self) -> Result<f32> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;
        Ok(f32::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_f64(&mut self) -> Result<f64> {
        let mut data = [0; 8];
        self.read_exact(&mut data)?;
        Ok(f64::from_be_bytes(data))
    }

    #[inline(always)]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut data = [0; N];
        self.read_exact(&mut data)?;
        Ok(data)
    }

    #[inline(always)]
    fn read_vec(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut data = vec![0; n];
        self.read_exact(&mut data)?;
        Ok(data)
    }
}

// -----------------------------------------------------------------------------

#[inline(always)]
pub fn pascal_string_unpadded(reader: &mut impl Read) -> Result<(usize, String)> {
    let len = reader.read_u8()? as usize;
    let data = reader.read_vec(len)?;

    Ok((1 + len, String::from_utf8(data)?))
}

#[inline(always)]
pub fn pascal_string_padded<const MULTIPLE: usize>(
    reader: &mut impl Read,
) -> Result<(usize, String)> {
    let len = reader.read_u8()? as usize;
    let padding = (MULTIPLE - ((len + 1) % MULTIPLE)) % MULTIPLE;
    let data = reader.read_vec(len + padding)?;

    Ok((
        len + padding + 1,
        String::from_utf8_lossy(&data[..len]).to_string(),
    ))
}

#[inline(always)]
pub fn unicode_string_padded<const PADDING: usize>(reader: &mut impl BufRead) -> Result<String> {
    let len = reader.read_u32()? as usize;
    let data = reader.read_vec(len * 2 + PADDING)?;
    Ok(String::from_utf16be_lossy(&data[..len * 2]))
}

#[inline(always)]
pub fn unicode_string_unpadded(reader: &mut impl BufRead) -> Result<String> {
    let len = reader.read_u32()? as usize;
    let data = reader.read_vec(len * 2)?;
    Ok(String::from_utf16be_lossy(&data))
}
