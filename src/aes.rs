#![allow(unused, non_upper_case_globals)]

use core::fmt;

use itertools::Itertools;

const Nb: usize = 4; // block size
const Nr: usize = 10; // number of rounds
const Nk: usize = 4; // key length

pub fn cipher(input: &[u8], word: &Vec<Vec<u8>>) -> Vec<u8> {
    assert!(input.len() == 4 * Nb && word.len() == Nb * (Nr + 1));
    let mut state = vec![];
    for i in 0..4 {
        let mut tmp = vec![];
        for j in 0..Nb {
            tmp.push(input[j * 4 + i]);
        }
        state.push(tmp);
    }

    #[cfg(test_aes)]
    println!("INVERSE CIPHER (DECRYPT):");
    #[cfg(test_aes)]
    println!("round[00].input\t{}", state_to_string(&state));

    add_round_key(&mut state, &key_at(word, 0));
    #[cfg(test_aes)]
    println!("round[00].k_sch\t{}", state_to_string(&key_at(word, 0)));
    for round in 1..Nr {
        #[cfg(test_aes)]
        println!("round[{:02}].start\t{}", round, state_to_string(&state));

        sub_bytes(&mut state);
        #[cfg(test_aes)]
        println!("round[{:02}].s_box\t{}", round, state_to_string(&state));

        shift_rows(&mut state);
        #[cfg(test_aes)]
        println!("round[{:02}].s_row\t{}", round, state_to_string(&state));

        mix_columns(&mut state);
        #[cfg(test_aes)]
        println!("round[{:02}].m_col\t{}", round, state_to_string(&state));

        #[cfg(test_aes)]
        println!(
            "round[{:02}].k_sch\t{}",
            round,
            state_to_string(&key_at(word, round))
        );
        add_round_key(&mut state, &key_at(word, round));
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &key_at(word, Nr));

    rotate(&state).concat()
}

pub fn decipher(input: &[u8], word: &Vec<Vec<u8>>) -> Vec<u8> {
    assert!(input.len() == 4 * Nb && word.len() == Nb * (Nr + 1));
    let mut state = vec![];
    for i in 0..4 {
        let mut tmp = vec![];
        for j in 0..Nb {
            tmp.push(input[j * 4 + i]);
        }
        state.push(tmp);
    }

    #[cfg(test_aes)]
    println!("CIPHER (ENCRYPT):");
    #[cfg(test_aes)]
    println!("round[00].iinput\t{}", state_to_string(&state));

    add_round_key(&mut state, &key_at(word, Nr));
    #[cfg(test_aes)]
    println!("round[00].ik_sch\t{}", state_to_string(&key_at(word, Nr)));

    for round in (1..Nr).rev() {
        #[cfg(test_aes)]
        println!(
            "round[{:02}].istart\t{}",
            Nr - round,
            state_to_string(&state)
        );

        inv_shift_rows(&mut state);
        #[cfg(test_aes)]
        println!(
            "round[{:02}].is_row\t{}",
            Nr - round,
            state_to_string(&state)
        );

        inv_sub_bytes(&mut state);
        #[cfg(test_aes)]
        println!(
            "round[{:02}].is_box\t{}",
            Nr - round,
            state_to_string(&state)
        );

        #[cfg(test_aes)]
        println!(
            "round[{:02}].ik_sch\t{}",
            Nr - round,
            state_to_string(&key_at(word, round))
        );

        add_round_key(&mut state, &key_at(word, round));
        #[cfg(test_aes)]
        println!(
            "round[{:02}].ik_add\t{}",
            Nr - round,
            state_to_string(&state)
        );

        inv_mix_columns(&mut state);
    }

    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &key_at(word, 0));

    rotate(&state).concat()
}

fn state_to_string(state: &Vec<Vec<u8>>) -> String {
    let mut res = "".to_string();

    for j in 0..state[0].len() {
        for i in 0..state.len() {
            res += &format!("{:02x}", state[i][j]);
        }
    }

    res
}

fn key_at(mat: &Vec<Vec<u8>>, index: usize) -> Vec<Vec<u8>> {
    rotate(&mat[index * Nb..(index + 1) * Nb])
}

