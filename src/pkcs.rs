#![allow(unused)]
pub fn pkcs7padding(input: &[u8], k: u8) -> Vec<u8> {
    let l = input.len();
    if l == k as usize {
        return input.to_vec();
    }
    let padding_size = k as u8 - (l % k as usize) as u8;
    let padding = vec![padding_size; padding_size as usize];
    let mut res = vec![];
    res.extend(input);
    res.extend(padding);
    res
}
pub fn pkcs7unpadding(input: &[u8], k: u8) -> Vec<u8> {
    println!("{input:?} len:{}", input.len());
    let l = input.len();
    let padding_size = *input.last().unwrap();
    if !is_pkcs7_padding(input, k) {
        return input.to_vec();
    }
    let input = &input[0..l - padding_size as usize];
    input.to_vec()
}
pub fn is_pkcs7_padding(input: &[u8], k: u8) -> bool {
    let l = input.len();
    let padding_size = *input.last().unwrap();
    input[l - padding_size as usize..l]
        .iter()
        .all(|n| *n == padding_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs7padding() {
        let input = b"YELLOW SUBMARINE";
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        let res = pkcs7padding(input, 20);
        assert_eq!(res, expected);
        let res = pkcs7unpadding(&res, 20);
        assert_eq!(res, input);

        let input = b"ICE ICE BABY\x04\x04\x04\x04";
        assert!(is_pkcs7_padding(input, 16));
        let input = b"ICE ICE BABY\x05\x05\x05\x05";
        assert!(!is_pkcs7_padding(input, 16));
        let input = b"ICE ICE BABY\x01\x02\x03\x04";
        assert!(!is_pkcs7_padding(input, 16));
    }
}
