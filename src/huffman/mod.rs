pub mod decoder;
pub mod encoder;

use std::io::Result;

const BUFF_SIZE: usize = 1024;

pub struct HuffmanInfo {
    pub comp_size: u64,
    pub file_size: u64,
    pub table_size: u64,
    pub table: Vec<TableEntry>,
}

pub struct TableEntry {
    pub orig: char,
    pub prob: f64,
    pub code: String,
}

pub fn get_stats(path: &str) -> Result<HuffmanInfo> {
    todo!()
}
