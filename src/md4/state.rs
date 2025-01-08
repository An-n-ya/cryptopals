use super::Wu32;
use std::num::Wrapping as W;

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
pub struct State {
    pub typ: StateType,
    pub num: u8,
    pub val: Wu32,
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
