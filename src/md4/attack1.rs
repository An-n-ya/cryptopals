use std::num::Wrapping as W;

use super::{attack1_consts::ROUND1Cmd, f, inv_op, op, Wu32, S0};

pub enum CmdType {
    Equal,
    Unset,
    Set,
}

pub struct Round1Cmd {
    pub typ: CmdType,
    pub bit: u32
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StateType {
    A,
    B,
    C,
    D
}

impl From<usize> for StateType {
    fn from(value: usize) -> Self {
        match value {
            0 => StateType::A,
            1 => StateType::B,
            2 => StateType::C,
            3 => StateType::D,
            i => panic!("unexpected value {i}")
        }
    }
}
impl Into<usize> for StateType {
    fn into(self) -> usize {
        match self {
            StateType::A => 0,
            StateType::B => 1,
            StateType::C => 2,
            StateType::D => 3,
        }
    }
}

#[derive(Clone, Copy)]
struct State {
    typ: StateType,
    num: u8,
    val: Wu32
}
impl Default for State {
    fn default() -> Self {
        Self {
            typ: StateType::A,
            num: 0,
            val: W(0)
        }
    }
}

fn create_weak_message(msg: &[Wu32; 16]) -> Vec<Wu32> {
    let mut x = Vec::from(msg);
    x[1] += W(1 << 31);
    x[2] = x[2] + W(1 << 31) - W(1 << 28);
    x[12] -= W(1 << 16);
    x
}

fn correct_bit_equal(u: Wu32, v: Wu32, i: u32) -> Wu32 {
    u ^ (u ^ v & W(1 << i)) 
}
fn correct_bit_set(u: Wu32, i: u32) -> Wu32 {
    u | W(1 << i) 
}
fn correct_bit_unset(u: Wu32, i: u32) -> Wu32 {
    u & W(!(1 << i))
}

fn attack_round1(state: &mut [State; 4], idx: StateType, msg_idx: usize, msg: &mut [Wu32; 16], shift: u32, cmds: &[Round1Cmd]) {
    let vals = state.iter().map(|v| v.val.clone()).collect::<Vec<_>>();
    let st = state.iter_mut().find(|s| s.typ == idx).expect("cannot find state");
    let idx_num= idx as usize;
    let mut v = op(f, vals[idx_num], vals[(idx_num + 1) % 4], vals[(idx_num + 2) % 4], vals[(idx_num + 3) %4], msg[msg_idx], shift);
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal => correct_bit_equal(v, vals[(idx_num + 1) % 4], bit),
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        }
    }

    msg[msg_idx] = inv_op(f, vals[idx_num], vals[(idx_num + 1) % 4], vals[(idx_num + 2) % 4], vals[(idx_num + 3) %4], W(0), shift, v);

    (*st).val = v;
}


fn attack() {
    let mut m = [W(0u32); 16];
    for v in m.iter_mut() {
        *v = W(rand::random());
    }
    let mut state = [State::default(); 4];
    for (i, iv) in S0.iter().enumerate() {
        state[i] = State {
            typ: i.into(),
            num: 0,
            val: iv.clone()
        } 
    }
    let order = [StateType::A, StateType::D, StateType::C, StateType::B];
    let shift = [3, 7, 11, 19];
    for i in 0..16 {
        attack_round1(&mut state, order[i % 4], i, &mut m, shift[i % 4], ROUND1Cmd[i]);
    }

}