fn rotate(mat: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for j in 0..mat[0].len() {
        let mut tmp = vec![];
        for i in 0..mat.len() {
            tmp.push(mat[i][j]);
        }
        res.push(tmp);
    }
    res
}

fn add_round_key(state: &mut Vec<Vec<u8>>, w: &[Vec<u8>]) {
    assert!(w.len() == Nb);

    for i in 0..4 {
        for j in 0..Nb {
            state[i][j] ^= w[i][j]
        }
    }
}

fn inv_sub_bytes(state: &mut Vec<Vec<u8>>) {
    for i in 0..4 {
        for j in 0..Nb {
            let val = state[i][j];
            let row = val >> 4;
            let col = val & 0b1111;
            state[i][j] = INV_S_BOX[row as usize][col as usize];
        }
    }
}
fn sub_bytes(state: &mut Vec<Vec<u8>>) {
    for i in 0..4 {
        for j in 0..Nb {
            let val = state[i][j];
            let row = val >> 4;
            let col = val & 0b1111;
            state[i][j] = S_BOX[row as usize][col as usize];
        }
    }
}

fn inv_shift_rows(state: &mut Vec<Vec<u8>>) {
    for i in 0..Nb {
        state[i].rotate_right(i);
    }
}
fn shift_rows(state: &mut Vec<Vec<u8>>) {
    for i in 0..Nb {
        state[i].rotate_left(i);
    }
}

fn inv_mix_columns(state: &mut Vec<Vec<u8>>) {
    for j in 0..Nb {
        let mut tmp = vec![];
        for i in 0..4 {
            tmp.push(state[i][j]);
        }
        let mut s = vec![0; 4];
        s[0] = mul(0x0e, tmp[0]) ^ mul(0x0b, tmp[1]) ^ mul(0x0d, tmp[2]) ^ mul(0x09, tmp[3]);
        s[1] = mul(0x09, tmp[0]) ^ mul(0x0e, tmp[1]) ^ mul(0x0b, tmp[2]) ^ mul(0x0d, tmp[3]);
        s[2] = mul(0x0d, tmp[0]) ^ mul(0x09, tmp[1]) ^ mul(0x0e, tmp[2]) ^ mul(0x0b, tmp[3]);
        s[3] = mul(0x0b, tmp[0]) ^ mul(0x0d, tmp[1]) ^ mul(0x09, tmp[2]) ^ mul(0x0e, tmp[3]);
        for i in 0..4 {
            state[i][j] = s[i];
        }
    }
}
fn mix_columns(state: &mut Vec<Vec<u8>>) {
    for j in 0..Nb {
        let mut tmp = vec![];
        for i in 0..4 {
            tmp.push(state[i][j]);
        }
        let mut s = vec![0; 4];
        s[0] = mul(0x02, tmp[0]) ^ mul(0x03, tmp[1]) ^ tmp[2] ^ tmp[3];
        s[1] = tmp[0] ^ mul(0x02, tmp[1]) ^ mul(0x03, tmp[2]) ^ tmp[3];
        s[2] = tmp[0] ^ tmp[1] ^ mul(0x02, tmp[2]) ^ mul(0x03, tmp[3]);
        s[3] = mul(0x03, tmp[0]) ^ tmp[1] ^ tmp[2] ^ mul(0x02, tmp[3]);
        for i in 0..4 {
            state[i][j] = s[i];
        }
    }
}

fn xtime(n: u8) -> u8 {
    if n & 0b1000_0000 != 0 {
        n.wrapping_shl(1) ^ 0x1b
    } else {
        n << 1
    }
}

fn mul(a: u8, b: u8) -> u8 {
    let mut res = 0;
    for i in 0..8 {
        if a & (1 << i) != 0 {
            let mut tmp = b;
            for _ in 0..i {
                tmp = xtime(tmp);
            }
            res ^= tmp;
        }
    }
    res
}

