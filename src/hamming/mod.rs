pub mod decoder;
pub mod encoder;
pub mod noise;

use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use crate::util::string::Extention;

pub const BLOCK_SIZES: [usize; 3] = [32, 2048, 65536];
pub const EXPONENTS: [usize; 3] = [5, 11, 16];

const MAX_BLOCK_SIZE: usize = 65536;
const MAX_EXPONENT: usize = 16;

const MASKS: [[u8; MAX_BLOCK_SIZE / 8]; MAX_EXPONENT] = init_masks();

pub struct HammingStats {
    orig_size: u64,
    proc_size: u64,
    info_bits: u64,
    proc_bits: u64,
    fill_bits: u64,
}

pub fn get_stats(path: &str) -> Result<()> {
    let exponent;
    let block_size;

    if let Some(mut index) = decoder::VALID_EXTENTIONS
        .iter()
        .position(|&x| path.has_extention(x))
    {
        index %= 3;
        exponent = EXPONENTS[index];
        block_size = BLOCK_SIZES[index];

        todo!();
    } else {
        todo!();
    }
}

pub const fn init_masks<'a>() -> [[u8; MAX_BLOCK_SIZE / 8]; MAX_EXPONENT] {
    let mut masks = [[0 as u8; MAX_BLOCK_SIZE / 8]; MAX_EXPONENT];

    let mut m = 1;
    let mut i = 0;
    let mut k = 0;
    while i < MAX_EXPONENT {
        while k < MAX_BLOCK_SIZE {
            if (k + 1) & m != 0 {
                masks[i][k / 8] |= 1 << 7 - k % 8;
            }
            k += 1;
        }

        m <<= 1;
        i += 1;
    }

    masks
}

#[test]
fn no_err_32_block() {
    generic_test(0, true, None);
}
#[test]
fn no_err_2048_block() {
    generic_test(1, true, None);
}
#[test]
fn no_err_65536_block() {
    generic_test(2, true, None);
}
#[test]
fn one_err_32_block() {
    generic_test(0, true, Some((1.0, 0.0)));
}
#[test]
fn one_err_2048_block() {
    generic_test(1, true, Some((1.0, 0.0)));
}
#[test]
fn one_err_65536_block() {
    generic_test(2, true, Some((1.0, 0.0)));
}
#[test]
#[should_panic]
fn two_err_32_block() {
    generic_test(0, true, Some((0.0, 1.0)));
}
#[test]
#[should_panic]
fn two_err_2048_block() {
    generic_test(1, true, Some((0.0, 1.0)));
}
#[test]
#[should_panic]
fn two_err_65536_block() {
    generic_test(2, true, Some((0.0, 1.0)));
}

fn generic_test(index: usize, correct: bool, errors: Option<(f32, f32)>) {
    let path = "./test/test.txt";
    let protected;
    let deprotected = &path.with_extention(decoder::EXTENTIONS[index]);
    let mut orig_buf = Vec::new();
    let mut decomp_buf = Vec::new();
    if errors.is_some() {
        protected = path.with_extention(noise::EXTENTIONS[index])
    } else {
        protected = path.with_extention(encoder::EXTENTIONS[index])
    };

    encoder::encode(path, BLOCK_SIZES[index]).expect("Error encoding");

    if let Some((prob_1, prob_2)) = errors {
        noise::corrupt(
            &path.with_extention(encoder::EXTENTIONS[index]),
            prob_1,
            prob_2,
        )
        .expect("Error in corruption");
    }

    decoder::decode(&protected, correct).expect("Error decoding");

    BufReader::new(File::open(path).expect("Error opening file"))
        .read_to_end(&mut orig_buf)
        .expect("Error reading file");
    BufReader::new(File::open(deprotected).expect("Error opening file"))
        .read_to_end(&mut decomp_buf)
        .expect("Error reading file");

    assert_eq!(orig_buf, decomp_buf);
}
