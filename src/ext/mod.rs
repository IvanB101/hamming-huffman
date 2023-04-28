mod raw;

use std::ffi::{c_char, CStr, CString};
use std::fmt;
use std::string::ParseError;

#[derive(Debug, Clone)]
pub struct FfiError {
    pub message: String,
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FfiError {}

pub fn encode(path: String, block_size: u64) -> Result<(), FfiError> {
    let err: *const c_char;

    let c_path = CString::new(path).unwrap();

    unsafe {
        err = raw::encode(c_path.as_ptr(), block_size);
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

impl Convienience for String {
    fn has_extention(path: &str, ext: &str) -> bool {
        match path.find('.') {
            Some(n) => path.split_at(n + 1).1 == ext,
            None => ext.is_empty(),
        }
    }

    fn change_extention(path: &str, new_ext: &str) -> String {
        match path.find('.') {
            Some(n) => {
                let name = path.split_at(n).0;

                name.to_owned() + new_ext
            }
            None => path.to_owned() + new_ext,
        }
    }
}

trait Convienience {
    fn has_extention(path: &str, ext: &str) -> bool;

    fn change_extention(path: &str, new_ext: &str) -> String;
}
