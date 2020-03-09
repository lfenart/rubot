use crate::robot::Block;

pub trait Grid: Clone + std::fmt::Debug {
    fn new() -> Self;

    fn add_handicap(&mut self, lines: &[u8]);

    fn calc_score(&self) -> i32;

    fn drop_block(&mut self, block: Block, rotation: u8, x: usize, spin: u8) -> (i32, bool);

    fn is_empty(&self) -> bool;

    fn ko(&mut self);
}
