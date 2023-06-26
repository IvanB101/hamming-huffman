use std::io::{BufRead, Read, Result};

pub fn read_u8<R: BufRead>(reader: &mut R) -> Result<u8> {
    Ok(reader.bytes().next().transpose()?.unwrap())
}

pub fn read_u32<R: BufRead>(reader: &mut R, result: &mut u32) -> std::io::Result<()> {
    let mut buffer = [0_u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut buffer)?;
    *result = u32::from_le_bytes(buffer);
    Ok(())
}

pub fn read_u64<R: BufRead>(reader: &mut R, result: &mut u64) -> std::io::Result<()> {
    let mut buffer = [0_u8; std::mem::size_of::<u64>()];
    reader.read_exact(&mut buffer)?;
    *result = u64::from_le_bytes(buffer);
    Ok(())
}

pub fn read_f64<R: BufRead>(reader: &mut R, result: &mut f64) -> std::io::Result<()> {
    let mut buffer = [0_u8; std::mem::size_of::<f64>()];
    reader.read_exact(&mut buffer)?;
    *result = f64::from_le_bytes(buffer);
    Ok(())
}
