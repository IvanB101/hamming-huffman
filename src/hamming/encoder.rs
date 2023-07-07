use crate::util::typed_io::TypedWrite;
use crate::util::{bitarr::BitArr, string::Extention};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write};

use super::{BLOCK_SIZES, EXPONENTS, MAX_BLOCK_SIZE, MAX_EXPONENT};

pub const VALID_EXTENTIONS: [&str; 1] = ["txt"];
pub const EXTENTIONS: [&str; 3] = ["HA1", "HA2", "HA3"];

pub fn encode(
    path: &str,
    block_size: usize,
    masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT],
) -> Result<(), Error> {
    let exponent;
    let extention;

    if let None = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    if let Some(index) = BLOCK_SIZES.iter().position(|&x| x == block_size) {
        exponent = EXPONENTS[index];
        extention = EXTENTIONS[index];
    } else {
        return Err(Error::new(ErrorKind::Other, "Invalid block size"));
    }

    let fd = File::open(path)?;
    let file_size = fd.metadata()?.len();
    let mut reader = BufReader::new(fd);
    let mut writer = BufWriter::new(File::create(path.with_extention(extention))?);

    let info_bits = block_size - exponent - 1;
    let block_size_bytes = block_size / 8;
    let n_blocks = if (file_size * 8) % (info_bits as u64) != 0 {
        file_size * 8 / info_bits as u64 + 1
    } else {
        file_size * 8 / info_bits as u64
    };

    let mut rem_buf: usize = block_size as usize;
    let mut block: Vec<u8> = Vec::with_capacity(block_size_bytes);
    let mut buffer: Vec<u8> = Vec::with_capacity(block_size_bytes);

    for _i in 0..block_size_bytes {
        block.push(0);
        buffer.push(0);
    }

    writer.write_u64(n_blocks)?;
    writer.write_u64(file_size)?;

    for _i in 0..n_blocks {
        rem_buf = pack(&mut reader, &mut block, &mut buffer, block_size, rem_buf)?;

        protect(&mut block, exponent, masks);

        writer.write_all(&mut block)?;
    }

    Ok(())
}

fn protect(block: &mut [u8], exponent: usize, masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT]) {
    let mut pos = 1;
    for i in 0..exponent {
        let flip = block.masked_parity(&masks[i]);

        if flip {
            block.flip_bit(pos - 1);
        }
        pos <<= 1;
    }
    let par = block.parity();
    let block_size = block.len() * 8;
    if par {
        block.flip_bit(block_size - 1);
    }
}

fn pack<'a, R: Read>(
    reader: &mut R,
    block: &'a mut [u8],
    buffer: &'a mut [u8],
    block_size: usize,
    offset: usize,
) -> Result<usize, std::io::Error> {
    let mut remain = block_size - 2;
    let mut start_from = offset;
    let mut start_to = 2;
    let mut size = 1;

    while remain > 0 {
        if size > block_size - start_from {
            let mut dif = block_size - start_from;

            block.put_bits(&buffer, start_to, start_from, dif);
            start_to += dif;

            if let Err(_) = reader.read_exact(buffer) {
                let mut temp = Vec::with_capacity(block_size / 8);
                reader.read_to_end(&mut temp)?;

                for i in 0..temp.len() {
                    buffer[i] = temp[i]
                }
            }
            start_from = 0;
            dif = size - dif;

            block.put_bits(&buffer, start_to, start_from, dif);
            start_from += dif;
            start_to += dif + 1;
        } else {
            block.put_bits(&buffer, start_to, start_from, size);

            start_from += size;
            start_to += size + 1;
        }
        remain -= size + 1;
        size = (size << 1) + 1;
    }

    Ok(start_from)
}