pub fn key_expansion(key: &[u8]) -> Vec<Vec<u8>> {
    assert!(key.len() == 4 * Nk);
    let mut w = vec![vec![0; 4]; (Nb * (Nr + 1))];
    for i in 0..Nk {
        for j in 0..4 {
            w[i][j] = key[i * 4 + j];
        }
    }
    let mut i = Nk;
    while i < Nb * (Nr + 1) {
        let mut temp = w[i - 1].clone();
        if i % Nk == 0 {
            temp = xor_word(&sub_word(&rot_word(&temp)), &vec![RC[i / Nk - 1], 0, 0, 0]);
        } else if Nk > 6 || i % Nk == 4 {
            temp = sub_word(&temp);
        }
        w[i] = xor_word(&w[i - Nk], &temp);
        i += 1;
    }

    w
}

fn xor_word(input1: &Vec<u8>, input2: &Vec<u8>) -> Vec<u8> {
    assert!(input1.len() == 4 && input2.len() == 4);
    let mut res = vec![];
    for (a, b) in input1.iter().zip(input2.iter()) {
        res.push(a ^ b);
    }
    res
}

fn rot_word(input: &Vec<u8>) -> Vec<u8> {
    assert!(input.len() == 4);
    let mut res = input.clone();
    res.rotate_left(1);
    res
}

fn sub_word(input: &Vec<u8>) -> Vec<u8> {
    assert!(input.len() == 4);
    let mut res = vec![];
    for i in 0..input.len() {
        let val = input[i];
        let row = val >> 4;
        let col = val & 0b1111;
        res.push(S_BOX[row as usize][col as usize]);
    }
    res
}

const RC: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
const S_BOX: [[u8; 16]; 16] = [
    [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76,
    ],
    [
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72,
        0xc0,
    ],
    [
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31,
        0x15,
    ],
    [
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2,
        0x75,
    ],
    [
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f,
        0x84,
    ],
    [
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58,
        0xcf,
    ],
    [
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f,
        0xa8,
    ],
    [
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3,
        0xd2,
    ],
    [
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19,
        0x73,
    ],
    [
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b,
        0xdb,
    ],
    [
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4,
        0x79,
    ],
    [
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae,
        0x08,
    ],
    [
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b,
        0x8a,
    ],
    [
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d,
        0x9e,
    ],
    [
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28,
        0xdf,
    ],
    [
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ],
];

const INV_S_BOX: [[u8; 16]; 16] = [
    [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb,
    ],
    [
        0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9,
        0xcb,
    ],
    [
        0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3,
        0x4e,
    ],
    [
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1,
        0x25,
    ],
    [
        0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6,
        0x92,
    ],
    [
        0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d,
        0x84,
    ],
    [
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45,
        0x06,
    ],
    [
        0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a,
        0x6b,
    ],
    [
        0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6,
        0x73,
    ],
    [
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf,
        0x6e,
    ],
    [
        0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe,
        0x1b,
    ],
    [
        0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a,
        0xf4,
    ],
    [
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec,
        0x5f,
    ],
    [
        0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c,
        0xef,
    ],
    [
        0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99,
        0x61,
    ],
    [
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ],
];

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_key_expansion() {
        let key = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let w = key_expansion(&key);
        for (i, line) in w.iter().enumerate() {
            println!(
                "{}\t{}",
                i,
                line.iter()
                    .map(|n| format!("{:02x}", n))
                    .fold("".to_string(), |acc, s| acc + &s),
            );
        }
        // panic!();
    }

    #[test]
    fn test_mul() {
        let a = 0x57;
        let b = 0x13;
        assert_eq!(mul(b, a), 0xfe);
        assert_eq!(mul(a, b), 0xfe);
    }

    fn str_to_arr(s: &str) -> Vec<u8> {
        let mut res = vec![];
        for item in &s.chars().into_iter().chunks(2) {
            let s = item.fold("".to_string(), |acc, c| acc + &c.to_string());
            res.push(u8::from_str_radix(&s, 16).unwrap());
        }
        res
    }

    #[test]
    fn test_cipher() {
        let input = str_to_arr("00112233445566778899aabbccddeeff");
        let key = str_to_arr("000102030405060708090a0b0c0d0e0f");
        let word = key_expansion(&key);
        let res = cipher(&input, &word);
        println!("res:");
        for i in 0..4 {
            for j in 0..Nb {
                print!("{:02x} ", res[i * Nb + j]);
            }
            println!();
        }

        let res = decipher(res.as_slice(), &word);
        assert_eq!(res, input);
    }
}
