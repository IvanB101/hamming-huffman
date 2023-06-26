pub mod decoder;
pub mod encoder;

use crate::util::bitarr::BitArr;

pub const MAX_BLOCK_SIZE: usize = 65536;
pub const MAX_EXPONENT: usize = 16;

pub fn init_masks<'a>() -> [[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT] {
    let mut masks = [[0 as u8; MAX_BLOCK_SIZE]; MAX_EXPONENT];

    let mut m = 1;
    for i in 0..MAX_EXPONENT {
        for k in 0..MAX_BLOCK_SIZE {
            if (k + 1) % m != 0 {
                let row = &mut masks[i];
                row.as_mut_slice().set_bit(k);
            }
        }
        m <<= 1;
    }

    masks
}
