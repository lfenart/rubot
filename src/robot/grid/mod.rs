mod grid_bombs;
mod grid_holes;

pub use grid_bombs::GridBombs;
pub use grid_holes::GridHoles;

use super::Block;

pub const MAIN_SIZE: usize = 32;
pub const FULL_SIZE: usize = 64;
pub const EMPTY_LINE: u16 = 0xe007;
pub const FULL_LINE: u16 = 0xffff;

pub trait Grid: Clone + std::fmt::Debug {
    fn new(lines: [u16; FULL_SIZE], handicap: usize) -> Self;

    fn empty() -> Self;

    fn add_handicap(&mut self, lines: &[u8]);

    fn calc_score(&self) -> i32;

    fn drop_block(&mut self, block: Block, rotation: i8, x: usize, spin: i8) -> (i32, bool);

    fn is_empty(&self) -> bool;

    fn ko(&mut self);
}
