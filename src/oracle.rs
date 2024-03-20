#![allow(unused)]
use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    base64::base64_to_u8,
    block_cipher_mode::{encrypt_in_cbc_mode, encrypt_in_ecb_mode},
    pkcs::pkcs7unpadding,
};

pub struct SuffixOracle<'a> {
    suffix: &'a [u8],
    key: Vec<u8>,
}

impl<'a> SuffixOracle<'a> {
    pub fn new(suffix: &'a [u8]) -> Self {
        let mut key = vec![];
        for _ in 0..16 {
            key.push(rand::random::<u8>());
        }
        Self { suffix, key }
    }
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut res = vec![];
        res.extend(input);
        res.extend(self.suffix);

        res = encrypt_in_ecb_mode(&res, &self.key);
        res
    }
}
pub fn prefix_suffix_oracle(input: &[u8]) -> (&str, Vec<u8>) {
    let mut key = vec![];
    for _ in 0..16 {
        key.push(rand::random::<u8>());
    }
    let mut res = vec![];
    for _ in 0..rand::random::<u8>() % 6 + 5 {
        res.push(rand::random::<u8>());
    }
    res.extend(input);
    for _ in 0..rand::random::<u8>() % 6 + 5 {
        res.push(rand::random::<u8>());
    }
    let mut mode = "CBC";

    if rand::random() {
        res = encrypt_in_ecb_mode(&res, &key);
        mode = "ECB"
    } else {
        let mut iv = vec![];
        for _ in 0..16 {
            iv.push(rand::random::<u8>());
        }
        res = encrypt_in_cbc_mode(&res, &key, &iv);
    }
    (mode, res)
}

pub fn count_repetition_in(n: usize, input: &[u8]) -> usize {
    // let mut chunks: Vec<String> = vec![];
    let mut chunks = vec![];
    input.iter().chunks(n).into_iter().for_each(|chunk| {
        // chunks.push(chunk.map(|&n| n as char).collect());
        chunks.push(chunk.collect_vec());
    });
    let l = chunks.len();
    let mut set = HashSet::new();
    for n in chunks {
        set.insert(n);
    }

    l - set.len()
}

fn get_block_size(oracle: &SuffixOracle) -> usize {
    let initial_len = oracle.encrypt(&[]).len();
    let mut data = vec![];
    // the max block size of AES is 128
    // so our max guess is 128
    for i in 0..128 {
        data.push('A' as u8);
        let cur_len = oracle.encrypt(&data).len();
        if cur_len != initial_len {
            return cur_len - initial_len;
        }
    }
    unreachable!()
}

pub fn decrypt_byte_by_byte(oracle: &SuffixOracle) -> Vec<u8> {
    let block_size = get_block_size(&oracle);
    #[cfg(debug_oracle)]
    println!("block size: {block_size}");

    let encrypt_text = oracle.encrypt(&vec![0; block_size * 2]);
    if count_repetition_in(block_size, &encrypt_text) == 0 {
        panic!("It seemed like we are not working in ECB mode");
    }
    #[cfg(debug_oracle)]
    println!("It is in ECB mode");

    let suffix_len = oracle.encrypt(&[]).len();
    let mut decrypt_text = vec![];
    for i in 0..suffix_len {
        let repeating_size = (block_size - (decrypt_text.len() % block_size)) - 1;

        let mut hack_arr = vec!['A' as u8; repeating_size];

        let initial_text = oracle.encrypt(&hack_arr);

        let compare_size = repeating_size + decrypt_text.len() + 1;
        hack_arr.extend(&decrypt_text);
        for guess in 0..=255u8 {
            hack_arr.push(guess);
            // hack_arr.iter().for_each(|n| print!("{}", *n as char));
            // println!();
            // println!("compare_size: {compare_size}");
            let encrypt_text = oracle.encrypt(&hack_arr);
            if encrypt_text[0..compare_size] == initial_text[0..compare_size] {
                decrypt_text.push(guess);
                break;
            }

            hack_arr.pop();
        }
        #[cfg(debug_oracle)]
        {
            println!(
                "repeating_size: {repeating_size}, hack_arr {hack_arr:?}, len: {}",
                hack_arr.len()
            );
            println!("decrypt_text: {:?}", decrypt_text);
        }
    }
    pkcs7unpadding(&decrypt_text, block_size as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let input = b"test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test";
        for i in 0..200 {
            let (expected_mode, res) = prefix_suffix_oracle(input);
            let mode = if count_repetition_in(16, &res) > 0 {
                "ECB"
            } else {
                "CBC"
            };
            assert_eq!(mode, expected_mode);
        }
    }

    #[test]
    fn test_decrypt_byte_by_byte() {
        let suffix = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";
        let suffix = base64_to_u8(suffix);
        println!("suffix {suffix:?}");
        let oracle = SuffixOracle::new(&suffix);
        let res = decrypt_byte_by_byte(&oracle);
        assert_eq!(res, suffix);
    }
}
