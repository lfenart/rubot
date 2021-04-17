use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub all_spin: bool,
    pub b2b_bonus: i32,
    pub b2b_quadruple: bool,
    pub combo_table: Vec<i32>,
    pub search_breadth: usize,
    pub search_depth: u8,
}

impl Config {
    pub fn wwc_bombs() -> Self {
        Self {
            all_spin: false,
            b2b_bonus: 1,
            b2b_quadruple: true,
            combo_table: vec![0, 0, 1, 2],
            search_breadth: 1024,
            search_depth: 4,
        }
    }

    pub fn wwc_chaos() -> Self {
        Self {
            all_spin: true,
            b2b_bonus: 1,
            b2b_quadruple: false,
            combo_table: vec![0, 0, 1],
            search_breadth: 1024,
            search_depth: 4,
        }
    }

    pub fn wwc_holes() -> Self {
        Self {
            all_spin: false,
            b2b_bonus: 1,
            b2b_quadruple: true,
            combo_table: vec![0, 0, 1, 2],
            search_breadth: 1024,
            search_depth: 4,
        }
    }
}
