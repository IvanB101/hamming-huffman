mod raw;

use std::fmt;
use std::ffi::{CString, CStr, c_char};

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
            Err(FfiError{message: CStr::from_ptr(err).to_str().unwrap().to_owned()})
        }
    }
}
