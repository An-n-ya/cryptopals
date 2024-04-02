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
