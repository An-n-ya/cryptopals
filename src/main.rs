use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use misc::get_eng_histogram;
use repeating_key_xor::repeating_key_xor_cipher;
use single_byte_xor_cipher::single_byte_xor_cipher;

use crate::{base64::base64_to_u8, block_cipher_mode::decrypt_in_ecb_mode};

mod aes;
mod base64;
mod block_cipher_mode;
mod misc;
mod oracle;
mod pkcs;
mod repeating_key_xor;
mod set_17_cbc_padding_oracle;
mod set_19_20_ctr_crack;
mod single_byte_xor_cipher;
mod xor;
mod mt19937;

fn main() {
    let mut f = File::open("/home/annya/Documents/20k_words.txt").unwrap();
    let mut word_list = "".to_string();
    f.read_to_string(&mut word_list).unwrap();
    #[allow(unused)]
    let word_list: HashSet<&str> = word_list.lines().collect();

    let histogram = get_eng_histogram();
    // repeating_xor_cipher(histogram);
    aes_in_ecb_mode_decipher();
}

#[allow(dead_code)]
fn single_xor_cipher(word_list: HashSet<&str>) {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = "".to_string();
    f.read_to_string(&mut buffer).unwrap();

    let mut max_score = 0;
    let mut res = "".to_string();
    for line in buffer.lines() {
        let (score, s) = single_byte_xor_cipher(line, &word_list);
        if score > max_score {
            max_score = score;
            res = s;
        }
    }

    println!("{res}");
}

#[allow(dead_code)]
fn repeating_xor_cipher(histogram: HashMap<u8, f64>) {
    let mut f = File::open("6.txt").unwrap();
    let mut buffer = "".to_string();
    f.read_to_string(&mut buffer).unwrap();
    let buffer = base64_to_u8(&buffer);
    let res = repeating_key_xor_cipher(&buffer, &histogram);
    println!("{res}");
}

#[allow(dead_code)]
fn aes_in_ecb_mode_decipher() {
    let mut f = File::open("7.txt").unwrap();
    let mut buffer = "".to_string();
    f.read_to_string(&mut buffer).unwrap();
    let key = b"YELLOW SUBMARINE";
    let buffer = base64_to_u8(&buffer);
    let res = decrypt_in_ecb_mode(&buffer, key);
    let res = res
        .iter()
        .fold("".to_string(), |acc, n| acc + &(*n as char).to_string());
    println!("{res}");
}
