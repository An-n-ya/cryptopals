#![allow(unused)]

use itertools::Itertools;

use crate::{
    aes::{cipher, decipher, key_expansion},
    pkcs::{self, pkcs7padding, pkcs7unpadding},
    xor::xor_data,
};

pub fn encrypt_in_cbc_mode(input: &Vec<u8>, key: &[u8], initial_vector: &[u8]) -> Vec<u8> {
    assert!(initial_vector.len() == 16);
    let mut res = vec![];
    let word = &key_expansion(key);
    let mut last_input = initial_vector.to_vec();
    for chunk in &input.iter().chunks(16) {
        let input: Vec<u8> = chunk.map(|n| n.clone()).collect();
        let input = pkcs7padding(&input, 16);
        let input = xor_data(&input, &last_input);
        let cipher_text = cipher(&input, word);
        last_input = cipher_text;
        res.extend(last_input.iter());
    }
    res
}

pub fn decrypt_in_cbc_mode(input: &Vec<u8>, key: &[u8], initial_vector: &[u8]) -> Vec<u8> {
    assert!(initial_vector.len() == 16);
    let mut res = vec![];
    let word = &key_expansion(key);
    let mut last_input = initial_vector.to_vec();
    for chunk in &input.iter().chunks(16) {
        let input: Vec<u8> = chunk.map(|n| n.clone()).collect();
        let cipher_text = decipher(&input, word);
        let plain_text = xor_data(&cipher_text, &last_input);
        last_input = input;
        res.extend(plain_text.iter());
    }
    pkcs7unpadding(&res, 16)
}
pub fn decipher_in_ecb_mode(input: &Vec<u8>, key: &[u8]) -> String {
    let mut res = vec![];
    let word = &key_expansion(key);
    for chunk in &input.iter().chunks(16) {
        let input: Vec<u8> = chunk.map(|n| n.clone()).collect();
        res.extend(decipher(&input, word));
    }
    res.iter()
        .fold("".to_string(), |acc, n| acc + &(*n as char).to_string())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::base64::base64_to_u8;

    use super::*;

    #[test]
    fn test_cbc_mode() {
        let iv = [0; 16];
        let key = b"YELLOW SUBMARINE";
        let mut file = File::open("10.txt").unwrap();
        let mut buffer = "".to_string();
        file.read_to_string(&mut buffer);
        let buffer = base64_to_u8(&buffer);
        let res = decrypt_in_cbc_mode(&buffer, key, &iv);
        let res: String = res.iter().map(|n| *n as char).collect();
        println!("{res}");
        // panic!();
    }

    #[test]
    fn test_basic() {
        let iv = [0; 16];
        let key = b"YELLOW SUBMARINE";
        let input = b"Trying to decrypt something to see if its works";
        let res = encrypt_in_cbc_mode(&input.to_vec(), key, &iv);
        let res = decrypt_in_cbc_mode(&res, key, &iv);
        assert_eq!(res, input);
        let res: String = res.iter().map(|n| *n as char).collect();
        println!("{res}");
    }
}
