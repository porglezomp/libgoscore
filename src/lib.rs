#![no_std]

extern crate libc;

use libc::c_char;
pub mod ffi;


// Data Structures /////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub struct Stone(c_char);

#[derive(Debug)]
pub struct Board<'a> {
    stones: &'a mut [Stone],
    width: usize,
    height: usize,
}


// Core Functions //////////////////////////////////////////////////////////////

pub fn guess_dead_tiles(board: &mut Board) {
    for stone in board.stones.iter_mut() {
        if stone.present() {
            stone.set_dead(true);
        }
    }
}

pub fn score_tiles(_board: &mut Board) {
    unimplemented!();
}

pub fn score_sums(board: &Board, komi: u32) -> (u32, u32) {
    let mut black = 0;
    let mut white = komi;
    for tile in board.stones.iter() {
        match tile.score() {
            None => (),
            Some(Color::Black) => black += 2,
            Some(Color::White) => white += 2,
        }
    }
    (black, white)
}


// Bitflag Constants ///////////////////////////////////////////////////////////

pub const STONE_PRESENCE: c_char = 0x1;
pub const STONE_COLOR: c_char = 0x2;
pub const STONE_DEAD: c_char = 0x4;
pub const STONE_SCORE: c_char = 0x8;
pub const STONE_SCORE_COLOR: c_char = 0x10;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}


// Stone Accessors /////////////////////////////////////////////////////////////

impl Stone {
    pub fn present(&self) -> bool {
        (self.0 & STONE_PRESENCE) != 0
    }

    pub fn set_present(&mut self, present: bool) {
        if present {
            self.0 |= STONE_PRESENCE;
        } else {
            self.0 &= !STONE_PRESENCE;
        }
    }

    pub fn dead(&self) -> bool {
        (self.0 & STONE_DEAD) != 0
    }

    pub fn set_dead(&mut self, dead: bool) {
        if dead {
            self.0 |= STONE_DEAD;
        } else {
            self.0 &= !STONE_DEAD;
        }
    }

    pub fn color(&self) -> Option<Color> {
        if self.present() {
            if (self.0 & STONE_COLOR) == 0 {
                Some(Color::Black)
            } else {
                Some(Color::White)
            }
        } else {
            None
        }
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        match color {
            None => self.0 &= !STONE_PRESENCE,
            Some(Color::Black) => {
                self.0 |= STONE_PRESENCE;
                self.0 &= !STONE_COLOR;
            }
            Some(Color::White) => {
                self.0 |= STONE_PRESENCE | STONE_COLOR;
            }
        }
    }

    pub fn score(&self) -> Option<Color> {
        if (self.0 & STONE_SCORE) != 0 {
            if (self.0 & STONE_SCORE_COLOR) == 0 {
                Some(Color::Black)
            } else {
                Some(Color::White)
            }
        } else {
            None
        }
    }

    pub fn set_score(&mut self, color: Option<Color>) {
        match color {
            None => self.0 &= !STONE_SCORE,
            Some(Color::Black) => {
                self.0 |= STONE_SCORE;
                self.0 &= !STONE_SCORE_COLOR;
            }
            Some(Color::White) => {
                self.0 |= STONE_SCORE | STONE_SCORE_COLOR;
            }
        }
    }
}


// Tests ///////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_accessors() {
        let mut stone = Stone(0);
        assert!(!stone.present());
        assert!(!stone.dead());
        assert_eq!(stone.color(), None);
        assert_eq!(stone.score(), None);

        let colors = [None, Some(Color::Black), Some(Color::White)];
        let bools = [false, true];
        for &color in &colors {
            for &score in &colors {
                for &present in &bools {
                    for &dead in &bools {
                        stone.set_color(color);
                        stone.set_score(score);
                        stone.set_dead(dead);

                        assert_eq!(stone.color(), color);
                        assert_eq!(stone.score(), score);
                        assert_eq!(stone.dead(), dead);

                        assert_eq!(stone.present(), color.is_some());

                        stone.set_present(present);
                        assert_eq!(stone.present(), present);

                        if present && color.is_some() {
                            assert_eq!(stone.color(), color);
                        }

                        if !present {
                            assert_eq!(stone.color(), None);
                        }
                    }
                }
            }
        }
    }
}
