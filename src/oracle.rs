#![allow(unused)]
use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    base64::base64_to_u8,
    block_cipher_mode::{encrypt_in_cbc_mode, encrypt_in_ecb_mode},
    pkcs::pkcs7unpadding,
};

pub struct Oracle<'a> {
    suffix: &'a [u8],
    prefix: Option<&'a [u8]>,
    key: Vec<u8>,
}

impl<'a> Oracle<'a> {
    pub fn new(suffix: &'a [u8], prefix: Option<&'a [u8]>) -> Self {
        let mut key = vec![];
        for _ in 0..16 {
            key.push(rand::random::<u8>());
        }
        Self {
            suffix,
            key,
            prefix,
        }
    }
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        let mut res = vec![];
        if let Some(prefix) = self.prefix {
            res.extend(prefix);
        }
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

fn get_block_size(oracle: &Oracle) -> usize {
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

pub fn decrypt_byte_by_byte(oracle: &Oracle) -> Vec<u8> {
    let block_size = get_block_size(&oracle);
    #[cfg(debug_oracle)]
    println!("block size: {block_size}");

    let encrypt_text = oracle.encrypt(&vec![0; block_size * 4]);
    if count_repetition_in(block_size, &encrypt_text) == 0 {
        panic!("It seemed like we are not working in ECB mode");
    }
    #[cfg(debug_oracle)]
    println!("It is in ECB mode");

    let mut prefix_len = 0usize;
    let text = vec!['A' as u8; block_size * 5];
    let cipher_text = oracle.encrypt(&text);
    let mut chunks_set = HashSet::new();
    let mut first_repeating_block = vec![];
    for chunk in cipher_text.iter().chunks(block_size).into_iter() {
        let chunk = chunk.collect_vec();
        if chunks_set.contains(&chunk) {
            first_repeating_block = chunk;
            break;
        }
        chunks_set.insert(chunk);
    }

    'find_prefix_len: for i in 0..block_size {
        let padding_text = vec!['A' as u8; block_size + i];
        let cipher_text = oracle.encrypt(&padding_text);

        for (index, chunk) in cipher_text
            .iter()
            .chunks(block_size)
            .into_iter()
            .enumerate()
        {
            let chunk = chunk.collect_vec();
            if chunk == first_repeating_block && index > 0 {
                prefix_len = (index - 1) * block_size + (block_size - i);
                break 'find_prefix_len;
            }
        }
    }

    println!("{prefix_len:?}");

    let suffix_len = oracle.encrypt(&[]).len() - prefix_len;
    let mut decrypt_text = vec![];
    for i in 0..suffix_len {
        let repeating_size = (block_size - ((decrypt_text.len() + prefix_len) % block_size)) - 1;

        let mut hack_arr = vec!['A' as u8; repeating_size];

        let initial_text = oracle.encrypt(&hack_arr);

        let compare_size = repeating_size + decrypt_text.len() + prefix_len + 1;
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
        let oracle = Oracle::new(&suffix, None);
        let res = decrypt_byte_by_byte(&oracle);
        assert_eq!(res, suffix);
    }

    #[test]
    fn test_decrypt_byte_by_byte_with_random_prefix() {
        let suffix = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";
        let suffix = base64_to_u8(suffix);
        let mut prefix = vec![];
        for _ in 0..rand::random::<u8>() % 100 {
            prefix.push(rand::random::<u8>());
        }
        let oracle = Oracle::new(&suffix, Some(&prefix));
        let res = decrypt_byte_by_byte(&oracle);
        assert_eq!(res, suffix);
    }
}
