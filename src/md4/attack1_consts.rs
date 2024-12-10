use super::attack1::{Cmd, CmdType, StateType};

pub const SHIFT1: [u32; 4] = [3, 7, 11, 19];
pub const SHIFT2: [u32; 4] = [3, 5, 9, 13];
pub const ORDER: [StateType; 4] = [StateType::A, StateType::D, StateType::C, StateType::B];
pub const ORDER_REV: [StateType; 4] = [StateType::A, StateType::B, StateType::C, StateType::D];

pub const ROUND1_CMD: [&[Cmd]; 16] = {
    use CmdType::*;
    [
        &[Cmd { typ: Equal(None), bit: 6 }],
        &[
            Cmd { typ: Unset, bit: 6 },
            Cmd { typ: Equal(None), bit: 7 },
            Cmd {
                typ: Equal(None),
                bit: 10,
            },
        ],
        &[
            Cmd { typ: Set, bit: 6 },
            Cmd { typ: Set, bit: 7 },
            Cmd {
                typ: Unset,
                bit: 10,
            },
            Cmd {
                typ: Equal(None),
                bit: 25,
            },
        ],
        &[
            Cmd { typ: Set, bit: 6 },
            Cmd { typ: Unset, bit: 7 },
            Cmd {
                typ: Unset,
                bit: 10,
            },
            Cmd {
                typ: Unset,
                bit: 25,
            },
        ],
        &[
            Cmd { typ: Set, bit: 7 },
            Cmd { typ: Set, bit: 10 },
            Cmd {
                typ: Unset,
                bit: 25,
            },
            Cmd {
                typ: Equal(None),
                bit: 13,
            },
        ],
        &[
            Cmd {
                typ: Unset,
                bit: 13,
            },
            Cmd {
                typ: Equal(None),
                bit: 18,
            },
            Cmd {
                typ: Equal(None),
                bit: 19,
            },
            Cmd {
                typ: Equal(None),
                bit: 20,
            },
            Cmd {
                typ: Equal(None),
                bit: 21,
            },
            Cmd { typ: Set, bit: 25 },
        ],
        &[
            Cmd {
                typ: Equal(None),
                bit: 12,
            },
            Cmd {
                typ: Unset,
                bit: 13,
            },
            Cmd {
                typ: Equal(None),
                bit: 14,
            },
            Cmd {
                typ: Unset,
                bit: 18,
            },
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd { typ: Set, bit: 20 },
            Cmd {
                typ: Unset,
                bit: 21,
            },
        ],
        &[
            Cmd { typ: Set, bit: 12 },
            Cmd { typ: Set, bit: 13 },
            Cmd {
                typ: Unset,
                bit: 14,
            },
            Cmd {
                typ: Equal(None),
                bit: 16,
            },
            Cmd {
                typ: Unset,
                bit: 18,
            },
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd {
                typ: Unset,
                bit: 20,
            },
            Cmd {
                typ: Unset,
                bit: 21,
            },
        ],
        &[
            Cmd { typ: Set, bit: 12 },
            Cmd { typ: Set, bit: 13 },
            Cmd { typ: Set, bit: 14 },
            Cmd {
                typ: Unset,
                bit: 16,
            },
            Cmd {
                typ: Unset,
                bit: 18,
            },
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd {
                typ: Unset,
                bit: 20,
            },
            Cmd {
                typ: Equal(None),
                bit: 22,
            },
            Cmd { typ: Set, bit: 21 },
            Cmd {
                typ: Equal(None),
                bit: 25,
            },
        ],
        &[
            Cmd { typ: Set, bit: 12 },
            Cmd { typ: Set, bit: 13 },
            Cmd { typ: Set, bit: 14 },
            Cmd {
                typ: Unset,
                bit: 16,
            },
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd { typ: Set, bit: 20 },
            Cmd { typ: Set, bit: 21 },
            Cmd {
                typ: Unset,
                bit: 22,
            },
            Cmd { typ: Set, bit: 25 },
            Cmd {
                typ: Equal(None),
                bit: 29,
            },
        ],
        &[
            Cmd { typ: Set, bit: 16 },
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd {
                typ: Unset,
                bit: 20,
            },
            Cmd {
                typ: Unset,
                bit: 21,
            },
            Cmd {
                typ: Unset,
                bit: 22,
            },
            Cmd {
                typ: Unset,
                bit: 25,
            },
            Cmd { typ: Set, bit: 29 },
            Cmd {
                typ: Equal(None),
                bit: 31,
            },
        ],
        &[
            Cmd {
                typ: Unset,
                bit: 19,
            },
            Cmd { typ: Set, bit: 20 },
            Cmd { typ: Set, bit: 21 },
            Cmd {
                typ: Equal(None),
                bit: 22,
            },
            Cmd { typ: Set, bit: 25 },
            Cmd {
                typ: Unset,
                bit: 29,
            },
            Cmd {
                typ: Unset,
                bit: 31,
            },
        ],
        &[
            Cmd {
                typ: Unset,
                bit: 22,
            },
            Cmd {
                typ: Unset,
                bit: 25,
            },
            Cmd {
                typ: Equal(None),
                bit: 26,
            },
            Cmd {
                typ: Equal(None),
                bit: 28,
            },
            Cmd { typ: Set, bit: 29 },
            Cmd {
                typ: Unset,
                bit: 31,
            },
        ],
        &[
            Cmd {
                typ: Unset,
                bit: 22,
            },
            Cmd {
                typ: Unset,
                bit: 25,
            },
            Cmd { typ: Set, bit: 26 },
            Cmd { typ: Set, bit: 28 },
            Cmd {
                typ: Unset,
                bit: 29,
            },
            Cmd { typ: Set, bit: 31 },
        ],
        &[
            Cmd {
                typ: Equal(None),
                bit: 18,
            },
            Cmd { typ: Set, bit: 22 },
            Cmd { typ: Set, bit: 25 },
            Cmd {
                typ: Unset,
                bit: 26,
            },
            Cmd {
                typ: Unset,
                bit: 28,
            },
            Cmd {
                typ: Unset,
                bit: 29,
            },
        ],
        &[
            Cmd {
                typ: Unset,
                bit: 18,
            },
            Cmd {
                typ: Equal(None),
                bit: 25,
            },
            Cmd { typ: Set, bit: 26 },
            Cmd { typ: Set, bit: 28 },
            Cmd {
                typ: Unset,
                bit: 29,
            },
            Cmd {
                typ: Equal(None),
                bit: 31,
            },
        ],
    ]
};
pub const ROUND2_CMD: [&[Cmd]; 1] = {
    use CmdType::*;
    [&[
        Cmd {
            typ: Equal(Some(StateType::C)),
            bit: 18,
        },
        Cmd { typ: Set, bit: 25 },
        Cmd { typ: Unset, bit: 26 },
        Cmd { typ: Set, bit: 28 },
        Cmd { typ: Set, bit: 31 },
    ]]
};
