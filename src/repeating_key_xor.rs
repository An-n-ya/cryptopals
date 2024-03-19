#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

use crate::single_byte_xor_cipher::single_byte_xor_cipher_with_histogram;
fn repeating_key_xor_to_str(input: &str, key: &str) -> String {
    let mut res = "".to_string();
    let mut cur_index = 0;
    let key_len = key.len();
    let key: Vec<u32> = key.chars().map(|c| c as u32).collect();
    for c in input.chars() {
        let c = c as u32;
        res.push((c ^ key[cur_index]) as u8 as char);
        cur_index = (cur_index + 1) % key_len;
    }
    res
}

fn repeating_key_xor(input: &str, key: &str) -> String {
    let mut res = "".to_string();
    let mut cur_index = 0;
    let key_len = key.len();
    let key: Vec<u32> = key.chars().map(|c| c as u32).collect();
    for c in input.chars() {
        let c = c as u32;
        res.push_str(&format!("{:02x}", c ^ key[cur_index]));
        cur_index = (cur_index + 1) % key_len;
    }
    res
}

fn hamming_distance(input1: &str, input2: &str) -> u32 {
    assert!(input1.len() == input2.len());
    let mut res = 0;
    for (c1, c2) in input1.chars().zip(input2.chars()) {
        let (n1, n2) = (c1 as u8, c2 as u8);
        res += (n1 ^ n2).count_ones();
    }
    res
}

fn find_key_size(input: &Vec<u8>) -> Vec<usize> {
    let calc_score = |key_size: usize| -> f32 {
        let mut arr = vec![];
        for i in 0..6 {
            let s: String = input
                .iter()
                .skip(i * key_size)
                .map(|n| *n as char)
                .take(key_size)
                .collect();
            arr.push(s);
        }
        let mut distance = 0f32;
        for ele in arr.iter().combinations(2) {
            distance += hamming_distance(&ele[0], &ele[1]) as f32;
        }
        // divide by C(6, 2)
        distance /= 15f32;
        distance /= key_size as f32;
        distance
    };

    let mut arr = vec![];
    for key_size in 2..50 {
        arr.push((calc_score(key_size), key_size));
    }
    arr.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut iter = arr.iter();
    let mut res = vec![];
    for _ in 0..5 {
        res.push(iter.next().unwrap().1);
    }
    res
}

pub fn repeating_key_xor_cipher(input: &Vec<u8>, histogram: &HashMap<u8, f64>) -> String {
    let try_cipher_with = |key_size: usize| -> (f64, String) {
        let mut score = 0f64;
        let mut key = "".to_string();
        for i in 0..key_size {
            let chunk: Vec<&u8> = input.iter().skip(i).step_by(key_size).collect();
            let (t_score, _, k) = single_byte_xor_cipher_with_histogram(&chunk, histogram);
            score += t_score;
            // println!("score: {score}, key: {}", k as u8);
            key.push(k);
        }
        (score, key)
    };
    let key_size_candidate = find_key_size(input);
    println!("key_size_candidate:{key_size_candidate:?}");
    let mut max_score = 0f64;
    let mut key = "".to_string();
    for key_size in key_size_candidate {
        let (score, k) = try_cipher_with(key_size);
        println!("score: {score}, key: {k}");
        if score > max_score {
            max_score = score;
            key = k;
        }
    }
    let mut s = "".to_string();
    for n in input {
        s.push(n.clone() as char);
    }

    repeating_key_xor_to_str(&s, &key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_key_xor() {
        let input = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let expect = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let res = repeating_key_xor(input, "ICE");
        assert_eq!(res, expect);
    }

    #[test]
    fn test_hamming_distance_calc() {
        let input1 = "this is a test";
        let input2 = "wokka wokka!!!";
        let res = hamming_distance(input1, input2);
        assert_eq!(res, 37);
    }
}
