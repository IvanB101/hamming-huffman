pub mod compress;
pub mod decompress;

const BUFF_SIZE: usize = 1024 * 1024 * 50;

struct CharInfo {
    orig: char,
    length: u8,
    code: String,
    prob: f64,
}

struct EncodeTree {
    card_orig: u32,
    distinct: u32,
    nodess: Vec<CharInfo>,
}
