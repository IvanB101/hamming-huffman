pub mod decoder;
pub mod encoder;

use std::io::{Read, Result, Seek};

const BUFF_SIZE: usize = 1024;
const CARD_ORIG: usize = 128;

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

#[derive(Default, Clone, Debug)]
struct CharInfo {
    orig: u8,
    code: Vec<bool>,
    prob: f64,
}

impl CharInfo {
    fn new(prob: f64) -> CharInfo {
        CharInfo {
            orig: 0,
            code: Vec::new(),
            prob,
        }
    }

    fn new_char(orig: u8, prob: f64) -> CharInfo {
        CharInfo {
            orig,
            code: Vec::new(),
            prob,
        }
    }

    fn merge(&self, info: &CharInfo) -> CharInfo {
        CharInfo {
            orig: 1,
            code: Vec::new(),
            prob: self.prob + info.prob,
        }
    }
}

pub fn get_stats(path: &str) -> Result<HuffmanInfo> {
    todo!()
}

fn get_probs<R: Read + Seek>(mut reader: R) -> Result<Vec<(u8, f64)>> {
    let mut ocurrencies = [0 as u64; CARD_ORIG];
    let mut probs = Vec::new();
    let mut counter: u64 = 0;

    for byte in (&mut reader).bytes() {
        let val = byte? as usize;

        ocurrencies[val] += 1;
        counter += 1;
    }

    for i in 0..CARD_ORIG {
        if ocurrencies[i] != 0 {
            probs.push((i as u8, ocurrencies[i] as f64 / counter as f64))
        }
    }

    reader.rewind()?;
    Ok(probs)
}
