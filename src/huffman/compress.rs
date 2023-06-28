use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Seek, Write},
};

use crate::{
    buffered::{
        reader::{read_f64, read_u32},
        writer::{write_u32, write_u64},
    },
    ext::Extention,
    util::bitarr::BitArr,
};

use super::BUFF_SIZE;

pub const VALID_EXTENTIONS: [&str; 3] = ["txt", "doc", "docx"];
pub const EXTENTION: &str = "huf";
const CARD_ORIG: usize = 128;

#[derive(Default, Clone, Debug)]
pub struct CharInfo {
    orig: u8,
    code: Vec<bool>,
    prob: f64,
}

pub struct Encoder {
    nodes: Vec<(u8, f64)>,
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

    write_u64(&mut writer, &file_size)?;
    encoder.write_to_file(&mut writer)?;

    let table = encoder.table;
    while let Some(Ok(byte)) = (&mut reader).bytes().next() {
        let (len, code) = &table[byte as usize];

        if buf_bits - rem_buf < *len as usize {
            let mut dif = buf_bits - rem_buf;
            buffer.put(code, rem_buf as usize, 0, dif);

            writer.write_all(&mut buffer)?;
            rem_buf = 0;

            dif = *len as usize - dif;
            buffer.put(code, rem_buf, 0, dif);
            rem_buf += dif
        } else {
            buffer.put(code, rem_buf, 0, *len as usize);

            rem_buf += *len as usize;
        }
    }
    writer.write_all(&mut buffer)?;

    Ok(())
}

impl Encoder {
    fn new(reader: &mut BufReader<File>) -> Result<Encoder> {
        let mut info_arr: Vec<Vec<CharInfo>> = Vec::new();
        let mut table = Vec::with_capacity(CARD_ORIG);
        let mut counter: Vec<u8> = Vec::from([0; 1].repeat(CARD_ORIG));
        let mut distinct: u32 = 0;

        while let Some(Ok(val)) = reader.bytes().next() {
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
            table.push((Default::default(), Default::default()));
        }

        let mut nodes = Vec::with_capacity(distinct as usize);
        for i in 0..distinct {
            let entry = &info_arr[0][i as usize];

            nodes.push((entry.orig, entry.prob));

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

        Ok(Encoder { nodes, table })
    }

    fn write_to_file(&self, writer: &mut BufWriter<File>) -> Result<()> {
        let mut buffer: Vec<u8> = Vec::new();
        let distinct = self.nodes.len();

        write_u32(writer, &(distinct as u32))?;

        for i in 0..distinct {
            buffer.clear();
            let (orig, prob) = self.nodes[i];
            let (len, code) = &self.table[orig as usize];

            buffer.push(orig);
            buffer.push(*len);
            buffer.extend_from_slice(&prob.to_le_bytes());
            buffer.extend_from_slice(&code);
            writer.write_all(&mut buffer)?;
        }

        Ok(())
    }

    pub fn read_from_file(reader: &mut BufReader<File>) -> Result<Encoder> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut table = Vec::new();
        let mut nodes = Vec::new();
        let mut prob = 0.0;
        let mut distinct = 0;

        read_u32(reader, &mut distinct)?;

        for _i in 0..distinct {
            buffer.clear();
            buffer.extend_from_slice(&[0, 0]);
            reader.read_exact(&mut buffer)?;
            let orig = buffer[0];
            let len = buffer[1];

            read_f64(reader, &mut prob)?;

            buffer.clear();
            let byte_len = if len % 8 == 0 { len / 8 } else { len / 8 + 1 };
            for _i in 0..byte_len {
                buffer.push(0);
            }

            reader.read_exact(&mut buffer)?;
            let code = buffer.clone();
            nodes.push((orig, prob));
            table.push((len, code));
        }

        Ok(Encoder { nodes, table })
    }

    pub fn pop_nodes(&mut self) -> Option<(u8, f64)> {
        self.nodes.pop()
    }

    pub fn pop_table(&mut self) -> Option<(u8, Vec<u8>)> {
        self.table.pop()
    }
}
