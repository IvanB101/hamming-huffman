use std::io::{Read, Result, Write};

pub trait TypedRead {
    fn read_u32(&mut self) -> Result<u32>;

    fn read_u64(&mut self) -> Result<u64>;

    fn read_f64(&mut self) -> Result<f64>;
}

pub trait TypedWrite {
    fn write_u32(&mut self, data: u32) -> Result<()>;

    fn write_u64(&mut self, data: u64) -> Result<()>;
}

impl<R> TypedRead for R
where
    R: Read,
{
    fn read_u32(&mut self) -> Result<u32> {
        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buffer = [0_u8; std::mem::size_of::<u64>()];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut buffer = [0_u8; std::mem::size_of::<f64>()];
        self.read_exact(&mut buffer)?;
        Ok(f64::from_le_bytes(buffer))
    }
}

impl<W> TypedWrite for W
where
    W: Write,
{
    fn write_u32(&mut self, data: u32) -> Result<()> {
        let mut buffer = data.to_le_bytes();
        self.write_all(&mut buffer)
    }

    fn write_u64(&mut self, data: u64) -> Result<()> {
        let mut buffer = data.to_le_bytes();
        self.write_all(&mut buffer)
    }
}
