use std::collections::{HashMap, HashSet};

pub fn single_byte_xor_cipher(input: &str, word_list: &HashSet<&str>) -> (u32, String) {
    let mut arr = vec![];
    let bytes: Vec<char> = input.chars().collect();
    for k in 0..256 {
        let mut s = "".to_string();
        for i in (0..bytes.len()).step_by(2) {
            let num = (bytes[i].to_digit(16).unwrap() << 4) + bytes[i + 1].to_digit(16).unwrap();
            let xor_num = num ^ k;
            assert!(xor_num < 256);
            s.push(char::from_u32(xor_num).unwrap());
        }
        let score = score_str(&s, &word_list);
        arr.push((score, s));
    }

    arr.sort();
    // let mut iter = arr.iter().rev();
    // for _ in 0..3 {
    //     if let Some(item) = iter.next() {
    //         println!("{}\t: {}", item.0, item.1);
    //     } else {
    //         break;
    //     }
    // }
    arr.last().unwrap().clone()
}

pub fn single_byte_xor_cipher_with_histogram(
    input: &Vec<&u8>,
    histogram: &HashMap<u8, f64>,
) -> (f64, String, char) {
    let mut arr = vec![];
    for k in 0..=255 {
        let mut s = "".to_string();
        for n in input {
            let xor_num = *n ^ k;
            s.push(xor_num as char);
        }
        let score = score_str_with_histogram(&s, &histogram);
        arr.push((score, s, k as u8 as char));
    }

    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    arr.last().unwrap().clone()
}

fn score_str(input: &str, word_list: &HashSet<&str>) -> u32 {
    let mut score = 0;
    for word in input.split(' ') {
        if word_list.contains(word) {
            score += 1;
        }
    }
    score
}
fn score_str_with_histogram(input: &str, histogram: &HashMap<u8, f64>) -> f64 {
    let mut score = 0f64;
    for c in input.chars() {
        let c = c as u8;
        score += histogram.get(&c).unwrap_or(&0f64);
    }

    score
}
