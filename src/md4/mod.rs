mod attack1;
mod attack1_consts;
use core::num::Wrapping as W;
type Wu32 = W<u32>;

pub const S0: [Wu32; 4] = [
    W(0x6745_2301),
    W(0xEFCD_AB89),
    W(0x98BA_DCFE),
    W(0x1032_5476),
];
pub const K1: Wu32 = W(0x5A82_7999);
pub const K2: Wu32 = W(0x6ED9_EBA1);

pub fn f(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    // TODO: use x86's selection opcode
    z ^ (x & (y ^ z))
}
pub fn g(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    (x & y) | (x & z) | (y & z)
}
pub fn h(x: Wu32, y: Wu32, z: Wu32) -> Wu32 {
    x ^ y ^ z
}
pub fn op<F>(f: F, a: Wu32, b: Wu32, c: Wu32, d: Wu32, k: Wu32, s: u32) -> Wu32 
    where F: Fn(Wu32, Wu32, Wu32) -> Wu32 {
    let t = a + f(b, c, d) + k;
    W(t.0.rotate_left(s))
}
pub fn inv_op<F>(f: F, a: Wu32, b: Wu32, c: Wu32, d: Wu32, k: Wu32, s: u32, t: Wu32) -> Wu32 
    where F: Fn(Wu32, Wu32, Wu32) -> Wu32 {
    W(t.0.rotate_right(s)) - a - f(b, c, d) - k
}



