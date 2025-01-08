mod attack1;
mod attack1_consts;
use core::num::Wrapping as W;
type Wu32 = W<u32>;

pub const S0: [Wu32; 4] = [
    W(0x6745_2301),
    W(0xEFCD_AB89),
    W(0x98BA_DCFE),
    W(0x1032_5476),
];
const PADDING: [u8; 64] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];
pub const K1: Wu32 = W(0x5A82_7999);
pub const K2: Wu32 = W(0x6ED9_EBA1);

pub fn f(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    // TODO: use x86's selection opcode
    // (x & y) | (!x & z)
    z ^ (x & (y ^ z))
}
pub fn g(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    (x & y) | (x & z) | (y & z)
}
pub fn h(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    x ^ y ^ z
}
pub fn op<F>(f: F, a: Wu32, b: Wu32, c: Wu32, d: Wu32, k: Wu32, s: u32) -> Wu32
where
    F: Fn(Wu32, Wu32, Wu32) -> Wu32,
{
    let t = a + f(b, c, d) + k;
    W(t.0.rotate_left(s))
}
pub fn inv_op<F>(f: F, a: Wu32, b: Wu32, c: Wu32, d: Wu32, k: Wu32, s: u32, t: Wu32) -> Wu32
where
    F: Fn(Wu32, Wu32, Wu32) -> Wu32,
{
    W(t.0.rotate_right(s)) - a - f(b, c, d) - k
}
pub fn md4(msg: &[u8]) -> String {
    let mut blocks = Vec::new();
    fn print_blocks(blocks: &Vec<Vec<Wu32>>) {
        for block in blocks {
            assert!(block.len() == 16);
            for n in block {
                let bytes = n.0.to_le_bytes();
                println!(
                    "{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
                    bytes[0], bytes[1], bytes[2], bytes[3]
                );
            }
        }
    }
    fn add_u8_arr(arr: &[u8], blocks: &mut Vec<Vec<Wu32>>) -> usize {
        let mut block = vec![W(0); 16];
        if arr.len() < 64 {
            return 0;
        }
        let mut cur = 0;
        for i in (0..arr.len()).step_by(4) {
            if i + 4 > arr.len() {
                break;
            }
            block[cur] = W(u32::from_le_bytes([
                arr[i + 0],
                arr[i + 1],
                arr[i + 2],
                arr[i + 3],
            ]));
            cur += 1;
            if cur == 16 {
                blocks.push(block.clone());
            }
        }
        cur
    }
    let cnt = add_u8_arr(msg, &mut blocks);
    let msg_len = msg.len();
    let remaining = msg_len - cnt * 4;
    // padding to 448
    let mut padding_block = msg[msg_len - remaining..msg_len]
        .iter()
        .map(|v| *v)
        .collect::<Vec<_>>();
    let padding_length = if padding_block.len() == 56 {
        64
    } else {
        56 - padding_block.len()
    };
    padding_block = padding_block
        .iter()
        .chain(PADDING[0..padding_length].iter())
        .map(|v| *v)
        .collect();
    // adding length
    let length = ((msg.len() * 8) as u64).to_le_bytes();
    padding_block = padding_block
        .iter()
        .chain(length.iter())
        .map(|v| *v)
        .collect();
    add_u8_arr(&padding_block, &mut blocks);
    assert!(padding_block.len() % 64 == 0);
    // print_blocks(&blocks);

    let mut states = S0;
    for msg in blocks {
        let [mut a, mut b, mut c, mut d] = states;
        for &i in &[0, 4, 8, 12] {
            a = op(f, a, b, c, d, msg[i], 3);
            d = op(f, d, a, b, c, msg[i + 1], 7);
            c = op(f, c, d, a, b, msg[i + 2], 11);
            b = op(f, b, c, d, a, msg[i + 3], 19);
        }
        for &i in &[0, 1, 2, 3] {
            a = op(g, a, b, c, d, msg[i] + K1, 3);
            d = op(g, d, a, b, c, msg[i + 4] + K1, 5);
            c = op(g, c, d, a, b, msg[i + 8] + K1, 9);
            b = op(g, b, c, d, a, msg[i + 12] + K1, 13);
        }
        for &i in &[0, 2, 1, 3] {
            a = op(h, a, b, c, d, msg[i] + K2, 3);
            d = op(h, d, a, b, c, msg[i + 8] + K2, 9);
            c = op(h, c, d, a, b, msg[i + 4] + K2, 11);
            b = op(h, b, c, d, a, msg[i + 12] + K2, 15);
        }
        states[0] += a;
        states[1] += b;
        states[2] += c;
        states[3] += d;
    }
    states
        .iter()
        .map(|v| v.0.to_le_bytes())
        .collect::<Vec<_>>()
        .concat()
        .iter()
        .fold(String::new(), |acc, v| format!("{acc}{v:x}"))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_md4_impl() {
        let messages = [
            "".as_bytes(),
            "The quick brown fox jumps over the lazy dog".as_bytes(),
            "BEES".as_bytes(),
            &[0xaa],
            &[0xaa, 0xbb],
            &[
                0x24, 0x40, 0x45, 0x10, 0x7e, 0x48, 0x3, 0xcb, 0x7e, 0xe5, 0x5d, 0xfa, 0x67, 0xa1,
                0x27, 0x29, 0x16, 0x8a, 0x6c, 0x52, 0x5e, 0x5a, 0xe, 0x5d, 0x9e, 0x44, 0x54, 0xc5,
                0x73, 0x62, 0x92, 0xf7, 0x65, 0x70, 0xb2, 0x47, 0x6a, 0x32, 0x34, 0xdc, 0xa2, 0x98,
                0xd, 0x3a, 0x10, 0x99, 0x81, 0xb, 0x82, 0x50, 0x72, 0x9, 0xe6, 0x56, 0x51, 0x97,
                0x24, 0x7e, 0xbd, 0x3, 0x4c, 0x24, 0xd5, 0x22,
            ],
        ];
        let known_hashes = [
            "31d6cfe0d16ae931b73c59d7e0c089c0",
            "1bee69a46ba811185c194762abaeae90",
            "501af1ef4b68495b5b7e37b15b4cda68",
            "f322852f43b3dd6c68b01de97bc547fd",
            "176078c7efaebfdacd1f4112467874e1",
            "a25f98cb8736de9f7c9641995982a44f",
        ];
        for (msg, expect) in messages.iter().zip(known_hashes.iter()) {
            let got = md4(msg);
            assert_eq!(&got, expect);
        }
    }
}
