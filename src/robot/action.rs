use super::Block;

use lazy_static::lazy_static;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub struct Action {
    pub hold: bool,
    pub rotation: i8,
    pub translation: i8,
    pub spin: i8,
}

fn get_index(block: Block) -> usize {
    match block {
        Block::I => 0,
        Block::J => 1,
        Block::L => 2,
        Block::O => 3,
        Block::S => 4,
        Block::T => 5,
        Block::Z => 6,
    }
}

impl Action {
    pub fn get_list(block1: Block, block2: Block) -> &'static [Action] {
        &ACTION_LIST[get_index(block1)][get_index(block2)]
    }
}

lazy_static! {
    static ref ACTION_LIST: Vec<Vec<Vec<Action>>> = {
        fn rotations(block: &Block) -> Vec<i8> {
            match block {
                Block::I | Block::S | Block::Z => vec![0, 1],
                Block::J | Block::L | Block::T => vec![0, 1, 2, 3],
                Block::O => vec![0],
            }
        }
        fn translations(block: &Block, rotation: i8) -> std::ops::Range<i8> {
            match block {
                Block::I => match rotation {
                    0 | 2 => 0..7,
                    1 | 3 => -2..8,
                    _ => unreachable!(),
                },
                Block::O => 2..11,
                Block::J | Block::L | Block::S | Block::T | Block::Z => match rotation {
                    0 | 2 => 0..8,
                    1 => -1..8,
                    3 => 0..9,
                    _ => unreachable!(),
                },
            }
        }
        fn spins(block: &Block, rotation: i8) -> Vec<i8> {
            match block {
                Block::I | Block::O => vec![0],
                Block::J => match rotation & 0x1 {
                    1 => vec![0, 1],
                    _ => vec![0],
                },
                Block::L => match rotation & 0x1 {
                    1 => vec![0, 3],
                    _ => vec![0],
                },
                Block::S => match rotation {
                    1 => vec![0, 1],
                    _ => vec![0],
                },
                Block::T => match rotation & 0x1 {
                    1 => vec![0, 1, 3],
                    _ => vec![0],
                },
                Block::Z => match rotation {
                    1 => vec![0, 1],
                    _ => vec![0],
                },
            }
        }
        let mut list: Vec<Vec<Vec<Action>>> = Vec::new();
        let block_list: Vec<Block> = "IJLOSTZ"
            .as_bytes()
            .iter()
            .map(|&x| Block::try_from(x as char).unwrap())
            .collect();
        for block1 in block_list.iter() {
            list.push(Vec::new());
            let mut list1 = Vec::new();
            for r in rotations(block1) {
                for t in translations(block1, r) {
                    for s in spins(block1, r) {
                        list1.push(Action {
                            hold: false,
                            rotation: r,
                            translation: t,
                            spin: s,
                        });
                    }
                }
            }
            for block2 in block_list.iter() {
                let mut list2 = list1.clone();
                if block1 != block2 {
                    for r in rotations(block2) {
                        for t in translations(block2, r) {
                            for s in spins(block2, r) {
                                list2.push(Action {
                                    hold: true,
                                    rotation: r,
                                    translation: t,
                                    spin: s,
                                });
                            }
                        }
                    }
                }
                list.last_mut().unwrap().push(list2);
            }
        }
        list
    };
}
