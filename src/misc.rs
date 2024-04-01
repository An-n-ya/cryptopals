#![allow(unused)]
use crate::block_cipher_mode::{decrypt_in_ecb_mode, encrypt_in_ecb_mode};

pub fn rand_vec(len: usize) -> Vec<u8> {
    let mut res = vec![];
    for i in 0..len {
        res.push(rand::random::<u8>());
    }
    res
}

pub struct Profile {
    email: String,
    uid: usize,
    role: String,
}

pub struct ProfileCrypt {
    key: Vec<u8>,
}

pub fn profile_for(email: &str) -> Profile {
    let mut email = email.replace("&", "");
    email = email.replace("=", "");

    Profile {
        email,
        uid: 10,
        role: "user".to_string(),
    }
}

impl ProfileCrypt {
    pub fn new() -> Self {
        let mut key = vec![];
        for _ in 0..16 {
            key.push(rand::random::<u8>());
        }
        Self { key }
    }
    pub fn decrypt_profile(&self, input: &[u8]) -> Profile {
        let data = decrypt_in_ecb_mode(input, &self.key);
        println!("{data}");
        let val: Vec<&str> = data.split("&").collect();
        let email = val[0].strip_prefix("email=").unwrap().to_string();
        let uid = val[1]
            .strip_prefix("uid=")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let role = val[2].strip_prefix("role=").unwrap().to_string();
        Profile { email, uid, role }
    }
    pub fn encrypt_profile(&self, profile: &Profile) -> Vec<u8> {
        let input: Vec<u8> = profile.encode().chars().map(|c| c as u8).collect();
        encrypt_in_ecb_mode(&input, &self.key)
    }
}

impl Profile {
    pub fn encode(&self) -> String {
        format!("email={}&uid={}&role={}", self.email, self.uid, self.role)
    }
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cur_and_paste() {
        let crypt = ProfileCrypt::new();

        // block1: email=YELLOW SUB
        // block2: master\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b
        // other block
        let email = "YELLOW SUBadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b";

        let text1 = crypt.encrypt_profile(&profile_for(email));

        // block1: email=annya@some
        // block2: .cn&uid=10&role=
        // other block
        let email = "annya@some.cn";
        let text2 = crypt.encrypt_profile(&profile_for(email));

        let final_text = [&text2[0..32], &text1[16..32]].concat();

        let cracked_profile = crypt.decrypt_profile(&final_text);
        println!("{}", cracked_profile.encode());
        assert!(cracked_profile.is_admin())
    }
}
