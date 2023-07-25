use crate::util::bitarr::BitArr;

mod hamming;
mod huffman;
mod util;

fn main() {
    let mut mask = 1 as u8;

    mask <<= 4;

    println!("{}", [mask].to_binary());

    /*
    let path = "./test/test.txt";

    huffman::encoder::compress(path).unwrap();

    huffman::decoder::decompress(&path.with_extention(huffman::encoder::EXTENTION)).unwrap();
    */
}
