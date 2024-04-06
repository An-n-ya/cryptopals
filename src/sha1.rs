#![allow(unused)]
use itertools::Itertools;

const H0: u32 = 0x67452301;
const H1: u32 = 0xEFCDAB89;
const H2: u32 = 0x98BADCFE;
const H3: u32 = 0x10325476;
const H4: u32 = 0xC3D2E1F0;

pub fn sha1(message: &[u8]) -> [u8; 20] {
    let mut message = message.to_vec();
    let ml = message.len() * 8;
    if ml % 8 == 0 {
        message.push(0x80);
    }
    while (message.len() * 8) % 512 != 448 {
        message.push(0x00);
    }
    for byte in ml.to_be_bytes() {
        message.push(byte);
    }
    assert!((message.len() * 8) % 512 == 0);

    let mut h = [H0, H1, H2, H3, H4];
    for chunk in message.chunks(64) {
        let mut w = chunk
            .chunks(4)
            .map(|c| u32::from_be_bytes(c.try_into().unwrap()))
            .collect_vec();
        for i in 16..80 {
            w.push((w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1));
        }
        let h_origin = h.clone();
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((h[1] & h[2]) | (!h[1] & h[3]), 0x5A827999),
                20..=39 => (h[1] ^ h[2] ^ h[3], 0x6ED9EBA1),
                40..=59 => ((h[1] & h[2]) | (h[1] & h[3]) | (h[2] & h[3]), 0x8F1BBCDC),
                60..=79 => (h[1] ^ h[2] ^ h[3], 0xCA62C1D6),
                _ => unreachable!(),
            };

            h[4] = (h[0].rotate_left(5))
                .wrapping_add(f)
                .wrapping_add(h[4])
                .wrapping_add(k)
                .wrapping_add(w[i]);
            h[1] = h[1].rotate_left(30);
            h.rotate_right(1);
        }
        for (i, v) in h_origin.iter().enumerate() {
            h[i] = h[i].wrapping_add(*v);
        }
    }
    [
        h[0].to_be_bytes(),
        h[1].to_be_bytes(),
        h[2].to_be_bytes(),
        h[3].to_be_bytes(),
        h[4].to_be_bytes(),
    ]
    .concat()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::base64::u8_to_base64;

    use super::*;

    #[test]
    fn test_sha1() {
        let data = b"";
        let res = sha1(data);
        let res = u8_to_base64(&res);
        assert_eq!(res, "2jmj7l5rSw0yVb/vlWAYkK/YBw");

        let res = sha1(b"The quick brown fox jumps over the lazy dog");
        let s: String = res.iter().map(|n| format!("{:02x}", n)).collect();
        assert_eq!(s, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");

        let res = sha1(b"The quick brown fox jumps over the lazy cog");
        let s: String = res.iter().map(|n| format!("{:02x}", n)).collect();
        assert_eq!(s, "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3");
    }
}
