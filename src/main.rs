use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use aes::decipher_in_ecb_mode;
use repeating_key_xor::repeating_key_xor_cipher;
use single_byte_xor_cipher::single_byte_xor_cipher;

use crate::base64::base64_to_u8;

mod aes;
mod base64;
mod repeating_key_xor;
mod single_byte_xor_cipher;
mod xor;

fn main() {
    let mut f = File::open("/home/annya/Documents/20k_words.txt").unwrap();
    let mut word_list = "".to_string();
    f.read_to_string(&mut word_list).unwrap();
    #[allow(unused)]
    let word_list: HashSet<&str> = word_list.lines().collect();

    let mut f = File::open("hemingwaye-oldmanandthesea.txt").unwrap();
    let mut buffer = "".to_string();
    f.read_to_string(&mut buffer).unwrap();
    let buffer_len = buffer.len() as f64;
    let mut histogram = HashMap::new();
    for c in buffer.chars() {
        let c = c as u8;
        histogram.insert(c, histogram.get(&c).unwrap_or(&0f64) + 1f64);
    }
    for i in 0..=255 {
        histogram.insert(i, histogram.get(&i).unwrap_or(&0f64) / buffer_len);
    }

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
    let res = decipher_in_ecb_mode(&buffer, key);
    println!("{res}");
}
