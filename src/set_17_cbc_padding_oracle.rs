#![allow(unused)]
use itertools::Itertools;

use crate::{
    block_cipher_mode::{
        decrypt_in_cbc_mode, decrypt_in_cbc_mode_without_unpadding, encrypt_in_cbc_mode,
    },
    misc::rand_vec,
    pkcs::is_pkcs7_padding,
};

const BLOCK_SIZE: usize = 16;
struct Oracle {
    pub initial_value: Vec<u8>,
    key: Vec<u8>,
}

impl Oracle {
    pub fn new() -> Self {
        Self {
            initial_value: rand_vec(BLOCK_SIZE),
            key: rand_vec(BLOCK_SIZE),
        }
    }
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        encrypt_in_cbc_mode(input, &self.key, &self.initial_value)
    }
    pub fn decrypt(&self, input: &[u8], initial_value: &[u8]) -> bool {
        let data = decrypt_in_cbc_mode_without_unpadding(input, &self.key, initial_value);
        is_pkcs7_padding(&data, BLOCK_SIZE as u8)
    }
    fn decrypt_debug(&self, input: &[u8], initial_value: &[u8]) -> (bool, Vec<u8>) {
        let data = decrypt_in_cbc_mode_without_unpadding(input, &self.key, initial_value);
        (is_pkcs7_padding(&data, BLOCK_SIZE as u8), data)
    }
}

fn crack(data: &[u8], initial_value: &[u8], oracle: &Oracle) -> Vec<u8> {
    let initial_value_vec = initial_value.to_vec();
    let mut blocks = vec![initial_value_vec];
    data.iter()
        .chunks(BLOCK_SIZE)
        .into_iter()
        .map(|chunk| chunk.into_iter().map(|n| *n).collect_vec())
        .for_each(|val| blocks.push(val));

    let mut decrypted_data: Vec<Vec<u8>> = vec![];

    for (prev_block, cur_block) in blocks.iter().zip(blocks.iter().skip(1)) {
        let mut arr = prev_block.clone();
        let mut decrypted_block = vec![];
        for i in 0..BLOCK_SIZE as u8 {
            let padding_start_index = BLOCK_SIZE - i as usize - 1;
            let padding_size = i + 1;
            let mut possible_guess = vec![];
            for guess in 0..=255 {
                // println!("guess {guess}, padding_start_index: {padding_start_index}, padding_size: {padding_size}");
                arr[padding_start_index] = prev_block[padding_start_index] ^ guess ^ padding_size;
                for (index, n) in decrypted_block.iter().rev().enumerate() {
                    arr[padding_start_index + index + 1] =
                        prev_block[padding_start_index + index + 1] ^ n ^ padding_size;
                }

                if oracle.decrypt(&cur_block, &arr) {
                    // println!("guess value: {guess}");
                    possible_guess.push(guess);
                }
            }
            if possible_guess.len() > 1 {
                // we have multiple candidates, we use next byte to judge which is better
                let padding_size = 1 + padding_size;
                let mut final_guess = possible_guess[0];
                'guess: for guess in &possible_guess {
                    for next_guess in 0..=255 {
                        arr[padding_start_index] =
                            prev_block[padding_start_index] ^ guess ^ padding_size;
                        arr[padding_start_index - 1] =
                            prev_block[padding_start_index - 1] ^ next_guess ^ padding_size;
                        for (index, n) in decrypted_block.iter().rev().enumerate() {
                            arr[padding_start_index + index + 1] =
                                prev_block[padding_start_index + index + 1] ^ n ^ padding_size;
                        }
                        if oracle.decrypt(&cur_block, &arr) {
                            // println!("choose guess value: {guess}");
                            final_guess = *guess;
                            break 'guess;
                        }
                    }
                }
                possible_guess.clear();
                possible_guess.push(final_guess);
            }
            decrypted_block.push(possible_guess[0]);
            // println!("decrypted_block {:?}", decrypted_block);
        }
        decrypted_data.push(decrypted_block.into_iter().rev().collect());
    }

    decrypted_data.concat()
}

#[cfg(test)]
mod tests {
    use crate::{base64::base64_to_u8, pkcs::pkcs7unpadding};

    use super::*;

    const S: [&str; 10] = [
        "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
        "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
        "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
        "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
        "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
        "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
        "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
        "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
        "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93",
    ];

    #[test]
    fn test_set_17() {
        let oracle = Oracle::new();
        let input = base64_to_u8(S[rand::random::<usize>() % 10]);
        println!("input {input:?}");
        let encrypt_data = oracle.encrypt(&input);
        let decrypt_data = oracle.decrypt(&encrypt_data, &oracle.initial_value);
        let crack_data = crack(&encrypt_data, &oracle.initial_value, &oracle);
        let crack_data = pkcs7unpadding(&crack_data, BLOCK_SIZE as u8);
        println!("crack {crack_data:?}");
        assert_eq!(crack_data, input);
    }
}
