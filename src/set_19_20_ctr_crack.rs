use crate::{
    block_cipher_mode::encrypt_in_ctr_mode,
    misc::{get_eng_histogram, rand_vec},
    single_byte_xor_cipher::single_byte_xor_cipher_with_histogram,
    xor::xor_data,
};

const BLOCK_SIZE: usize = 16;
struct Oracle {
    nonce: u64,
    key: Vec<u8>,
}

impl Oracle {
    pub fn new(nonce: u64) -> Self {
        Self {
            nonce,
            key: rand_vec(BLOCK_SIZE),
        }
    }
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        encrypt_in_ctr_mode(input, &self.key, self.nonce)
    }
}

pub fn crack_ctr_statically(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let max_len = input.iter().fold(0, |acc, data| acc.max(data.len()));
    let histogram = get_eng_histogram();
    let mut key = "".to_string();
    for i in 0..max_len {
        let mut columns = vec![];
        for line in input {
            if let Some(val) = line.get(i) {
                columns.push(val);
            }
        }
        let (_, _, k) = single_byte_xor_cipher_with_histogram(&columns, &histogram);
        key.push(k);
    }

    let mut plain_text = vec![];
    let key: Vec<u8> = key.chars().map(|c| c as u8).collect();
    for line in input {
        plain_text.push(xor_data(&line, &key));
    }
    plain_text
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::base64::base64_to_u8;

    use super::*;

    const SET_18_INPUT: &str = "SSBoYXZlIG1ldCB0aGVtIGF0IGNsb3NlIG9mIGRheQ==
Q29taW5nIHdpdGggdml2aWQgZmFjZXM=
RnJvbSBjb3VudGVyIG9yIGRlc2sgYW1vbmcgZ3JleQ==
RWlnaHRlZW50aC1jZW50dXJ5IGhvdXNlcy4=
SSBoYXZlIHBhc3NlZCB3aXRoIGEgbm9kIG9mIHRoZSBoZWFk
T3IgcG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==
T3IgaGF2ZSBsaW5nZXJlZCBhd2hpbGUgYW5kIHNhaWQ=
UG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==
QW5kIHRob3VnaHQgYmVmb3JlIEkgaGFkIGRvbmU=
T2YgYSBtb2NraW5nIHRhbGUgb3IgYSBnaWJl
VG8gcGxlYXNlIGEgY29tcGFuaW9u
QXJvdW5kIHRoZSBmaXJlIGF0IHRoZSBjbHViLA==
QmVpbmcgY2VydGFpbiB0aGF0IHRoZXkgYW5kIEk=
QnV0IGxpdmVkIHdoZXJlIG1vdGxleSBpcyB3b3JuOg==
QWxsIGNoYW5nZWQsIGNoYW5nZWQgdXR0ZXJseTo=
QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=
VGhhdCB3b21hbidzIGRheXMgd2VyZSBzcGVudA==
SW4gaWdub3JhbnQgZ29vZCB3aWxsLA==
SGVyIG5pZ2h0cyBpbiBhcmd1bWVudA==
VW50aWwgaGVyIHZvaWNlIGdyZXcgc2hyaWxsLg==
V2hhdCB2b2ljZSBtb3JlIHN3ZWV0IHRoYW4gaGVycw==
V2hlbiB5b3VuZyBhbmQgYmVhdXRpZnVsLA==
U2hlIHJvZGUgdG8gaGFycmllcnM/
VGhpcyBtYW4gaGFkIGtlcHQgYSBzY2hvb2w=
QW5kIHJvZGUgb3VyIHdpbmdlZCBob3JzZS4=
VGhpcyBvdGhlciBoaXMgaGVscGVyIGFuZCBmcmllbmQ=
V2FzIGNvbWluZyBpbnRvIGhpcyBmb3JjZTs=
SGUgbWlnaHQgaGF2ZSB3b24gZmFtZSBpbiB0aGUgZW5kLA==
U28gc2Vuc2l0aXZlIGhpcyBuYXR1cmUgc2VlbWVkLA==
U28gZGFyaW5nIGFuZCBzd2VldCBoaXMgdGhvdWdodC4=
VGhpcyBvdGhlciBtYW4gSSBoYWQgZHJlYW1lZA==
QSBkcnVua2VuLCB2YWluLWdsb3Jpb3VzIGxvdXQu
SGUgaGFkIGRvbmUgbW9zdCBiaXR0ZXIgd3Jvbmc=
VG8gc29tZSB3aG8gYXJlIG5lYXIgbXkgaGVhcnQs
WWV0IEkgbnVtYmVyIGhpbSBpbiB0aGUgc29uZzs=
SGUsIHRvbywgaGFzIHJlc2lnbmVkIGhpcyBwYXJ0
SW4gdGhlIGNhc3VhbCBjb21lZHk7
SGUsIHRvbywgaGFzIGJlZW4gY2hhbmdlZCBpbiBoaXMgdHVybiw=
VHJhbnNmb3JtZWQgdXR0ZXJseTo=
QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=";
    const SET_18_PLAINTEXT: &str = "i have met them at close of da0
coming with vivid faces
from counter or desk among gre0
eighteenth-century houses.
i have passed with a nod of th,dhi46
or polite meaningless words,
or have lingered awhile and sa
polite meaningless words,
and thought before I had done
of a mocking tale or a gibe
to please a companion
around the fire at the club,
being certain that they and I
but lived where motley is worns
all changed, changed utterly:
a terrible beauty is born.
that woman's days were spent
in ignorant good will,
her nights in argument
until her voice grew shrill.
what voice more sweet than her:
when young and beautiful,
she rode to harriers?
this man had kept a school
and rode our winged horse.
this other his helper and frie' 
was coming into his force;
he might have won fame in the ,*d 
so sensitive his nature seemede
so daring and sweet his though=j
this other man I had dreamed
a drunken, vain-glorious lout.
he had done most bitter wrong
to some who are near my heart,
yet I number him in the song;
he, too, has resigned his part
in the casual comedy;
he, too, has been changed in h 7 x    
transformed utterly:
a terrible beauty is born.";

    #[test]
    fn test_set_19() {
        let oracle = Oracle::new(0);
        let cipher_text: Vec<Vec<u8>> = SET_18_INPUT
            .split('\n')
            .map(|s| base64_to_u8(s))
            .map(|data| oracle.encrypt(&data))
            .collect();
        let plain_text = crack_ctr_statically(&cipher_text);
        let mut res = vec![];
        for line in plain_text {
            let s: String = line
                .iter()
                .filter(|n| **n != 0)
                .map(|n| *n as char)
                .collect();
            res.push(s);
        }
        res.join("\n");
        // assert_eq!(res, SET_18_PLAINTEXT);
    }

    #[test]
    fn test_set_20() {
        let oracle = Oracle::new(rand::random::<u64>());
        let s = fs::read_to_string("20.txt").unwrap();
        let expect = fs::read_to_string("20_expect.txt").unwrap();
        let cipher_text: Vec<Vec<u8>> = s
            .split('\n')
            .map(|s| base64_to_u8(s))
            .map(|data| oracle.encrypt(&data))
            .collect();
        let plain_text = crack_ctr_statically(&cipher_text);
        let mut res = vec![];
        for line in plain_text {
            let s: String = line
                .iter()
                .filter(|n| **n != 0)
                .map(|n| *n as char)
                .collect();
            res.push(s);
        }
        let res = res.join("\n");
        assert_eq!(res, expect);
    }
}
