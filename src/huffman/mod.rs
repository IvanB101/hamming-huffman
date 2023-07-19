pub mod decoder;
pub mod encoder;

use std::{
    fs::File,
    io::{BufReader, Result},
};

use crate::util::typed_io::TypedRead;

use self::encoder::Encoder;

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
    let mut table_size: u32 = 0;
    let mut table = Vec::new();

    let mut fd = File::open(path)?;
    let size = fd.metadata()?.len();
    let mut reader = BufReader::new(&mut fd);

    let file_size = reader.read_u64()?;
    let mut encoder = Encoder::read_from_file(&mut reader)?;

    while let (Some((orig, prob)), Some((len, code))) = (encoder.pop_nodes(), encoder.pop_table()) {
        let mut aux = String::new();

        let mut mask = 1 << 7;
        for i in 0..len {
            if code[(i / 8) as usize] & mask != 0 {
                aux.push('1');
            } else {
                aux.push('0');
            }
            if i % 8 == 0 {
                mask = 1 << 7;
            }
        }
        table_size += (10 + if len % 8 == 0 { len / 8 } else { len / 8 + 1 }) as u32;

        table.push(TableEntry {
            orig: orig as char,
            prob,
            code: aux,
        })
    }

    let comp_size = size as u32 - table_size;

    Ok(HuffmanInfo {
        comp_size: comp_size.into(),
        file_size,
        table_size: table_size.into(),
        table,
    })
}
