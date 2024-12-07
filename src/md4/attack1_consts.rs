use super::attack1::{CmdType, Round1Cmd, StateType};

pub const SHIFT: [u32; 4] = [3, 7, 11, 9];
pub const ORDER: [StateType; 4] = [StateType::A, StateType::D, StateType::C, StateType::B];
pub const ORDER_REV: [StateType; 4] = [StateType::A, StateType::B, StateType::C, StateType::D];

pub const ROUND1Cmd: [&[Round1Cmd]; 16] = {
    use CmdType::*;
    [
    &[
      Round1Cmd{typ: Equal, bit: 6}
    ],
    &[
      Round1Cmd{typ: Unset, bit: 6},
      Round1Cmd{typ: Equal, bit: 7},
      Round1Cmd{typ: Equal, bit: 10},
    ],
    &[
      Round1Cmd{typ: Set, bit: 6},
      Round1Cmd{typ: Set, bit: 7},
      Round1Cmd{typ: Unset, bit: 10},
      Round1Cmd{typ: Equal, bit: 25},
    ],
    &[
      Round1Cmd{typ: Set, bit: 6},
      Round1Cmd{typ: Unset, bit: 7},
      Round1Cmd{typ: Unset, bit: 10},
      Round1Cmd{typ: Unset, bit: 25},
    ],
    &[
      Round1Cmd{typ: Set, bit: 7},
      Round1Cmd{typ: Set, bit: 10},
      Round1Cmd{typ: Unset, bit: 25},
      Round1Cmd{typ: Equal, bit: 13},
    ],
    &[
      Round1Cmd{typ: Unset, bit: 13},
      Round1Cmd{typ: Equal, bit: 18},
      Round1Cmd{typ: Equal, bit: 19},
      Round1Cmd{typ: Equal, bit: 20},
      Round1Cmd{typ: Equal, bit: 21},
      Round1Cmd{typ: Set, bit: 25},
    ],
    &[
      Round1Cmd{typ: Equal, bit: 12},
      Round1Cmd{typ: Unset, bit: 13},
      Round1Cmd{typ: Equal, bit: 14},
      Round1Cmd{typ: Unset, bit: 18},
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Set, bit: 20},
      Round1Cmd{typ: Unset, bit: 21},
    ],
    &[
      Round1Cmd{typ: Set, bit: 12},
      Round1Cmd{typ: Set, bit: 13},
      Round1Cmd{typ: Unset, bit: 14},
      Round1Cmd{typ: Equal, bit: 16},
      Round1Cmd{typ: Unset, bit: 18},
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Unset, bit: 20},
      Round1Cmd{typ: Unset, bit: 21},
    ],
    &[
      Round1Cmd{typ: Set, bit: 12},
      Round1Cmd{typ: Set, bit: 13},
      Round1Cmd{typ: Set, bit: 14},
      Round1Cmd{typ: Unset, bit: 16},
      Round1Cmd{typ: Unset, bit: 18},
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Unset, bit: 20},
      Round1Cmd{typ: Equal, bit: 22},
      Round1Cmd{typ: Set, bit: 21},
      Round1Cmd{typ: Equal, bit: 25},
    ],
    &[
      Round1Cmd{typ: Set, bit: 12},
      Round1Cmd{typ: Set, bit: 13},
      Round1Cmd{typ: Set, bit: 14},
      Round1Cmd{typ: Unset, bit: 16},
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Set, bit: 20},
      Round1Cmd{typ: Set, bit: 21},
      Round1Cmd{typ: Unset, bit: 22},
      Round1Cmd{typ: Set, bit: 25},
      Round1Cmd{typ: Equal, bit: 29},
    ],
    &[
      Round1Cmd{typ: Set, bit: 16},
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Unset, bit: 20},
      Round1Cmd{typ: Unset, bit: 21},
      Round1Cmd{typ: Unset, bit: 22},
      Round1Cmd{typ: Unset, bit: 25},
      Round1Cmd{typ: Set, bit: 29},
      Round1Cmd{typ: Equal, bit: 31},
    ],
    &[
      Round1Cmd{typ: Unset, bit: 19},
      Round1Cmd{typ: Set, bit: 20},
      Round1Cmd{typ: Set, bit: 21},
      Round1Cmd{typ: Equal, bit: 22},
      Round1Cmd{typ: Set, bit: 25},
      Round1Cmd{typ: Unset, bit: 29},
      Round1Cmd{typ: Unset, bit: 31},
    ],
    &[
      Round1Cmd{typ: Unset, bit: 22},
      Round1Cmd{typ: Unset, bit: 25},
      Round1Cmd{typ: Equal, bit: 26},
      Round1Cmd{typ: Equal, bit: 28},
      Round1Cmd{typ: Set, bit: 29},
      Round1Cmd{typ: Unset, bit: 31},
    ],
    &[
      Round1Cmd{typ: Unset, bit: 22},
      Round1Cmd{typ: Unset, bit: 25},
      Round1Cmd{typ: Set, bit: 26},
      Round1Cmd{typ: Set, bit: 28},
      Round1Cmd{typ: Unset, bit: 29},
      Round1Cmd{typ: Set, bit: 31},
    ],
    &[
      Round1Cmd{typ: Equal, bit: 18},
      Round1Cmd{typ: Set, bit: 22},
      Round1Cmd{typ: Set, bit: 25},
      Round1Cmd{typ: Unset, bit: 26},
      Round1Cmd{typ: Unset, bit: 28},
      Round1Cmd{typ: Unset, bit: 29},
    ],
    &[
      Round1Cmd{typ: Unset, bit: 18},
      Round1Cmd{typ: Equal, bit: 25},
      Round1Cmd{typ: Set, bit: 26},
      Round1Cmd{typ: Set, bit: 28},
      Round1Cmd{typ: Unset, bit: 29},
      Round1Cmd{typ: Equal, bit: 31},
    ],
    ]
};
