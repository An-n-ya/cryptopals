#![allow(unused)]
use std::num::Wrapping as W;

use crate::md4::{attack1_consts::ORDER_REV, g, K1};

use super::{
    attack1_consts::{ORDER, ROUND1_CMD, ROUND2_CMD, SHIFT1, SHIFT2},
    f, inv_op, op, Wu32, S0,
};

// TODO: refactor

#[derive(Debug)]
pub enum CmdType {
    Equal(Option<StateType>),
    Unset,
    Set,
}

pub struct Cmd {
    pub typ: CmdType,
    pub bit: u32,
}
impl Cmd {
    pub fn debug(&self, v: &str, u: &str, round: usize) {
        print!("\tperforming round {round} command: ");
        let bit = self.bit + 1;
        match self.typ {
            CmdType::Equal(_) => print!("{v}_{} = {u}_{}", bit, bit),
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

impl StateType {
    pub fn prev(&self) -> StateType {
        use StateType::*;
        match self {
            A => B,
            B => C,
            C => D,
            D => A,
        }
    }
    pub fn next(&self) -> StateType {
        use StateType::*;
        match self {
            A => D,
            B => A,
            C => B,
            D => C,
        }
    }
}

impl PartialOrd for StateType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order = [0, 3, 2, 1];
        order[*self as usize].partial_cmp(&order[*other as usize])
    }
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

#[derive(Clone, Copy, PartialEq)]
struct State {
    typ: StateType,
    num: u8,
    val: Wu32,
}
impl State {
    pub fn prev_n(&self, mut n: usize) -> State {
        let mut typ = self.typ.clone();
        let mut num = self.num;
        while n > 0 {
            if typ == StateType::A {
                assert!(n > 0, "A0 doesn't have previous state");
                num -= 1;
            }
            typ = typ.prev();
            n -= 1;
        }
        State {
            typ,
            num,
            val: W(0),
        }
    }
    pub fn next_n(&self, mut n: usize) -> State {
        let mut typ = self.typ.clone();
        let mut num = self.num;
        while n > 0 {
            if typ == StateType::B {
                num += 1;
            }
            typ = typ.next();
            n -= 1;
        }
        State {
            typ,
            num,
            val: W(0),
        }
    }
    pub fn prev(&self) -> State {
        self.prev_n(1)
    }
    pub fn next(&self) -> State {
        self.next_n(1)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}: ", self.typ, self.num).unwrap();
        write!(f, "{}", format_wu32(self.val)).unwrap();
        Ok(())
    }
}
fn format_wu32(v: Wu32) -> String {
    // let bytes = format!("{:0>32b}", v.0);
    // let a = [&bytes[0..8], &bytes[8..16], &bytes[16..24], &bytes[24..32]].join("_");
    // a
    format!("{:x}", v.0)
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.num == other.num {
            self.typ.partial_cmp(&other.typ)
        } else {
            self.num.partial_cmp(&other.num)
        }
    }
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
            SHIFT1[i % 4],
        );
        vals[idx_num] = v;
        let round = i / 4 + 1;
        let cur_state = format!("{:?}{}", idx, round);
        let prev_state = format!(
            "{:?}{}",
            ORDER_REV[(idx_num + 1) % 4],
            round - if idx == StateType::A { 1 } else { 0 }
        );
        // println!("checker v: {}", format_wu32(v));
        let cmds = ROUND1_CMD[i].iter();
        for cmd in cmds {
            cmd.debug(&cur_state, &prev_state, 1);
            match cmd.typ {
                CmdType::Equal(_) => {
                    assert!(
                        (v ^ vals[(idx_num + 1) % 4]).0 & (1 << cmd.bit) == 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
                CmdType::Unset => {
                    assert!(
                        v.0 & (1 << cmd.bit) == 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
                CmdType::Set => {
                    assert!(
                        v.0 & (1 << cmd.bit) != 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
            }
        }
    }
    // check round2
    for (i, cmds) in ROUND2_CMD.iter().enumerate() {
        let idx_remain = i % 4;
        let idx = ORDER[idx_remain];
        let idx_num = idx as usize;
        let v = op(
            g,
            vals[idx_num],
            vals[(idx_num + 1) % 4],
            vals[(idx_num + 2) % 4],
            vals[(idx_num + 3) % 4],
            msg[idx_remain * 4 + i / 4] + K1,
            SHIFT2[i % 4],
        );
        vals[idx_num] = v;
        let round = i / 4 + 5;
        let cur_state = format!("{:?}{}", idx, round);
        for cmd in *cmds {
            let prev_ind = {
                match cmd.typ {
                    CmdType::Equal(prev) => {
                        if let Some(prev) = prev {
                            prev as usize
                        } else {
                            (idx_num + 1) % 4
                        }
                    }
                    _ => (idx_num + 1) % 4,
                }
            };
            let prev_state = format!(
                "{:?}{}",
                ORDER_REV[prev_ind],
                round - if idx == StateType::A { 1 } else { 0 }
            );
            cmd.debug(&cur_state, &prev_state, 2);
            match cmd.typ {
                CmdType::Equal(_) => {
                    assert!(
                        (v ^ vals[prev_ind]).0 & (1 << cmd.bit) == 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
                CmdType::Unset => {
                    assert!(
                        v.0 & (1 << cmd.bit) == 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
                }
                CmdType::Set => {
                    assert!(
                        v.0 & (1 << cmd.bit) != 0,
                        "cmd {:?} failed, i: {}",
                        cmd.typ,
                        i
                    )
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
    cmds: &[Cmd],
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
    // println!("performing on {}:", cur_state);
    // println!("\tinit_v:\t{}", format_wu32(v));
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal(_) => correct_bit_equal(v, vals[(idx_num + 1) % 4], bit),
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        };
        //cmd.debug(&cur_state, &prev_state);
        // println!("\tcmd_v:\t{}", format_wu32(v));
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
    // println!(
    //     "old_msg: {}, new_msg: {}",
    //     format_wu32(old_msg),
    //     format_wu32(msg[msg_idx])
    // );

    (*st).val = v;
}
fn attack_round2(
    state: &mut [State; 4],
    idx: StateType,
    msg_idx: usize,
    msg: &mut [Wu32; 16],
    shift: u32,
    cmds: &[Cmd],
    mut affecting_state: State,
) {
    let vals = state.iter().map(|v| v.val.clone()).collect::<Vec<_>>();
    let st = state
        .iter_mut()
        .find(|s| s.typ == idx)
        .expect("cannot find state");
    (*st).num += 1;
    let idx_num = idx as usize;
    let mut v = op(
        g,
        vals[idx_num],
        vals[(idx_num + 1) % 4],
        vals[(idx_num + 2) % 4],
        vals[(idx_num + 3) % 4],
        msg[msg_idx] + K1,
        shift,
    );
    let round = (*st).num;
    let cur_state = format!("{:?}{}", st.typ, round);
    let mut prev_state = format!(
        "{:?}{}",
        ORDER_REV[(idx_num + 1) % 4],
        round - if idx == StateType::A { 1 } else { 0 }
    );
    println!("performing on {}:", cur_state);
    println!("\tinit_v:\t{}", format_wu32(v));
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal(v_id) => {
                let state_id = v_id.unwrap() as usize;
                prev_state = format!(
                    "{:?}{}",
                    ORDER[state_id],
                    round - if idx == StateType::A { 1 } else { 0 }
                );
                correct_bit_equal(v, vals[state_id], bit)
            }
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        };
        cmd.debug(&cur_state, &prev_state, 2);
        println!("\tcmd_v:\t{}", format_wu32(v));
    }

    let mut old_states = get_states_from(affecting_state.prev_n(3), 8, msg);
    let old_msg = msg[msg_idx];
    msg[msg_idx] = inv_op(
        g,
        vals[idx_num],
        vals[(idx_num + 1) % 4],
        vals[(idx_num + 2) % 4],
        vals[(idx_num + 3) % 4],
        K1,
        shift,
        v,
    );
    println!(
        "old_msg: {}, new_msg: {}",
        format_wu32(old_msg),
        format_wu32(msg[msg_idx])
    );

    (*st).val = v;

    // multi-step correction
    get_state(&mut affecting_state, msg);
    println!("affecting_state: {:?}", affecting_state);
    println!("old_states: {:?}", old_states);

    // msg[msg_idx + 1] = inv_op(f, d0.val, new_a1.val, b0.val, c0.val, W(0), SHIFT1[1], d1.val);
    // msg[msg_idx + 2] = inv_op(f, c0.val, d1.val, new_a1.val, b0.val, W(0), SHIFT1[2], c1.val);
    // msg[msg_idx + 3] = inv_op(f, b0.val, c1.val, d1.val, new_a1.val, W(0), SHIFT1[3], b1.val);
    // msg[msg_idx + 4] = inv_op(f, new_a1.val, b1.val, c1.val, d1.val, W(0), SHIFT1[0], a2.val);

    old_states[3] = affecting_state;
    let ind = affecting_state.typ as usize;
    // msg[msg_idx + 1] = inv_op(f, old_states[0].val, old_states[3].val, old_states[2].val, old_states[1].val, W(0), SHIFT1[ind + 1], old_states[4].val);
    // msg[msg_idx + 2] = inv_op(f, old_states[1].val, old_states[4].val, old_states[3].val, old_states[2].val, W(0), SHIFT1[ind + 2], old_states[5].val);
    // msg[msg_idx + 3] = inv_op(f, old_states[2].val, old_states[5].val, old_states[4].val, old_states[3].val, W(0), SHIFT1[ind + 3], old_states[6].val);
    // msg[msg_idx + 4] = inv_op(f, old_states[3].val, old_states[6].val, old_states[5].val, old_states[4].val, W(0), SHIFT1[ind + 0], old_states[7].val);

    for i in 0..4 {
        msg[msg_idx + 1 + i] = inv_op(
            f,
            old_states[i].val,
            old_states[3 + i].val,
            old_states[2 + i].val,
            old_states[1 + i].val,
            W(0),
            SHIFT1[(i + 1 + ind) % 4],
            old_states[4 + i].val,
        );
    }
    let new_states = get_states_from(affecting_state.prev_n(3), 8, msg);
    println!("new_states: {:?}", new_states);
}
fn get_state(state: &mut State, msg: &[Wu32; 16]) {
    let states = get_states_from(state.clone(), 1, msg);
    state.val = states[0].val;
}
// NOTE: only work in first round
fn get_states_from(mut state: State, mut n: usize, msg: &[Wu32; 16]) -> Vec<State> {
    let mut states = [State::default(); 4];
    for (i, iv) in S0.iter().enumerate() {
        states[i] = State {
            typ: i.into(),
            num: 0,
            val: iv.clone(),
        }
    }
    let mut res = vec![];
    while state.num == 0 && n > 0 {
        res.push(states[state.typ as usize]);

        state = state.next();
        n -= 1;
    }
    if n == 0 {
        return res;
    }
    assert!(state.num > 0 && n > 0);
    let mut cur_state = State {
        typ: StateType::A,
        num: 1,
        val: W(0),
    };

    let order = [0, 3, 2, 1];
    let mut i = 0;
    while cur_state < state {
        let ind = ORDER[i % 4] as usize;
        let s = op(
            f,
            states[ind].val,
            states[(ind + 1) % 4].val,
            states[(ind + 2) % 4].val,
            states[(ind + 3) % 4].val,
            msg[i],
            SHIFT1[i % 4],
        );
        states[ind].val = s;
        i = i + 1;
        cur_state = cur_state.next();
    }
    while n > 0 {
        let ind = ORDER[i % 4] as usize;
        let s = op(
            f,
            states[ind].val,
            states[(ind + 1) % 4].val,
            states[(ind + 2) % 4].val,
            states[(ind + 3) % 4].val,
            msg[i],
            SHIFT1[i % 4],
        );
        states[ind].val = s;
        state.val = s;
        res.push(state.clone());
        i = i + 1;
        state = state.next();
        n -= 1;
    }
    res
}

fn attack() -> [Wu32; 16] {
    let mut m = [W(u32::MAX); 16];
    // for v in m.iter_mut() {
    //     *v = W(rand::random());
    // }A0: 67452301
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
            SHIFT1[i % 4],
            ROUND1_CMD[i],
        );
        #[cfg(test)]
        if i % 4 == 3 {
            print_state(&state);
        }
    }
    let msg_idx_arr = [0, 4, 8, 12];
    let affecting_states = [
        State {
            typ: StateType::A,
            num: 1,
            val: W(0)
        },
        State {
            typ: StateType::A,
            num: 2,
            val: W(0)
        },
        State {
            typ: StateType::A,
            num: 3,
            val: W(0)
        },
        State {
            typ: StateType::A,
            num: 4,
            val: W(0)
        },
    ];
    for (i, cmds) in ROUND2_CMD.iter().enumerate() {
        attack_round2(&mut state, ORDER[i % 4], i / 4 + msg_idx_arr[i % 4], &mut m, SHIFT2[i % 4], cmds, affecting_states[i]);
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

    #[test]
    fn test_get_state() {
        let states: [u32; 20] = [
            0x67452301, 0x10325476, 0x98badcfe, 0xefcdab89, 0xffffffb7, 0x1f80, 0x44e430c4,
            0x59dd534c, 0x420c7d2, 0x626141a0, 0x4104af5, 0xc742bff0, 0x4260f609, 0x5b3079d4,
            0x3801a653, 0x2310d19, 0x691356ec, 0x950ef735, 0x8a67f1d9, 0x966b1a40,
        ];
        let msg = [
            W(0xfffffff7),
            W(0x0),
            W(0x8fffffff),
            W(0xff7fffff),
            W(0xffbffc7f),
            W(0x2fbf),
            W(0xfffffd79),
            W(0xfdfffffa),
            W(0xa0bff),
            W(0x4605f),
            W(0xfff3bf1f),
            W(0xffffeffe),
            W(0xf1bfffff),
            W(0xffe7ffff),
            W(0xfffc487f),
            W(0x80000e7f),
        ];
        for i in 0..=4 {
            for t in ORDER {
                let mut state = State {
                    typ: t,
                    num: i,
                    val: W(0),
                };
                get_state(&mut state, &msg);
                let expect = states[state.num as usize * 4 + ORDER[state.typ as usize] as usize];
                assert_eq!(
                    state.val.0, expect,
                    "diff on i: {:?}, expect: 0x{:x}",
                    state, expect
                );
            }
        }
        for i in 0..=3 {
            for t in ORDER {
                let mut state = State {
                    typ: t,
                    num: i,
                    val: W(0),
                };
                let res = get_states_from(state, 4, &msg);
                let start_ind = state.num as usize * 4 + ORDER[state.typ as usize] as usize;
                let expects = &states[start_ind..start_ind + 4];
                for (expect, got) in expects.iter().zip(res) {
                    assert_eq!(
                        got.val.0, *expect,
                        "diff on i: {:?}, expect: 0x{:x}",
                        got, expect
                    );
                }
            }
        }
    }
}
