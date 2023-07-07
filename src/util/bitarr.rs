use std::cmp::min;

pub trait BitArr {
    fn set_bit(&mut self, position: usize);

    fn reset_bit(&mut self, position: usize);

    fn flip_bit(&mut self, position: usize);

    fn put_bits(&mut self, from: &Self, start_to: usize, start_from: usize, size: usize);

    fn parity(&self) -> bool;

    fn masked_parity(&self, mask: &[u8]) -> bool;

    fn to_binary(&self) -> String;

    fn to_binary_len(&self, len: usize) -> String;
}

impl BitArr for [u8] {
    fn set_bit(&mut self, position: usize) {
        let mask: u8 = 1 << (7 - position % 8);

        self[position / 8] |= mask;
    }

    fn reset_bit(&mut self, position: usize) {
        let mask: u8 = 1 << (7 - position % 8);

        self[position / 8] &= mask;
    }

    fn flip_bit(&mut self, position: usize) {
        let mask: u8 = 1 << (7 - position % 8);

        self[position / 8] ^= mask;
    }

    fn put_bits(&mut self, from: &Self, start_to: usize, start_from: usize, size: usize) {
        let mut passed = 0;
        let mut current_from = start_from;
        let mut current_to = start_to;

        while passed < size {
            let dist_from = 8 - (current_from % 8);
            let dist_to = 8 - (current_to % 8);
            let to_move = min(min(dist_from, dist_to), size - passed);

            let mut mask = (((1 as i16) << to_move) - 1) as u8;

            let mut temp = from[current_from / 8] & (mask << (dist_from as i32 - to_move as i32));
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
        let mut res: u8 = self[0];

        for i in 1..self.len() {
            res ^= self[i];
        }

        return res.count_ones() % 2 == 1;
    }

    fn masked_parity(&self, mask: &[u8]) -> bool {
        let mut res: u8 = self[0] & mask[0];

        for i in 1..self.len() {
            res ^= self[i] & mask[i];
        }

        return res.count_ones() % 2 == 1;
    }

    fn to_binary(&self) -> String {
        let mut res = String::with_capacity(self.len());

        for i in 0..(self.len() * 8) {
            res.push(if self[i / 8] & 1 << (7 - (i % 8)) != 0 {
                '1'
            } else {
                '0'
            })
        }

        res
    }

    fn to_binary_len(&self, len: usize) -> String {
        let mut res = String::with_capacity(len);

        for i in 0..(len) {
            res.push(if self[i / 8] & 1 << (7 - (i % 8)) != 0 {
                '1'
            } else {
                '0'
            })
        }

        res
    }
}
