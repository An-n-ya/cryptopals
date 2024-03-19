#![allow(dead_code)]
enum State {
    Need8,
    Need6,
    Need4,
    Need2,
}
pub fn hex_to_base64(hex: &str) -> String {
    let mut res = "".to_string();
    let mut cur_state = State::Need6;
    let mut cur_num = 0;
    for c in hex.chars() {
        assert!(c.is_ascii_hexdigit());
        let num = u8::from_str_radix(&c.to_string(), 16).unwrap();
        match cur_state {
            State::Need6 => {
                cur_num = num << 2;
                cur_state = State::Need2;
            }
            State::Need4 => {
                cur_num |= num;
                res.push(convert(cur_num));
                cur_num = 0;
                cur_state = State::Need6
            }
            State::Need2 => {
                cur_num |= num >> 2;
                res.push(convert(cur_num));
                cur_num = (num & 0b11) << 4;
                cur_state = State::Need4
            }
            State::Need8 => unreachable!(),
        }
    }
    res
}

pub fn base64_to_u8(input: &str) -> Vec<u8> {
    let mut res = vec![];
    let mut cur_state = State::Need8;
    let mut cur_num = 0u8;
    for c in input.chars() {
        if c == '=' {
            break;
        }
        if c == '\n' {
            continue;
        }
        let num = convert_rev(c);
        match cur_state {
            State::Need8 => {
                cur_num = num << 2;
                cur_state = State::Need2;
            }
            State::Need2 => {
                cur_num |= num >> 4;
                res.push(cur_num);
                cur_num = (num & 0b1111) << 4;
                cur_state = State::Need4;
            }
            State::Need4 => {
                cur_num |= num >> 2;
                res.push(cur_num);
                cur_num = (num & 0b11) << 6;
                cur_state = State::Need6;
            }
            State::Need6 => {
                cur_num |= num;
                res.push(cur_num);
                cur_num = 0;
                cur_state = State::Need8;
            }
        }
    }
    res
}

const ASCII_A: u32 = 65;
#[allow(non_upper_case_globals)]
const ASCII_a: u32 = 97;
const ASCII_0: u32 = 48;

fn convert(num: u8) -> char {
    assert!(num < 64);
    if num <= 25 {
        char::from_u32(num as u32 + ASCII_A).unwrap()
    } else if num <= 51 {
        char::from_u32((num - 26) as u32 + ASCII_a).unwrap()
    } else if num <= 61 {
        char::from_u32((num - 52) as u32 + ASCII_0).unwrap()
    } else if num == 62 {
        '+'
    } else if num == 63 {
        '/'
    } else {
        unreachable!()
    }
}

fn convert_rev(c: char) -> u8 {
    if c <= 'Z' && c >= 'A' {
        c as u8 - 'A' as u8
    } else if c <= 'z' && c >= 'a' {
        c as u8 - 'a' as u8 + 26
    } else if c <= '9' && c >= '0' {
        c as u8 - '0' as u8 + 52
    } else if c == '+' {
        62
    } else if c == '/' {
        63
    } else {
        unreachable!("cannot handle {c} in convert_rev of base64");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_to_vec() {
        let input = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let expect = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let arr = base64_to_u8(input);
        let mut res = "".to_string();
        for n in arr {
            res.push_str(&format!("{:02x}", n));
        }
        assert_eq!(res, expect);
    }
    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expect = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let res = hex_to_base64(input);
        assert_eq!(res, expect);
    }
}
