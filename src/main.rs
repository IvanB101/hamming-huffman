use util::string::Extention;

mod hamming;
mod huffman;
mod util;

fn main() {
    let path = "./test/test.txt";

    huffman::compress::compress(path).unwrap();

    huffman::decompress::decompress(&path.with_extention(huffman::compress::EXTENTION)).unwrap();
}
