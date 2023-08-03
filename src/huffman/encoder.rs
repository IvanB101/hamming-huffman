use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Seek, Write},
};

use crate::util::{bitarr::BitArr, string::Extention, typed_io::TypedWrite};

use super::BUFF_SIZE;

pub const VALID_EXTENTIONS: [&str; 3] = ["txt", "doc", "docx"];
pub const EXTENTION: &str = "huf";
const CARD_ORIG: usize = 128;

#[derive(Default, Clone, Debug)]
struct CharInfo {
    orig: u8,
    code: Vec<bool>,
    prob: f64,
}

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
    fn new<R: Read + Seek>(mut reader: R) -> Result<Encoder> {
        let mut info_arr: Vec<Vec<CharInfo>> = Vec::new();
        let mut table = Vec::with_capacity(CARD_ORIG);
        let mut counter: Vec<u8> = Vec::from([0; 1].repeat(CARD_ORIG));
        let mut distinct: u32 = 0;

        while let Some(Ok(val)) = (&mut reader).bytes().next() {
            if counter[val as usize] == 0 {
                distinct += 1;
            }
            counter[val as usize] += 1;
        }
        reader.rewind()?;

        info_arr.push(Vec::new());
        for i in 0..CARD_ORIG {
            if counter[i] != 0 {
                info_arr[0].push(CharInfo {
                    orig: i as u8,
                    code: Vec::new(),
                    prob: (counter[i] as f64) / (distinct as f64),
                })
            }
        }
        info_arr[0].sort_by(|x, y| x.prob.partial_cmp(&y.prob).unwrap());

        for i in (2..=(distinct - 1)).rev() {
            let mut level = Vec::with_capacity(i as usize);
            let prev_lev = &mut info_arr.last().unwrap();

            level.push(CharInfo {
                orig: 1,
                code: Vec::new(),
                prob: prev_lev[0].prob + prev_lev[1].prob,
            });

            for k in 2..prev_lev.len() {
                let mut temp = prev_lev[k].clone();
                temp.orig = 0;
                level.push(temp);
            }
            level.sort_by(|x, y| x.prob.partial_cmp(&y.prob).unwrap());

            info_arr.push(level);
        }

        info_arr.last_mut().unwrap()[0].code.push(false);
        info_arr.last_mut().unwrap()[1].code.push(true);
        for _i in 2..distinct {
            let mut last = info_arr.pop().unwrap();
            let current = info_arr.last_mut().unwrap();

            let mut k = last.len();
            while let Some(val) = last.pop() {
                if val.orig == 1 {
                    current[0].code = val.code.clone();
                    current[0].code.push(false);
                    current[1].code = val.code;
                    current[1].code.push(true);
                } else {
                    current[k].code = val.code;
                    k -= 1;
                }
            }
        }

        for _i in 0..CARD_ORIG {
            table.push((0, Vec::new()));
        }

        for i in 0..distinct {
            let entry = &info_arr[0][i as usize];

            let mut code: Vec<u8> = Vec::new();
            let length = entry.code.len();

            for i in 0..length {
                if i % 8 == 0 {
                    code.push(0);
                }
                if entry.code[i] {
                    code.as_mut_slice().set_bit(i);
                }
            }

            table[entry.orig as usize] = (length as u8, code);
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
