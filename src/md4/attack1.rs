use std::num::Wrapping as W;

use crate::md4::attack1_consts::ORDER_REV;

use super::{
    attack1_consts::{ROUND1Cmd, ORDER, SHIFT},
    f, inv_op, op, Wu32, S0,
};

#[derive(Debug)]
pub enum CmdType {
    Equal,
    Unset,
    Set,
}

pub struct Round1Cmd {
    pub typ: CmdType,
    pub bit: u32,
}
impl Round1Cmd {
    pub fn debug(&self, v: &str, u: &str) {
        print!("\tperforming round1 command: ");
        let bit = self.bit + 1;
        match self.typ {
            CmdType::Equal => print!("{v}_{} = {u}_{}", bit, bit),
            CmdType::Unset => print!("{v}_{} = 0", bit),
            CmdType::Set => print!("{v}_{} = 1", bit),
        }
        println!();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StateType {
    A,
    B,
    C,
    D,
}

impl From<usize> for StateType {
    fn from(value: usize) -> Self {
        match value {
            0 => StateType::A,
            1 => StateType::B,
            2 => StateType::C,
            3 => StateType::D,
            i => panic!("unexpected value {i}"),
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
    val: Wu32,
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}: ", self.typ, self.num).unwrap();
        write!(f, "{}", format_wu32(self.val)).unwrap();
        Ok(())
    }
}
fn format_wu32(v: Wu32) -> String {
    let bytes = format!("{:0>32b}", v.0);
    let a = [&bytes[0..8], &bytes[8..16], &bytes[16..24], &bytes[24..32]].join("_");
    a
}
impl Default for State {
    fn default() -> Self {
        Self {
            typ: StateType::A,
            num: 0,
            val: W(0),
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
    u ^ ((u ^ v) & W(1 << i))
}
fn correct_bit_set(u: Wu32, i: u32) -> Wu32 {
    u | W(1 << i)
}
fn correct_bit_unset(u: Wu32, i: u32) -> Wu32 {
    u & W(!(1 << i))
}

fn check(msg: &[Wu32; 16]) {
    let mut state = [State::default(); 4];
    for (i, iv) in S0.iter().enumerate() {
        state[i] = State {
            typ: i.into(),
            num: 0,
            val: iv.clone(),
        }
    }
    let mut vals = state.iter().map(|v| v.val.clone()).collect::<Vec<_>>();
    for i in 0..16 {
        let idx = ORDER[i % 4];
        let idx_num = idx as usize;
        let v = op(
            f,
            vals[idx_num],
            vals[(idx_num + 1) % 4],
            vals[(idx_num + 2) % 4],
            vals[(idx_num + 3) % 4],
            msg[i],
            SHIFT[i % 4],
        );
        vals[idx_num] = v;
        let round = i / 4 + 1;
        let cur_state = format!("{:?}{}", idx, round);
        let prev_state = format!(
            "{:?}{}",
            ORDER_REV[(idx_num + 1) % 4],
            round - if idx == StateType::A { 1 } else { 0 }
        );
        println!("checker v: {}", format_wu32(v));
        let cmds = ROUND1Cmd[i];
        for cmd in cmds {
            cmd.debug(&cur_state, &prev_state);
            match cmd.typ {
                CmdType::Equal => {
                    assert!(
                        (v ^ vals[(idx_num + 1) % 4]).0 & (1 << cmd.bit) == 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
                CmdType::Unset => {
                    assert!(v.0 & (1 << cmd.bit) == 0, "cmd {:?} failed, i: {}", cmd.typ, i)
                }
                CmdType::Set => {
                    assert!(v.0 & (1 << cmd.bit) != 0, "cmd {:?} failed, i: {}", cmd.typ, i)
                }
            }
        }
    }
}

fn attack_round1(
    state: &mut [State; 4],
    idx: StateType,
    msg_idx: usize,
    msg: &mut [Wu32; 16],
    shift: u32,
    cmds: &[Round1Cmd],
) {
    let vals = state.iter().map(|v| v.val.clone()).collect::<Vec<_>>();
    let st = state
        .iter_mut()
        .find(|s| s.typ == idx)
        .expect("cannot find state");
    (*st).num += 1;
    let idx_num = idx as usize;
    let mut v = op(
        f,
        vals[idx_num],
        vals[(idx_num + 1) % 4],
        vals[(idx_num + 2) % 4],
        vals[(idx_num + 3) % 4],
        msg[msg_idx],
        shift,
    );
    let round = (*st).num;
    let cur_state = format!("{:?}{}", st.typ, round);
    let prev_state = format!(
        "{:?}{}",
        ORDER_REV[(idx_num + 1) % 4],
        round - if idx == StateType::A { 1 } else { 0 }
    );
    println!("performing on {}:", cur_state);
    println!("\tinit_v:\t{}", format_wu32(v));
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal => correct_bit_equal(v, vals[(idx_num + 1) % 4], bit),
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        };
        cmd.debug(&cur_state, &prev_state);
        println!("\tcmd_v:\t{}", format_wu32(v));
    }

    let old_msg = msg[msg_idx];
    msg[msg_idx] = inv_op(
        f,
        vals[idx_num],
        vals[(idx_num + 1) % 4],
        vals[(idx_num + 2) % 4],
        vals[(idx_num + 3) % 4],
        W(0),
        shift,
        v,
    );
    println!(
        "old_msg: {}, new_msg: {}",
        format_wu32(old_msg),
        format_wu32(msg[msg_idx])
    );

    (*st).val = v;
}

fn attack() -> [Wu32; 16] {
    let mut m = [W(u32::MAX); 16];
    // for v in m.iter_mut() {
    //     *v = W(rand::random());
    // }
    let mut state = [State::default(); 4];
    for (i, iv) in S0.iter().enumerate() {
        state[i] = State {
            typ: i.into(),
            num: 0,
            val: iv.clone(),
        }
    }
    #[cfg(test)]
    print_state(&state);
    for i in 0..16 {
        attack_round1(
            &mut state,
            ORDER[i % 4],
            i,
            &mut m,
            SHIFT[i % 4],
            ROUND1Cmd[i],
        );
        #[cfg(test)]
        if i % 4 == 3 {
            print_state(&state);
        }
    }
    m
}

fn print_state(state: &[State; 4]) {
    for o in ORDER {
        println!("{:?}", state[o as usize]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round1() {
        let msg = attack();
        check(&msg);
    }
}
