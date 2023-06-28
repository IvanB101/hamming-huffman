pub mod compress;
pub mod decompress;

use std::{
    fs::File,
    io::{BufReader, Result},
};

use self::compress::Encoder;

const BUFF_SIZE: usize = 1024;

struct HuffmanInfo {
    pub comp_size: u32,
    pub file_size: u32,
    pub table_size: u32,
    pub table: Vec<TableEntry>,
}

struct TableEntry {
    pub orig: char,
    pub prob: f32,
    pub code: String,
}

pub fn get_info(path: &str) -> Result<HuffmanInfo> {
    let mut file_size;
    let mut table_size;
    let mut table = Vec::new();

    let fd = File::open(path)?;
    let size = fd.metadata()?.len();
    let reader = BufReader::new(&mut fd);
    read_u64(&mut reader, &mut file_size)?;
    let encoder = Encoder::read_from_file(&mut reader);

    while let (Some((orig, _prob)), Some((len, code))) = (encoder.pop_nodes(), encoder.pop_table())
    {
        let mut aux = Vec::new();

        let mut mask = 1 << 7;
        for i in 0..len {
            if code[i / 8] & mask != 0 {
                aux.push('1');
            } else {
                aux.push('0');
            }
            if code % 8 == 0 {
                mask = 1 << 7;
            }
        }

        table.push(TableEntry {
            orig,
            prob,
            code: aux,
        })
    }

    let comp_size = size - table_size;

    Ok(HuffmanInfo {
        comp_size,
        file_size,
        table_size,
        table,
    })
}
