use serde::Serialize;

const KICK: [[[(isize, isize); 5]; 4]; 2] = [
    [
        // spin = 1
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ],
    [
        // spin = 3
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ],
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Block {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Block {
    pub fn value(self, rotation: i8) -> u16 {
        match self {
            Block::I => match rotation & 0x3 {
                0 => 0x00f0,
                1 => 0x4444,
                2 => 0x0f00,
                3 => 0x2222,
                _ => unreachable!(),
            },
            Block::J => match rotation & 0x3 {
                0 => 0x0710,
                1 => 0x2260,
                2 => 0x4700,
                3 => 0x3220,
                _ => unreachable!(),
            },
            Block::L => match rotation & 0x3 {
                0 => 0x0740,
                1 => 0x6220,
                2 => 0x1700,
                3 => 0x2230,
                _ => unreachable!(),
            },
            Block::O => 0x0660,
            Block::S => match rotation & 0x3 {
                0 => 0x0360,
                1 => 0x4620,
                2 => 0x3600,
                3 => 0x2310,
                _ => unreachable!(),
            },
            Block::T => match rotation & 0x3 {
                0 => 0x0720,
                1 => 0x2620,
                2 => 0x2700,
                3 => 0x2320,
                _ => unreachable!(),
            },
            Block::Z => match rotation & 0x3 {
                0 => 0x0630,
                1 => 0x2640,
                2 => 0x6300,
                3 => 0x1320,
                _ => unreachable!(),
            },
        }
    }

    pub fn get_kick(rotation: i8, spin: i8) -> &'static [(isize, isize)] {
        &KICK[(spin >> 1) as usize][rotation as usize]
    }
}

use std::convert::TryFrom;

impl TryFrom<char> for Block {
    type Error = char;

    fn try_from(n: char) -> Result<Block, Self::Error> {
        match n {
            'I' => Ok(Block::I),
            'J' => Ok(Block::J),
            'L' => Ok(Block::L),
            'O' => Ok(Block::O),
            'S' => Ok(Block::S),
            'T' => Ok(Block::T),
            'Z' => Ok(Block::Z),
            _ => Err(n),
        }
    }
}

impl From<Block> for char {
    fn from(block: Block) -> char {
        match block {
            Block::I => 'I',
            Block::J => 'J',
            Block::L => 'L',
            Block::O => 'O',
            Block::S => 'S',
            Block::T => 'T',
            Block::Z => 'Z',
        }
    }
}
