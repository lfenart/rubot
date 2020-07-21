mod grid_bombs;
mod grid_holes;

pub use grid_bombs::GridBombs;
pub use grid_holes::GridHoles;

use super::Block;

pub trait Grid: Clone + std::fmt::Debug {
    fn new() -> Self;

    fn add_handicap(&mut self, lines: &[u8]);

    fn calc_score(&self) -> i32;

    fn drop_block(&mut self, block: Block, rotation: i8, x: usize, spin: i8) -> (i32, bool);

    fn is_empty(&self) -> bool;

    fn ko(&mut self);
}
