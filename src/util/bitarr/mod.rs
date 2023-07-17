pub mod slice_u8;
pub mod vec_u8;

pub struct BitIterator<'a, T: BitArr + ?Sized> {
    arr: &'a T,
    index: usize,
    mask: u8,
}

pub struct BitIteratorLen<'a, T: BitArr + ?Sized> {
    arr: &'a T,
    index: usize,
    mask: u8,
    len: usize,
    current: usize,
}

pub trait BitArr {
    fn len(&self) -> usize;

    fn set_bit(&mut self, position: usize);

    fn reset_bit(&mut self, position: usize);

    fn flip_bit(&mut self, position: usize);

    fn check_bit(&self, position: usize) -> bool;

    fn put_bits(&mut self, from: &Self, start_to: usize, start_from: usize, size: usize);

    fn parity(&self) -> bool;

    fn masked_parity(&self, mask: &[u8]) -> bool;

    fn to_binary(&self) -> String;

    fn to_binary_len(&self, len: usize) -> String;

    fn iter_bits(&self) -> BitIterator<Self>;

    fn iter_bits_len(&self, len: usize) -> BitIteratorLen<Self>;
}
