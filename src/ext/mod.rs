mod raw;

use std::ffi::{c_char, c_int, CStr, CString};

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
    let valid_extentions: [&str; 1] = ["txt"];

    if let None = valid_extentions.iter().position(|&x| path.has_extention(x)) {
        return Err(FfiError {
            message: String::from("Extension invalida"),
        });
    }

    let ext;
    let exponent;

    if let Some(index) = valid_sizes.iter().position(|&x| x == block_size) {
        ext = extentions[index];
        exponent = exponents[index]
    } else {
        return Err(FfiError {
            message: String::from("Tamaño invalido"),
        });
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

pub fn decode(path: String, correct: bool) -> Result<(), FfiError> {
    let sizes: [u64; 3] = [32, 2048, 65536];
    let exponents: [u64; 3] = [5, 11, 16];
    let valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"];
    let extentions = [["DE1", "DE2", "DE3"], ["DC1", "DC2", "DC3"]];

    let exponent;
    let block_size;
    let ext;

    if let Some(mut index) = valid_extentions.iter().position(|&x| path.has_extention(x)) {
        index %= 3;
        exponent = exponents[index];
        block_size = sizes[index];
        ext = extentions[correct as usize][index];
    } else {
        return Err(FfiError {
            message: String::from("Extension invalida"),
        });
    }

    let err: *const c_char;
    let c_path = CString::new(path.clone()).unwrap();
    let c_dest = CString::new(path.with_extention(ext)).unwrap();
    let c_correct = if correct { 1 as c_int } else { 0 as c_int };

    unsafe {
        err = raw::decode(
            c_path.as_ptr(),
            c_dest.as_ptr(),
            block_size,
            exponent,
            c_correct,
        );
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

pub fn corrupt(path: String, probability: f64) -> Result<(), FfiError> {
    let sizes: [u64; 3] = [32, 2048, 65536];
    let exponents: [u64; 3] = [5, 11, 16];
    let valid_extentions = ["HA1", "HA2", "HA3"];
    let extentions = ["HE1", "HE2", "HE3"];

    let exponent;
    let block_size;
    let ext;

    if probability < 0.0 || probability > 1.0 {
        return Err(FfiError {
            message: String::from("Probabilidad de error invalida"),
        });
    }

    if let Some(index) = valid_extentions.iter().position(|&x| path.has_extention(x)) {
        exponent = exponents[index];
        block_size = sizes[index];
        ext = extentions[index];
    } else {
        return Err(FfiError {
            message: String::from("Extension invalida"),
        });
    }

    let err: *const c_char;
    let c_path = CString::new(path.clone()).unwrap();
    let c_dest = CString::new(path.with_extention(ext)).unwrap();

    unsafe {
        err = raw::corrupt(
            c_path.as_ptr(),
            c_dest.as_ptr(),
            block_size,
            exponent,
            probability,
        );
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

pub fn compress(path: String) -> Result<(), FfiError> {
    let valid_extentions = ["txt"];
    let ext = "huf";

    if let None = valid_extentions.iter().position(|&x| path.has_extention(x)) {
        return Err(FfiError {
            message: String::from("Extension invalida"),
        });
    }

    let err: *const c_char;
    let c_path = CString::new(path.clone()).unwrap();
    let c_dest = CString::new(path.with_extention(ext)).unwrap();

    unsafe {
        err = raw::compress(c_path.as_ptr(), c_dest.as_ptr());
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

pub fn decompress(path: String) -> Result<(), FfiError> {
    let ext = "dhu";

    if !path.has_extention("huf") {
        return Err(FfiError {
            message: String::from("Invalid extention"),
        });
    }

    let err: *const c_char;
    let c_path = CString::new(path.clone()).unwrap();
    let c_dest = CString::new(path.with_extention(ext)).unwrap();

    unsafe {
        err = raw::decompress(c_path.as_ptr(), c_dest.as_ptr());
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

impl Extention for &str {
    fn has_extention(&self, ext: &str) -> bool {
        match self.split('.').last() {
            Some(e) => e == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let parts = self.split('/');
        let mut path = "".to_owned();

        if parts.clone().count() > 1 {
            for part in parts.clone().take(parts.clone().count() - 1) {
                path += part;
                path += "/";
            }
        }

        let file = parts.last().unwrap_or(self);

        let name = match file.chars().rev().position(|x| x == '.') {
            Some(index) => file.split_at(file.len() - 1 - index).0,
            None => file,
        };

        path + name + "." + ext
    }
}

impl Extention for String {
    fn has_extention(&self, ext: &str) -> bool {
        match self.split('.').last() {
            Some(e) => e == ext,
            None => ext.is_empty(),
        }
    }

    fn with_extention(&self, ext: &str) -> String {
        let parts = self.split('/');
        let mut path = "".to_owned();

        if parts.clone().count() > 1 {
            for part in parts.clone().take(parts.clone().count() - 1) {
                path += part;
                path += "/";
            }
        }

        let file = parts.last().unwrap_or(self);

        let name = match file.chars().rev().position(|x| x == '.') {
            Some(index) => file.split_at(file.len() - 1 - index).0,
            None => file,
        };

        path + name + "." + ext
    }
}

pub trait Extention {
    fn has_extention(&self, ext: &str) -> bool;

    fn with_extention(&self, new_ext: &str) -> String;
}
