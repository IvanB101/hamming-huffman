use crate::buffered::writer::write_u64;
use crate::util::{bitarr::BitArr, string::Extention};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write};

use super::{MAX_BLOCK_SIZE, MAX_EXPONENT};

pub const VALID_EXTENTIONS: [&str; 1] = ["txt"];
pub const EXTENTIONS: [&str; 3] = ["HA1", "HA2", "HA3"];
pub const BLOCK_SIZES: [usize; 3] = [32, 2048, 65536];
pub const EXPONENTS: [usize; 3] = [5, 11, 16];

pub fn encode(
    path: &str,
    block_size: usize,
    masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT],
) -> Result<(), Error> {
    let exponent;
    let extention;

    if let Some(index) = BLOCK_SIZES.iter().position(|&x| x == block_size) {
        exponent = EXPONENTS[index];
        extention = EXTENTIONS[index];
    } else {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let fd = File::open(path)?;
    let file_size = fd.metadata()?.len() as usize;
    let mut reader = BufReader::new(fd);
    let mut writer = BufWriter::new(File::create(path.with_extention(extention))?);

    let info_bits: usize = block_size - exponent - 1;
    let block_size_bytes: usize = block_size / 8;
    let n_blocks: usize = if (file_size * 8) % (info_bits) != 0 {
        file_size * 8 / info_bits + 1
    } else {
        file_size * 8 / info_bits
    };

    let mut rem_buf: usize = 0;
    let mut block: Vec<u8> = Vec::with_capacity(block_size_bytes);
    let mut buffer: Vec<u8> = Vec::with_capacity(block_size_bytes);

    for _i in 0..block_size_bytes {
        block.push(0);
        buffer.push(0);
    }

    write_u64(&mut writer, &(n_blocks as u64))?;
    write_u64(&mut writer, &(file_size as u64))?;

    for _i in 0..n_blocks {
        rem_buf = pack(&mut reader, &mut block, &mut buffer, block_size, rem_buf)?;

        protect(&mut block, exponent, masks);

        writer.write_all(&mut block)?;
    }

    Ok(())
}

fn protect(mut block: &mut [u8], exponent: usize, masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT]) {
    let mut pos = 1;
    for i in 0..exponent {
        let flip = block.masked_parity(&masks[i]);

        if flip {
            block.flip_bit(pos - 1);
        }
        pos <<= 1;
    }
}

fn pack<'a>(
    reader: &mut BufReader<File>,
    mut block: &'a mut [u8],
    buffer: &'a mut [u8],
    block_size: usize,
    rem_buf: usize,
) -> Result<usize, std::io::Error> {
    let mut remain = block_size - 2;
    let mut start_from = block_size - rem_buf;
    let mut start_to = 2;
    let mut size = 1;

    while remain > 0 {
        println!("Size: {}", size);
        println!("Start from: {}", start_from);
        println!("Start to: {}", start_to);
        println!("Remain: {}", remain);
        if size > block_size - start_from {
            println!("Leer");
            let mut dif = block_size - start_from;

            block.put(&buffer, start_to, start_from, dif);
            start_to += dif;

            if let Err(_) = reader.read_exact(buffer) {
                println!("last");
                let mut temp = Vec::with_capacity(block_size / 8);
                reader.read_to_end(&mut temp)?;

                for i in 0..temp.len() {
                    buffer[i] = temp[i]
                }
            }
            start_from = 0;
            dif = size - dif;

            block.put(&buffer, start_to, start_from, dif);
            start_from += dif;
            start_to += dif + 1;

            remain -= size + 1;
            size = (size << 1) + 1;
        } else {
            println!("Normal");
            block.put(&buffer, start_to, start_from, size);

            remain -= size + 1;
            start_from += size;
            start_to += size + 1;
            size = (size << 1) + 1;
        }
    }

    Ok(block_size - start_from)
}
