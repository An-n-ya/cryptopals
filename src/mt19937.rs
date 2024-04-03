#![allow(unused)]
const N: usize = 624;
pub struct MT19937 {
    mt: [u32; N],
    mti: usize,
}

impl MT19937 {
    pub fn new(seed: u32) -> Self {
        let mut mt = [0u32; N];
        mt[0] = seed;
        for i in 1..N {
            mt[i] = 1812433253u32.wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> 30)) + i as u32;
        }
        Self { mt, mti: 0 }
    }

    pub fn new_with_mt(mt: Vec<u32>) -> Self {
        Self {
            mt: mt.try_into().unwrap(),
            mti: 0,
        }
    }

    pub fn gen_u32(&mut self) -> u32 {
        if self.mti == 0 {
            self.twist();
        }

        let mut y = self.mt[self.mti];
        y ^= y >> 11;
        y ^= (y << 7) & 2636928640;
        y ^= (y << 15) & 4022730752;
        y ^= y >> 18;
        self.mti = (self.mti + 1) % 624;
        y
    }

    fn twist(&mut self) {
        for i in 0..N {
            let y = (self.mt[i] & 0x8000_0000) + (self.mt[(i + 1) % N] & 0x7fff_ffff);
            self.mt[i] = (y >> 1) ^ self.mt[(i + 397) % N];
            if y % 2 != 0 {
                self.mt[i] = self.mt[i] ^ 0x9908_b0df;
            }
        }
    }
}

pub fn untemper(val: u32) -> u32 {
    let mut val = val;
    val = undo_shift_right(val, 18);
    val = undo_shift_left(val, 15, 4022730752);
    val = undo_shift_left(val, 7, 2636928640);
    val = undo_shift_right(val, 11);
    val
}

fn undo_shift_left(n: u32, shift: u8, and: u32) -> u32 {
    let mut res = n.get_lower_bit(shift);
    for i in (shift..32) {
        res = res.set_bit(
            i,
            n.is_bit_set(i) ^ res.is_bit_set(i - shift) & and.is_bit_set(i),
        );
    }
    res
}
fn undo_shift_right(n: u32, shift: u8) -> u32 {
    if shift >= 16 {
        return n ^ (n >> shift);
    }
    let mut res = n.get_upper_bit(shift);
    for i in (0..32 - shift) {
        res = res.set_bit(
            31 - shift - i,
            n.is_bit_set(31 - shift - i) ^ res.is_bit_set(31 - i),
        );
    }
    res
}

trait Helper {
    fn get_upper_bit(&self, n: u8) -> u32;
    fn get_lower_bit(&self, n: u8) -> u32;
    fn is_bit_set(&self, n: u8) -> bool;
    fn set_bit(&self, n: u8, val: bool) -> u32;
}

impl Helper for u32 {
    fn get_lower_bit(&self, n: u8) -> u32 {
        let mut res = 0;
        for i in 0..n {
            res |= (1 << i) & self
        }
        res
    }
    fn get_upper_bit(&self, n: u8) -> u32 {
        let mut res = 0;
        for i in (32 - n..32).rev() {
            res |= (1 << i) & self
        }
        res
    }
    fn is_bit_set(&self, n: u8) -> bool {
        (self & (1 << n)) != 0
    }
    fn set_bit(&self, n: u8, val: bool) -> u32 {
        if val {
            self | (1 << n)
        } else {
            self & !(1 << n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_helper() {
        let n = 0x7fff_ffffu32;
        assert_eq!(n.get_upper_bit(16), 0x7fff_0000);
        assert!(n.is_bit_set(30));
        assert!(!n.is_bit_set(31));
        let n = n.set_bit(31, true);
        assert_eq!(n.get_upper_bit(16), 0xffff_0000);
        let n = n.set_bit(31, false);
        let n = n.set_bit(30, false);
        assert_eq!(n.get_upper_bit(16), 0x3fff_0000);
    }

    #[test]
    fn test_undo_shift_right() {
        let origin = 0x7fff_ffffu32;
        let n = origin ^ (origin >> 16);
        let res = undo_shift_right(n, 16);
        assert_eq!(res, origin);
        let n = origin ^ (origin >> 8);
        let res = undo_shift_right(n, 8);
        // println!("shift:{:#08x} n:{:#08x} res:{:#08x}", origin >> 8, n, res);
        assert_eq!(res, origin);
    }

    #[test]
    fn test_undo_shift_left() {
        let origin = 0x7fff_ffffu32;
        let n = origin ^ (origin << 16);
        let res = undo_shift_left(n, 16, 0xffff_ffff);
        assert_eq!(res, origin);
        let n = origin ^ (origin << 8);
        let res = undo_shift_left(n, 8, 0xffff_ffff);
        assert_eq!(res, origin);
    }

    #[test]
    fn test_untemper() {
        let origin = 0x7fff_ffffu32;
        let mut y = origin;
        y ^= y >> 11;
        println!("1 {y:#x}");
        y ^= (y << 7) & 2636928640;
        println!("2 {y:#x}");
        y ^= (y << 15) & 4022730752;
        println!("3 {y:#x}");
        y ^= y >> 18;
        println!("4 {y:#x}");

        let res = untemper(y);
        assert_eq!(res, origin);
    }
}
