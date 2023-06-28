use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Write},
};

use rand::Rng;

use crate::{
    buffered::{reader::read_u64, writer::write_u64},
    util::{bitarr::BitArr, string::Extention},
};

use super::BLOCK_SIZES;

pub const VALID_EXTENTIONS: [&str; 3] = ["HA1", "HA2", "HA3"];
pub const EXTENTIONS: [&str; 3] = ["HE1", "HE2", "HE3"];

pub fn corrupt(path: &str, prob1: f32, prob2: f32) -> Result<(), Error> {
    let extention;
    let block_size;

    if let Some(index) = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        extention = EXTENTIONS[index];
        block_size = BLOCK_SIZES[index];
    } else {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let mut reader = BufReader::new(File::open(&path)?);
    let mut writer = BufWriter::new(File::create(path.with_extention(extention))?);

    let mut n_blocks: u64 = 0;
    let mut file_size: u64 = 0;
    read_u64(&mut reader, &mut n_blocks)?;
    read_u64(&mut reader, &mut file_size)?;
    write_u64(&mut writer, &n_blocks)?;
    write_u64(&mut writer, &file_size)?;

    let mut buffer = Vec::from([0; 1].repeat(block_size / 8));
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let mut rng3 = rand::thread_rng();

    for _i in 0..n_blocks {
        reader.read_exact(&mut buffer)?;

        let errors: f64 = rng1.gen();

        if errors < prob1 as f64 {
            let pos: f64 = rng2.gen();
            buffer
                .as_mut_slice()
                .flip_bit((pos * block_size as f64) as usize);
        } else if errors < (prob2 + prob1) as f64 {
            let pos1: f64 = rng2.gen();
            buffer
                .as_mut_slice()
                .flip_bit((pos1 * block_size as f64) as usize);
            let pos2: f64 = rng3.gen();
            buffer
                .as_mut_slice()
                .flip_bit((pos2 * block_size as f64) as usize);
        }

        writer.write_all(&mut buffer)?;
    }

    Ok(())
}
