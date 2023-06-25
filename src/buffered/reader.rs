use std::io::BufRead;

fn read_u8<R: BufRead>(reader: &mut R, result: &mut u8) -> std::io::Result<()> {
    let mut buffer = [0_u8; 1];
    reader.read_exact(&mut buffer)?;
    *result = u8::from_le_bytes(buffer);
    Ok(())
}

fn read_u32<R: BufRead>(reader: &mut R, result: &mut u32) -> std::io::Result<()> {
    let mut buffer = [0_u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut buffer)?;
    *result = u32::from_le_bytes(buffer);
    Ok(())
}

fn read_u64<R: BufRead>(reader: &mut R, result: &mut u64) -> std::io::Result<()> {
    let mut buffer = [0_u8; std::mem::size_of::<u64>()];
    reader.read_exact(&mut buffer)?;
    *result = u64::from_le_bytes(buffer);
    Ok(())
}
