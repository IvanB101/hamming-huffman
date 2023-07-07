use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Write},
};

use crate::{
    buffered::reader::read_u64,
    util::{bitarr::BitArr, string::Extention},
};

use super::{BLOCK_SIZES, EXPONENTS, MAX_BLOCK_SIZE, MAX_EXPONENT};

pub const VALID_EXTENTIONS: [&str; 6] = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"];
pub const EXTENTIONS: [&str; 6] = ["DC1", "DC2", "DC3", "DE1", "DE2", "DE3"];

pub fn decode(
    path: &str,
    corr: bool,
    masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT],
) -> Result<(), Error> {
    let exponent;
    let extention;
    let block_size;

    if let Some(mut index) = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        index %= 3;
        extention = EXTENTIONS[if corr { index } else { index + 3 }];
        exponent = EXPONENTS[index];
        block_size = BLOCK_SIZES[index];
    } else {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let mut reader = BufReader::new(File::open(&path)?);
    let mut res_fd = File::create(path.with_extention(extention))?;
    let mut writer = BufWriter::new(&mut res_fd);

    let mut n_blocks: u64 = 0;
    let mut file_size: u64 = 0;
    read_u64(&mut reader, &mut n_blocks)?;
    read_u64(&mut reader, &mut file_size)?;
    let block_size_bytes: usize = block_size / 8;

    let mut block: Vec<u8> = Vec::with_capacity(block_size_bytes);
    let mut buffer: Vec<u8> = Vec::with_capacity(block_size_bytes);

    for _i in 0..block_size_bytes {
        block.push(0);
        buffer.push(0);
    }

    let mut offset: usize = 0;
    for _i in 0..n_blocks {
        reader.read_exact(&mut block)?;

        if corr {
            correct(&mut block, exponent, masks);
        }

        offset = unpack(&mut writer, &mut block, &mut buffer, offset)?;
    }
    writer.write_all(&mut buffer)?;
    drop(writer);

    res_fd.set_len(file_size)?;

    Ok(())
}

fn correct(block: &mut [u8], exponent: usize, masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT]) {
    let mut sindrome: usize = 0;

    for i in 0..exponent {
        if block.masked_parity(&masks[i]) {
            sindrome |= (1 as usize) << i;
        }
    }
    if block.parity() && sindrome != 0 {
        block.flip_bit(sindrome - 1);
    }
    if !block.parity() && sindrome != 0 {
        println!("Doble error");
    }
}

fn unpack<'a>(
    writer: &mut BufWriter<&mut File>,
    block: &'a mut [u8],
    mut buffer: &'a mut [u8],
    offset: usize,
) -> Result<usize, std::io::Error> {
    let block_size = block.len() * 8;
    let mut remain = block_size - 2;
    let mut start_from = 2;
    let mut start_to = offset;
    let mut size = 1;

    while remain > 0 {
        if size > block_size - start_to {
            let mut dif = block_size - start_to;
            buffer.put_bits(&block, start_to, start_from, dif);

            writer.write_all(&mut buffer)?;
            buffer.into_iter().for_each(|x| *x = 0);

            start_to = 0;
            start_from += dif;

            dif = size - dif;
            buffer.put_bits(&block, start_to, start_from, dif);

            start_from += dif + 1;
            start_to += dif;
        } else {
            buffer.put_bits(&block, start_to, start_from, size);

            start_from += size + 1;
            start_to += size;
        }
        remain -= size + 1;
        size = (size << 1) + 1;
    }

    Ok(start_to)
}
