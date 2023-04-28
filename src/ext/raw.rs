use std::os::raw::c_char;
use std::os::raw::c_ulong;

#[link(name = "c_procedures")]
extern {
    pub fn encode(path: *const c_char, block_size: c_ulong) -> *const c_char;

    pub fn decode(path: *const c_char) -> *const c_char;

    pub fn corrupt(path: *const c_char) -> *const c_char;

    pub fn compress(path: *const c_char) -> *const c_char;

    pub fn decompress(path: *const c_char) -> *const c_char;
}
