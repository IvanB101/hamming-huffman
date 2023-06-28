use std::io::{BufReader, Write};
use std::io::{BufWriter, Read};
use std::{
    fs::File,
    io::{Error, ErrorKind, Result},
};

use crate::buffered::reader::read_u64;
use crate::util::string::Extention;

use super::compress::Encoder;

pub const VALID_EXTENTIONS: [&str; 1] = ["huf"];
const EXTENTION: &str = "dhu";

#[derive(Debug)]
struct Node {
    val: u8,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

struct DecodingTree {
    root: Option<Box<Node>>,
}

pub fn decompress(path: &str) -> Result<()> {
    if let None = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let mut reader = BufReader::new(File::open(&path)?);
    let mut res_fd = File::create(path.with_extention(EXTENTION))?;
    let mut writer = BufWriter::new(&mut res_fd);

    let mut file_size = 0;
    read_u64(&mut reader, &mut file_size)?;

    let tree = DecodingTree::new(Encoder::read_from_file(&mut reader)?)?;

    let mut anchor = &tree.root;
    while let Some(Ok(byte)) = (&mut reader).bytes().next() {
        let mut mask: u8 = 1 << 7;

        for _i in 0..8 {
            if let Some(ref node) = anchor {
                if node.val != 0 {
                    writer.write_all(&[node.val])?;
                    if let Some(ref node) = &tree.root {
                        if byte & mask != 0 {
                            anchor = &node.right;
                        } else {
                            anchor = &node.left;
                        }
                    }
                } else {
                    if byte & mask != 0 {
                        anchor = &node.right;
                    } else {
                        anchor = &node.left;
                    }
                }
            } else {
                println!("Error en decodificacion");
                break;
            }
            mask >>= 1;
        }
    }
    drop(writer);
    res_fd.set_len(file_size)?;

    Ok(())
}

impl DecodingTree {
    fn new(mut encoder: Encoder) -> Result<DecodingTree> {
        let mut root = Some(Box::new(Node {
            val: 0,
            right: None,
            left: None,
        }));

        while let (Some((orig, _prob)), Some((len, code))) =
            (encoder.pop_nodes(), encoder.pop_table())
        {
            let mut anchor = &mut root;
            let mut mask = 1 << 7;
            for i in 0..(len + 1) {
                match { anchor } {
                    &mut Some(ref mut node) => {
                        anchor = if code[(i / 8) as usize] & (mask) != 0 {
                            &mut node.right
                        } else {
                            &mut node.left
                        }
                    }
                    other => {
                        *other = Some(Box::new(Node {
                            val: 0,
                            right: None,
                            left: None,
                        }));
                        if i == len {
                            other.as_mut().unwrap().val = orig;
                            break;
                        }
                        if let &mut Some(ref mut node) = other {
                            anchor = if code[(i / 8) as usize] & (mask) != 0 {
                                &mut node.right
                            } else {
                                &mut node.left
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Error en construccion de arbol de decodificacion",
                            ));
                        }
                    }
                }
                mask >>= 1;
            }
        }

        Ok(DecodingTree { root })
    }
}
