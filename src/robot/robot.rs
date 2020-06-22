use super::grid::{Grid, GridBombs, GridHoles};
use super::Action;
use super::Block;
use crate::config::Config;

use std::collections::VecDeque;

pub type RobotBombs = Robot<GridBombs>;
pub type RobotHoles = Robot<GridHoles>;

#[derive(Clone, Debug)]
pub struct Robot<T> {
    b2b: bool,
    block: Block,
    combo: u8,
    config: &'static Config,
    grid: T,
    hold: Option<Block>,
    queue: VecDeque<Block>,
    sent: i32,
}

impl<T> Robot<T>
where
    T: Grid,
{
    pub fn new(blocks: &[Block], config: &'static Config) -> Self {
        Self {
            b2b: false,
            block: blocks[0],
            combo: 0,
            config,
            grid: T::new(),
            hold: None,
            queue: VecDeque::from(blocks[1..].to_vec()),
            sent: 0,
        }
    }

    pub fn play(&mut self, action: Action) {
        if action.hold {
            self.do_hold();
        }
        let (lines, spin) = self.grid.drop_block(
            self.block,
            action.rotation,
            (action.translation + 3) as usize,
            action.spin,
        );
        if lines > 0 {
            if self.grid.is_empty() {
                self.sent += 10;
                if spin || (lines >= 4 && self.config.b2b_quadruple) {
                    self.b2b = true
                } else {
                    self.b2b = false
                }
            } else if spin && (self.block == Block::T || self.config.all_spin) {
                self.sent += 2 * lines;
                if self.b2b {
                    self.sent += 1;
                } else {
                    self.b2b = true;
                }
            } else if lines >= 4 {
                self.sent += 4;
                if self.config.b2b_quadruple {
                    if self.b2b {
                        self.sent += 1;
                    } else {
                        self.b2b = true;
                    }
                } else {
                    self.b2b = false
                }
            } else {
                self.b2b = false;
                self.sent += lines - 1;
            }
            self.sent += Self::combo_bonus(self.config, self.combo);
            self.combo += 1;
        } else {
            self.combo = 0;
        }
        self.block = self.queue.pop_front().unwrap();
    }

    pub fn next_action(&self) -> Action {
        let depth = self.config.search_depth;
        let breadth = self.config.search_breadth as usize;
        let action_list = self.get_action_list();
        let mut tuples = {
            let mut tuples: Vec<(Action, Self, i32)> = Vec::with_capacity(256);
            for &action in action_list {
                let mut clone = self.clone();
                clone.play(action);
                let score = clone.calc_score();
                tuples.push((action, clone, score));
            }
            tuples.sort_unstable_by_key(|x| -x.2);
            tuples
        };
        for _ in 1..depth {
            let mut new_tuples: Vec<(Action, Self, i32)> = Vec::with_capacity(256 * breadth);
            for (j, pair) in tuples.iter().enumerate() {
                if j >= breadth {
                    break;
                }
                let robot = &pair.1;
                let action_list = robot.get_action_list();
                for &action in action_list {
                    let mut clone = robot.clone();
                    clone.play(action);
                    let score = clone.calc_score();
                    new_tuples.push((pair.0, clone, score));
                }
            }
            tuples = new_tuples;
            tuples.sort_unstable_by_key(|x| -x.2);
        }
        tuples[0].0
    }

    pub fn add_handicap(&mut self, lines: &[u8]) {
        self.grid.add_handicap(lines);
    }

    pub fn add_block(&mut self, block: Block) {
        self.queue.push_back(block);
    }

    pub fn ko(&mut self) {
        self.grid.ko();
    }

    fn calc_score(&self) -> i32 {
        16 * self.sent + self.grid.calc_score()
    }

    fn do_hold(&mut self) {
        match self.hold {
            None => {
                self.hold = Some(self.block);
                self.block = self.queue.pop_front().unwrap();
            }
            Some(x) => {
                self.hold = Some(self.block);
                self.block = x;
            }
        };
    }

    fn get_action_list(&self) -> &[Action] {
        match self.hold {
            None => Action::get_list(self.block, self.queue[0]),
            Some(block) => Action::get_list(self.block, block),
        }
    }

    fn combo_bonus(config: &Config, combo: u8) -> i32 {
        let combo = combo as usize;
        if combo >= config.combo_table.len() {
            config.combo_table[config.combo_table.len() - 1]
        } else {
            config.combo_table[combo]
        }
    }
}
