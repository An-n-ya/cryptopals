use std::collections::HashSet;

use itertools::Itertools;

use crate::block_cipher_mode::{encrypt_in_cbc_mode, encrypt_in_ecb_mode};

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
    println!("{chunks:?}");
    let mut set = HashSet::new();
    for n in chunks {
        set.insert(n);
    }

    l - set.len()
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
}
