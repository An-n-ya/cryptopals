#![allow(unused)]

use itertools::Itertools;
use std::num::Wrapping as W;

use crate::md4::{attack1_consts::ORDER_REV, g, K1};

use super::{
    attack1_consts::{ORDER, ROUND1_CMD, ROUND2_CMD, SHIFT1, SHIFT2},
    f, get_state, get_states_from, h, inv_op, md4, op,
    state::{State, StateType},
    Wu32, K2, S0,
};

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
        let cmds = ROUND1_CMD[i].iter();
        for cmd in cmds {
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
            // cmd.debug(&cur_state, &prev_state, 2);
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
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal(_) => correct_bit_equal(v, vals[(idx_num + 1) % 4], bit),
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        };
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
    for cmd in cmds {
        let bit = cmd.bit;
        v = match cmd.typ {
            CmdType::Equal(v_id) => {
                let state_id = v_id.unwrap() as usize;
                correct_bit_equal(v, vals[state_id], bit)
            }
            CmdType::Unset => correct_bit_unset(v, bit),
            CmdType::Set => correct_bit_set(v, bit),
        };
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

    (*st).val = v;

    // multi-step correction
    get_state(&mut affecting_state, msg);

    old_states[3] = affecting_state;
    let ind = affecting_state.typ as usize;

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
}

fn create_colliding_msg(msg: [Wu32; 16]) -> ([Wu32; 16], [Wu32; 16]) {
    let mut n_msg = msg.clone();
    n_msg[1] = msg[1] + W(1 << 31);
    n_msg[2] = msg[2] + W((1 << 31) - (1 << 28));
    n_msg[12] = msg[12] - W(1 << 16);
    (msg, n_msg)
}

fn convert_msg(msg: [Wu32; 16]) -> Vec<u8> {
    msg.iter()
        .map(|v| v.0.to_le_bytes())
        .collect::<Vec<_>>()
        .concat()
}

fn format_msg(msg: [Wu32; 16]) -> String {
    convert_msg(msg)
        .iter()
        .fold("".to_string(), |acc, v| format!("{acc}{v:0>2x}"))
}

fn attack() {
    loop {
        let msg = generate_msg();
        // maybe this msg is not fully satisfy the conditions a1~d4, but the
        // conditions in Wang's paper is not necessary, it is still good to
        // have a try
        let (msg, n_msg) = create_colliding_msg(msg);
        let (md4_1, md4_2) = (md4(&convert_msg(msg)), md4(&convert_msg(n_msg)));
        if md4_1 == md4_2 {
            let (msg, n_msg) = (format_msg(msg), format_msg(n_msg));
            println!("we found it!\nM1: {}\nM2: {}\nmd4: {}", msg, n_msg, md4_1);
            break;
        }
    }
}

fn generate_msg() -> [Wu32; 16] {
    let mut m = [W(u32::MAX); 16];
    for v in m.iter_mut() {
        *v = W(rand::random());
    }
    let mut state = [State::default(); 4];
    for (i, iv) in S0.iter().enumerate() {
        state[i] = State {
            typ: i.into(),
            num: 0,
            val: iv.clone(),
        }
    }
    for i in 0..16 {
        attack_round1(
            &mut state,
            ORDER[i % 4],
            i,
            &mut m,
            SHIFT1[i % 4],
            ROUND1_CMD[i],
        );
    }
    let msg_idx_arr = [0, 4, 8, 12];
    let affecting_states = [
        State {
            typ: StateType::A,
            num: 1,
            val: W(0),
        },
        State {
            typ: StateType::A,
            num: 2,
            val: W(0),
        },
        State {
            typ: StateType::A,
            num: 3,
            val: W(0),
        },
        State {
            typ: StateType::A,
            num: 4,
            val: W(0),
        },
    ];
    for (i, cmds) in ROUND2_CMD.iter().enumerate() {
        attack_round2(
            &mut state,
            ORDER[i % 4],
            i / 4 + msg_idx_arr[i % 4],
            &mut m,
            SHIFT2[i % 4],
            cmds,
            affecting_states[i],
        );
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md4_attack() {
        attack();
        panic!()
    }

    #[test]
    fn test_round1() {
        let msg = generate_msg();
        check(&msg);
    }
}
