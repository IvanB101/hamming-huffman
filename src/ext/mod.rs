mod raw;

use std::ffi::{c_char, CStr, CString};

#[derive(Debug, Clone)]
pub struct FfiError {
    pub message: String,
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FfiError {}

pub fn encode(path: String, block_size: u64) -> Result<(), FfiError> {
    let valid_sizes: [u64; 3] = [32, 2048, 65536];
    let exponents: [u64; 3] = [5, 11, 16];
    let extentions: [&str; 3] = ["HA1", "HA2", "HA3"];

    if !path.has_extention("txt") {
        return Err(FfiError{message: String::from("Invalid extention")});
    }

    let ext;
    let exponent;

    println!("Block size: {}", block_size);
    match valid_sizes.iter().position(|&x| x == block_size) {
        Some(index) => {
            ext = extentions[index];
            exponent = exponents[index];
        },
        None => return Err(FfiError{message: String::from("Invalid block size")}),
    }

    let err: *const c_char;
    let c_path = CString::new(path.clone()).unwrap();
    let c_dest = CString::new(path.with_extention(ext)).unwrap();

    unsafe {
        err = raw::encode(c_path.as_ptr(), c_dest.as_ptr(), block_size, exponent);
    }

    if err.is_null() {
        Ok(())
    } else {
        unsafe {
            Err(FfiError {
                message: CStr::from_ptr(err).to_str().unwrap().to_owned(),
            })
        }
    }
}

impl Convienience for &str {
    fn has_extention(&self, ext: &str) -> bool {
        match self.find('.') {
            Some(n) => self.split_at(n + 1).1 == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let name = match self.find('.') {
            Some(n) => self.split_at(n).0,
            None => self,
        };

        name.to_owned() + "." + ext
    }
}

impl Convienience for String {
    fn has_extention(&self, ext: &str) -> bool {
        match self.find('.') {
            Some(n) => self.split_at(n + 1).1 == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let name = match self.find('.') {
            Some(n) => self.split_at(n).0,
            None => self,
        };

        name.to_owned() + "." + ext
    }
}

pub trait Convienience {
    fn has_extention(&self, ext: &str) -> bool;

    fn with_extention(&self, new_ext: &str) -> String;
}
