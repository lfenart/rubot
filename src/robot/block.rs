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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub fn from_byte(n: u8) -> Option<Self> {
        match n {
            b'I' => Some(Block::I),
            b'J' => Some(Block::J),
            b'L' => Some(Block::L),
            b'O' => Some(Block::O),
            b'S' => Some(Block::S),
            b'T' => Some(Block::T),
            b'Z' => Some(Block::Z),
            _ => None,
        }
    }

    pub fn get_byte(self) -> u8 {
        match self {
            Block::I => 0,
            Block::J => 1,
            Block::L => 2,
            Block::O => 3,
            Block::S => 4,
            Block::T => 5,
            Block::Z => 6,
        }
    }

    pub fn value(self, rotation: u8) -> u16 {
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

    pub fn get_kick(rotation: u8, spin: u8) -> &'static [(isize, isize)] {
        &KICK[(spin / 2) as usize][rotation as usize]
    }
}
