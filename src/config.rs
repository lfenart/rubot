use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub all_spin: bool,
    pub b2b_bonus: i32,
    pub b2b_quadruple: bool,
    pub combo_table: Vec<i32>,
    pub search_breadth: usize,
    pub search_depth: u8,
}
