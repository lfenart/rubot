use std::fmt;

use super::Grid;
use crate::robot::Block;

const MAIN_SIZE: usize = 32;
const FULL_SIZE: usize = 64;
const EMPTY_LINE: u16 = 0xe007;
const FULL_LINE: u16 = 0xffff;

#[derive(Clone)]
pub struct GridHoles {
    lines: [u16; FULL_SIZE],
    handicap: usize,
}

impl Grid for GridHoles {
    fn new() -> Self {
        Self {
            lines: [EMPTY_LINE; FULL_SIZE],
            handicap: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.get_height() == 0
    }

    fn calc_score(&self) -> i32 {
        let height = self.get_height() + self.handicap;
        let mut clean = 0u16;
        let mut n = 0;
        for i in (0..height).rev() {
            let index = MAIN_SIZE + self.handicap - i - 1;
            n += (clean & !self.lines[index]).count_ones();
            clean |= self.lines[index];
        }
        -((height * height * height) as i32) - (256 * n as i32)
    }

    fn add_handicap(&mut self, lines: &[u8]) {
        for line in lines {
            self.lines[MAIN_SIZE + self.handicap] = FULL_LINE ^ (1u16 << line);
            self.handicap += 1;
        }
    }

    fn drop_block(&mut self, block: Block, rotation: u8, x: usize, spin: u8) -> (i32, bool) {
        let y = {
            let value = block.value(rotation);
            let mut square = self.square(x, 0);
            let mut y = self.handicap;
            while (value & square) == 0 {
                square >>= 4;
                square |= (self.get_line(y + 4) << (12 - x)) & 0xf000;
                y += 1;
            }
            if y == 0 {
                return (0, false);
            }
            y - 1
        };
        if spin != 0 {
            self.do_spin(block, rotation, x, y, spin)
        } else {
            (self.put_block(block, rotation, x, y), false)
        }
    }

    fn ko(&mut self) {
        self.handicap = 0;
    }
}

impl GridHoles {
    fn get_line(&self, index: usize) -> u16 {
        if index >= MAIN_SIZE + self.handicap {
            FULL_LINE
        } else {
            self.lines[index]
        }
    }

    fn square(&self, x: usize, y: usize) -> u16 {
        let mut square = 0u16;
        for i in 0..4 {
            let index = y + i;
            square |= ((self.get_line(index) >> x) & 0xf) << (4 * i)
        }
        square
    }

    fn get_height(&self) -> usize {
        let mut i = 0;
        while self.lines[MAIN_SIZE - i - 1] != EMPTY_LINE {
            i += 1;
        }
        i
    }

    fn do_spin(&mut self, block: Block, rotation: u8, x: usize, y: usize, spin: u8) -> (i32, bool) {
        match self.do_kick(block, rotation, x, y, spin) {
            Some((new_x, new_y)) => self.drop_block_spin(block, rotation + spin, new_x, new_y),
            None => (self.put_block(block, rotation, x, y), false),
        }
    }

    fn do_kick(
        &mut self,
        block: Block,
        rotation: u8,
        x: usize,
        y: usize,
        spin: u8,
    ) -> Option<(usize, usize)> {
        let kick_list = Block::get_kick(rotation, spin);
        for i in 0..5 {
            let new_x = (x as isize + kick_list[i].0) as usize;
            let new_y = (y as isize - kick_list[i].1) as usize;
            if self.fit_block(block, rotation + spin, new_x, new_y) {
                return Some((new_x, new_y));
            }
        }
        None
    }

    fn drop_block_spin(&mut self, block: Block, rotation: u8, x: usize, y: usize) -> (i32, bool) {
        let new_y = {
            let mut new_y = y;
            let value = block.value(rotation);
            let mut square = self.square(x, y);
            while (value & square) == 0 {
                square >>= 4;
                square |= (self.get_line(new_y + 4) << (12 - x)) & 0xf000;
                new_y += 1;
            }
            new_y - 1
        };
        let is_spin = !self.fit_block(block, rotation, x, new_y - 1)
            && !self.fit_block(block, rotation, x - 1, new_y)
            && !self.fit_block(block, rotation, x + 1, new_y - 1);
        let lines = self.put_block(block, rotation, x, new_y);
        (lines, is_spin)
    }

    fn fit_block(&self, block: Block, rotation: u8, x: usize, y: usize) -> bool {
        (self.square(x, y) & block.value(rotation)) == 0
    }

    fn put_block(&mut self, block: Block, rotation: u8, x: usize, y: usize) -> i32 {
        let (n, mut lines, handicap_lines) = {
            let mut n = 0i32;
            let mut value = block.value(rotation);
            let mut lines: Vec<usize> = Vec::with_capacity(4);
            let mut handicap_lines: Vec<usize> = Vec::with_capacity(4);
            let max = (MAIN_SIZE + self.handicap - y).min(4);
            for i in 0..max {
                let index = i + y;
                self.lines[index] |= (value & 0xf) << x;
                if self.lines[index] == FULL_LINE {
                    if index < MAIN_SIZE {
                        lines.push(index);
                    } else {
                        handicap_lines.push(index);
                    }
                    n += 1;
                }
                value >>= 4;
            }
            (n, lines, handicap_lines)
        };
        if lines.len() != 0 {
            let mut delta = 0usize;
            let start = lines.pop().unwrap();
            let mut limit = lines.pop().unwrap_or(0);
            let mut i = start;
            while i > delta {
                while i == limit + delta + 1 {
                    delta += 1;
                    limit = lines.pop().unwrap_or(0);
                }
                if self.get_line(i) == EMPTY_LINE {
                    break;
                }
                self.lines[i] = self.lines[i - 1 - delta];
                i -= 1;
            }
        }
        if handicap_lines.len() != 0 {
            let delta = handicap_lines.len();
            let start = handicap_lines[0];
            for i in start..MAIN_SIZE + self.handicap - delta {
                self.lines[i] = self.lines[i + delta];
            }
            self.handicap -= delta;
        }
        n
    }
}

impl fmt::Debug for GridHoles {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.lines[..].fmt(formatter)
    }
}
