use core::time;
use std::{thread::sleep, time::SystemTime};

use crate::mt19937::MT19937;

fn timestamp_seed() -> (u32, u32) {
    sleep(time::Duration::from_secs(rand::random::<u64>() % 10));
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    sleep(time::Duration::from_secs(rand::random::<u64>() % 10));
    let mut rng = MT19937::new(seed);
    (seed, rng.gen_u32())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crack_timestamp_seed() {
        let (true_seed, cipher_text) = timestamp_seed();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        let mut seed = now;
        loop {
            let val = MT19937::new(seed).gen_u32();
            if cipher_text == val {
                assert_eq!(seed, true_seed);
                return;
            }
            seed -= 1;
        }
    }
}
