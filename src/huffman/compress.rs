use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, ErrorKind},
};

use crate::ext::Extention;

use super::BUFF_SIZE;

pub const VALID_EXTENTIONS: [&str; 2] = ["txt", "doc"];
pub const EXTENTION: &str = "huf";

pub fn compress(path: &str) -> Result<(), Error> {
    if let None = VALID_EXTENTIONS.iter().position(|&x| path.has_extention(x)) {
        return Err(Error::new(ErrorKind::Other, "Invalid extention"));
    }

    let fd = File::open(path)?;
    let file_size = fd.metadata()?.len();
    let mut reader = BufReader::new(fd);
    let mut writer = BufWriter::new(File::create(path.with_extention(EXTENTION))?);

    let mut buffer = [0, 1].repeat(BUFF_SIZE);

    Ok(())
}
