#![allow(dead_code)]
pub fn xor_data(input1: &[u8], input2: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    for (n1, n2) in input1.iter().zip(input2.iter()) {
        res.push(n1 ^ n2);
    }

    res
}

fn xor(input1: &str, input2: &str) -> String {
    let mut res = "".to_string();
    for (c1, c2) in input1.chars().zip(input2.chars()) {
        let n1 = c1.to_digit(16).unwrap();
        let n2 = c2.to_digit(16).unwrap();
        res.push_str(&format!("{:x}", n1 ^ n2));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let expect = "746865206b696420646f6e277420706c6179";

        let res = xor(input1, input2);
        assert_eq!(res, expect);
    }
}
