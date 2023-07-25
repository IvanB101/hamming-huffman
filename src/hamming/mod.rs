pub mod decoder;
pub mod encoder;
pub mod noise;

use std::io::Result;

use crate::util::{bitarr::BitArr, string::Extention};

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
