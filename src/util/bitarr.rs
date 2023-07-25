use std::cmp::min;

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

macro_rules! bounded_impl {
    ($type:ty, $bit_size:expr, $max_mask: expr) => {
        impl BitArr for [$type] {
            bit_arr_impl!($type, $bit_size, $max_mask);
        }

        impl<'a> Iterator for BitIterator<'a, [$type]> {
            iter_impl!($type, $bit_size);
        }

        impl<'a> Iterator for BitIteratorLen<'a, [$type]> {
            iter_len_impl!($type, $bit_size);
        }

        impl BitArr for Vec<$type> {
            bit_arr_impl!($type, $bit_size, $max_mask);
        }

        impl<'a> Iterator for BitIterator<'a, Vec<$type>> {
            iter_impl!($type, $bit_size);
        }

        impl<'a> Iterator for BitIteratorLen<'a, Vec<$type>> {
            iter_len_impl!($type, $bit_size);
        }
    };
}

macro_rules! bit_arr_impl {
    ($type:ty, $bit_size:expr, $max_mask: expr) => {
        fn len(&self) -> usize {
            self.len()
        }

        fn set_bit(&mut self, position: usize) {
            let mask: $type = 1 << ($bit_size - 1 - position % $bit_size);

            self[position / 8] |= mask;
        }

        fn reset_bit(&mut self, position: usize) {
            let mask: $type = 1 << ($bit_size - 1 - position % $bit_size);

            self[position / 8] &= mask;
        }

        fn flip_bit(&mut self, position: usize) {
            let mask: $type = 1 << ($bit_size - 1 - position % $bit_size);

            self[position / 8] ^= mask;
        }

        fn check_bit(&self, position: usize) -> bool {
            let mask: $type = 1 << ($bit_size - 1 - position % $bit_size);

            self[position / 8] & mask != 0
        }

        fn put_bits(&mut self, from: &Self, start_to: usize, start_from: usize, size: usize) {
            let mut passed = 0;
            let mut current_from = start_from;
            let mut current_to = start_to;

            while passed < size {
                let dist_from = $bit_size - (current_from % $bit_size);
                let dist_to = $bit_size - (current_to % $bit_size);
                let to_move = min(min(dist_from, dist_to), size - passed);

                let mut mask = ((1 << to_move) - 1) as $type;
                if mask == 0 {
                    mask = $max_mask;
                }

                let mut temp =
                    from[current_from / $bit_size] & (mask << (dist_from as i32 - to_move as i32));

                mask <<= dist_to as i32 - to_move as i32;

                let dif = dist_from as i32 - dist_to as i32;
                if dif < 0 {
                    temp <<= -dif;
                } else {
                    temp >>= dif;
                }
                self[current_to / 8] &= !mask;
                self[current_to / 8] |= temp;

                passed += to_move;
                current_from += to_move;
                current_to += to_move;
            }
        }

        fn parity(&self) -> bool {
            let mut res: $type = self[0];

            for i in 1..self.len() {
                res ^= self[i];
            }

            return res.count_ones() % 2 == 1;
        }

        fn masked_parity(&self, mask: &[u8]) -> bool {
            let mut res: $type = self[0] & mask[0];

            for i in 1..self.len() {
                res ^= self[i] & mask[i];
            }

            return res.count_ones() % 2 == 1;
        }

        fn to_binary(&self) -> String {
            let len = self.len() * $bit_size;
            let mut res = String::with_capacity(len);

            for i in 0..len {
                res.push(
                    if self[i / $bit_size] & 1 << ($bit_size - 1 - (i % $bit_size)) != 0 {
                        '1'
                    } else {
                        '0'
                    },
                )
            }

            res
        }

        fn to_binary_len(&self, len: usize) -> String {
            let mut res = String::with_capacity(len);

            for i in 0..(len) {
                res.push(
                    if self[i / $bit_size] & 1 << ($bit_size - 1 - (i % $bit_size)) != 0 {
                        '1'
                    } else {
                        '0'
                    },
                )
            }

            res
        }

        fn iter_bits(&self) -> BitIterator<Self> {
            BitIterator {
                arr: self,
                index: 0,
                mask: 1 << $bit_size - 1,
            }
        }

        fn iter_bits_len(&self, len: usize) -> BitIteratorLen<Self> {
            BitIteratorLen {
                arr: self,
                index: 0,
                mask: 1 << $bit_size - 1,
                len,
                current: 0,
            }
        }
    };
}

macro_rules! iter_impl {
    ($type:ty, $bit_size:expr) => {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == self.arr.len() {
                return None;
            }

            let bit = self.arr[self.index] & self.mask != 0;

            self.mask >>= 1;
            if self.mask == 0 {
                self.index += 1;
                self.mask = 1 << $bit_size - 1;
            }

            Some(bit)
        }
    };
}

macro_rules! iter_len_impl {
    ($type:ty, $bit_size:expr) => {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current == self.len {
                return None;
            }

            let bit = self.arr[self.index] & self.mask != 0;

            self.mask >>= 1;
            self.current += 1;
            if self.mask == 0 {
                self.index += 1;
                self.mask = 1 << $bit_size - 1;
            }

            Some(bit)
        }
    };
}

bounded_impl!(u8, 8, 127);
