use std::io::{BufWriter, Result, Write};

pub fn write_u64<W: Write>(writer: &mut BufWriter<W>, number: &u64) -> Result<()> {
    let mut buffer = number.to_le_bytes();
    writer.write_all(&mut buffer)
}

pub fn write_u32<W: Write>(writer: &mut BufWriter<W>, number: &u32) -> Result<()> {
    let mut buffer = number.to_le_bytes();
    writer.write_all(&mut buffer)
}
