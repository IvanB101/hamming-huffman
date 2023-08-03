use std::io::{BufReader, Write};
use std::io::{BufWriter, Read};
use std::{
    fs::File,
    io::{Error, ErrorKind},
};
use thiserror::Error;

use crate::util::bitarr::BitArr;
use crate::util::string::Extention;
use crate::util::typed_io::TypedRead;

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

#[derive(Error, Debug)]
#[error("{message:}")]
pub struct HuffmanError {
    message: String,
    error_bytes: Vec<u64>,
}

pub fn decompress(path: &str) -> Result<(), Error> {
    if let None = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let mut reader = BufReader::new(File::open(&path)?);
    let mut res_fd = File::create(path.with_extention(EXTENTION))?;
    let mut writer = BufWriter::new(&mut res_fd);

    let file_size = reader.read_u64()?;

    let tree = DecodingTree::new(&mut reader)?;

    let mut error_bytes = Vec::new();

    let mut anchor = &tree.root;
    for (byte, index) in reader.bytes().zip(0..) {
        for bit in [byte?].iter_bits() {
            if let Some(ref node) = anchor {
                if node.val != 0 {
                    writer.write_all(&[node.val])?;
                    if let Some(ref node) = &tree.root {
                        anchor = node.get_ref_child(bit);
                    }
                } else {
                    anchor = node.get_ref_child(bit);
                }
            } else {
                error_bytes.push(index);
                break;
            }
        }
    }
    drop(writer);
    res_fd.set_len(file_size)?;

    if error_bytes.len() > 0 {
        return Err(Error::new(
            ErrorKind::Other,
            HuffmanError {
                message: "Decoding Error".into(),
                error_bytes,
            },
        ));
    }

    Ok(())
}

impl Node {
    fn new(val: u8) -> Node {
        Node {
            val,
            right: None,
            left: None,
        }
    }

    fn get_ref_mut_child<'a>(&'a mut self, val: bool) -> &'a mut Option<Box<Self>> {
        if val {
            &mut self.right
        } else {
            &mut self.left
        }
    }

    fn get_ref_child<'a>(&'a self, val: bool) -> &'a Option<Box<Self>> {
        if val {
            &self.right
        } else {
            &self.left
        }
    }
}

impl DecodingTree {
    fn new<R: Read>(mut reader: R) -> Result<DecodingTree, Error> {
        let mut root = Some(Box::new(Node::new(0)));
        let mut buffer: Vec<u8> = Vec::new();

        let distinct = reader.read_u32()?;

        for _i in 0..distinct {
            // Reading info
            buffer.clear();
            buffer.extend_from_slice(&[0, 0]);
            reader.read_exact(&mut buffer)?;
            let orig = buffer[0];
            let len = buffer[1];

            buffer.clear();
            let byte_len = if len % 8 == 0 { len / 8 } else { len / 8 + 1 };
            for _i in 0..byte_len {
                buffer.push(0);
            }

            reader.read_exact(&mut buffer)?;

            // Generating corresponding tree nodes
            let mut anchor = &mut root;
            for bit in buffer.iter_bits_len(len.into()) {
                match anchor {
                    &mut Some(ref mut node) => {
                        anchor = node.get_ref_mut_child(bit);
                    }
                    other => {
                        *other = Some(Box::new(Node::new(0)));
                        if let &mut Some(ref mut node) = other {
                            anchor = node.get_ref_mut_child(bit);
                        } else {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Unable to construct decoding tree",
                            ));
                        }
                    }
                }
            }
            *anchor = Some(Box::new(Node::new(orig)));
        }
        println!("{:?}", root);

        Ok(DecodingTree { root })
    }
}
