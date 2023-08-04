use crate::huffman::CharInfo;
use crate::huffman::CARD_ORIG;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Seek, Write},
};

use crate::util::{bitarr::BitArr, string::Extention, typed_io::TypedWrite};

use super::{get_char_info, BUFF_SIZE};

pub const VALID_EXTENTIONS: [&str; 3] = ["txt", "doc", "docx"];
pub const EXTENTION: &str = "huf";

struct Encoder {
    distinct: u32,
    table: Vec<(u8, Vec<u8>)>,
}

pub fn compress(path: &str) -> Result<()> {
    if let None = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let fd = File::open(path)?;
    let file_size = fd.metadata()?.len();
    let mut reader = BufReader::new(fd);
    let mut writer = BufWriter::new(File::create(path.with_extention(EXTENTION))?);

    let mut buffer = [0].repeat(BUFF_SIZE);
    let mut rem_buf = 0;
    let buf_bits = BUFF_SIZE * 8;

    let encoder = Encoder::new(&mut reader)?;

    writer.write_u64(file_size)?;
    encoder.write(&mut writer)?;

    let table = encoder.table;
    while let Some(Ok(byte)) = (&mut reader).bytes().next() {
        let (len, code) = &table[byte as usize];

        if buf_bits - rem_buf < *len as usize {
            let mut dif = buf_bits - rem_buf;
            buffer.put_bits(code, rem_buf as usize, 0, dif);

            writer.write_all(&mut buffer)?;
            rem_buf = 0;

            dif = *len as usize - dif;
            buffer.put_bits(code, rem_buf, 0, dif);
            rem_buf += dif
        } else {
            buffer.put_bits(code, rem_buf, 0, *len as usize);

            rem_buf += *len as usize;
        }
    }
    writer.write_all(&mut buffer)?;

    Ok(())
}

impl Encoder {
    fn new<R: Read + Seek>(reader: R) -> Result<Encoder> {
        let mut table = Vec::with_capacity(CARD_ORIG);

        let mut info_arr: Vec<Vec<CharInfo>> = Vec::from([get_char_info(reader)?]);
        let distinct = info_arr[0].len() as u32;

        info_arr[0].sort_by(|x, y| x.prob.partial_cmp(&y.prob).expect("Invalid probability"));

        while let [first, second, rest @ ..] = &mut info_arr
            .last()
            .expect("Error generating new encoding")
            .as_slice()
        {
            let mut level = Vec::with_capacity(rest.len() + 1);
            let merged = first.merge(second);

            let mut passed = false;
            for char_info in rest {
                if !passed && char_info.prob > merged.prob {
                    level.push(merged.clone());
                    passed = true;
                }
                level.push(CharInfo::new(char_info.prob));
            }
            if !passed {
                level.push(merged);
            }
            info_arr.push(level);
        }
        info_arr.pop();

        if let [ref mut first, ref mut second] = info_arr
            .last_mut()
            .expect("Error generating new encoding")
            .as_mut_slice()
        {
            first.code.push(false);
            second.code.push(true);
        }

        for _i in 2..distinct {
            let mut last = info_arr.pop().unwrap();
            if let [ref mut first, ref mut second, ref mut rest @ ..] = info_arr
                .last_mut()
                .expect("Error generating new encoding")
                .as_mut_slice()
            {
                let mut rest_iter = rest.iter_mut().rev();

                while let Some(val) = last.pop() {
                    if val.orig == 1 {
                        first.code = val.code.clone();
                        first.code.push(false);
                        second.code = val.code;
                        second.code.push(true);
                    } else {
                        rest_iter
                            .next()
                            .expect("Error generating new encoding")
                            .code = val.code;
                    }
                }
            }
        }

        for _i in 0..CARD_ORIG {
            table.push((0, Vec::new()));
        }

        for i in 0..distinct {
            let CharInfo { orig, code, .. } = &info_arr[0][i as usize];

            table[*orig as usize] = (code.len() as u8, bool_to_u8_vec(code));
        }

        Ok(Encoder { table, distinct })
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut buffer: Vec<u8> = Vec::new();

        writer.write_u32(self.distinct as u32)?;

        for i in 0..self.table.len() {
            let entry = &self.table[i];

            if entry.1.is_empty() {
                continue;
            }

            buffer.clear();
            let (len, code) = &entry;

            buffer.push(i as u8);
            buffer.push(*len);
            buffer.extend_from_slice(&code);
            writer.write_all(&mut buffer)?;
        }

        Ok(())
    }
}

fn bool_to_u8_vec(vec: &Vec<bool>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for i in 0..vec.len() {
        if i % 8 == 0 {
            res.push(0);
        }
        if vec[i] {
            res.set_bit(i);
        }
    }

    res
}
