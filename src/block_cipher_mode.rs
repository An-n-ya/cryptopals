#![allow(unused)]

use std::borrow::Borrow;

use itertools::Itertools;

use crate::{
    aes::{cipher, decipher, key_expansion},
    pkcs::{self, pkcs7padding, pkcs7unpadding},
    xor::xor_data,
};

pub fn encrypt_in_cbc_mode(input: &[u8], key: &[u8], initial_vector: &[u8]) -> Vec<u8> {
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

pub fn decrypt_in_cbc_mode_without_unpadding(
    input: &[u8],
    key: &[u8],
    initial_vector: &[u8],
) -> Vec<u8> {
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
    res
}
pub fn decrypt_in_cbc_mode(input: &[u8], key: &[u8], initial_vector: &[u8]) -> Vec<u8> {
    let res = decrypt_in_cbc_mode_without_unpadding(input, key, initial_vector);
    pkcs7unpadding(&res, 16)
}
pub fn encrypt_in_ecb_mode(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    let word = &key_expansion(key);
    for chunk in &input.iter().chunks(16) {
        let input: Vec<u8> = chunk.map(|n| n.clone()).collect();
        let input = pkcs7padding(&input, 16);
        res.extend(cipher(&input, word));
    }
    res
}
pub fn decrypt_in_ecb_mode(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    let word = &key_expansion(key);
    for chunk in &input.iter().chunks(16) {
        let input: Vec<u8> = chunk.map(|n| n.clone()).collect();
        res.extend(decipher(&input, word));
    }
    pkcs7unpadding(&res, 16)
}

pub fn encrypt_in_ctr_mode(input: &[u8], key: &[u8], nonce: u64) -> Vec<u8> {
    let mut counter = 0u64;
    let mut result = vec![];
    for data in input.iter().chunks(16).borrow() {
        let mut nonce_ctr = vec![];
        nonce.to_le_bytes().iter().for_each(|n| nonce_ctr.push(*n));
        counter
            .to_le_bytes()
            .iter()
            .for_each(|n| nonce_ctr.push(*n));
        let res = encrypt_in_ecb_mode(&nonce_ctr, key);
        let data = data.map(|n| *n).collect_vec();
        let res = xor_data(&data, &res);

        res.iter().for_each(|n| result.push(*n));

        counter += 1;
    }
    result
}
pub fn decrypt_in_ctr_mode(input: &[u8], key: &[u8], nonce: u64) -> Vec<u8> {
    encrypt_in_ctr_mode(input, key, nonce)
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
        let input = b"Trying to decrypt something to see if its works";
        let res = encrypt_in_cbc_mode(&input.to_vec(), key, &iv);
        let res = decrypt_in_cbc_mode(&res, key, &iv);
        assert_eq!(res, input);
        let res: String = res.iter().map(|n| *n as char).collect();
        println!("{res}");
    }

    #[test]
    fn test_ctr_mode() {
        let key = b"YELLOW SUBMARINE";
        let input = base64_to_u8(
            "L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==",
        );
        let res = encrypt_in_ctr_mode(&input, key, 0);
        assert_eq!(res, b"Yo, VIP Let's kick it Ice, Ice, baby Ice, Ice, baby ");

        let key = b"key with 16 byte";
        let input = b"Trying to decrypt something to see if its works";
        let encrypt_data = encrypt_in_ctr_mode(input, key, u64::MAX);
        let decrypt_data = decrypt_in_ctr_mode(&encrypt_data, key, u64::MAX);
        assert_eq!(decrypt_data, input);
    }
}
