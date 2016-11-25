#![no_std]

extern crate libc;

use libc::c_char;
pub mod ffi;


// Data Structures /////////////////////////////////////////////////////////////

/// A wrapper around a `c_char` to enable accessors and setters for the
/// bitflags. A `Stone` contains information on whether a given point contains a
/// stone, whether it's dead, and what the value of the position is.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Stone(c_char);

/// A struct that represents a Go board.
#[derive(Debug)]
pub struct Board<'a> {
    stones: &'a mut [Stone],
    width: usize,
    height: usize,
}


// Core Functions //////////////////////////////////////////////////////////////

/// Flags all the stones that it believes are dead.
pub fn guess_dead_stones(_board: &mut Board) {
}

/// Assigns scores to each stone, taking into account which stones are dead. A
/// stone is worth a point for a given player if it's occupied by that player's
/// living stone, or if it cannot reach a living stone of the opposing player.
pub fn score_stones(_board: &mut Board) {
    unimplemented!();
}

/// Computes the sum of all the scores on the board, accounting for komi. Komi
/// is the handicap and tie-breaker in Go. White always gets at least 1/2 point,
/// the standard Japanese komi is 6.5 points, most other countries use this
/// value as well. `komi` is a point value in half-points, and scores are
/// returned in half-points.
pub fn score_sums(board: &Board, komi: u32) -> (u32, u32) {
    let mut black = 0;
    let mut white = komi;
    for stone in board.stones.iter() {
        match stone.score() {
            None => (),
            Some(Color::Black) => black += 2,
            Some(Color::White) => white += 2,
        }
    }
    (black, white)
}


// Bitflags ////////////////////////////////////////////////////////////////////

/// This bit is set if the space contains a stone.
const STONE_PRESENCE: c_char = 0x1;
/// This bit is set if the stone is white, and unset if it's black. If
/// `STONE_PRESENCE` is not set, then the value is unspecified.
const STONE_COLOR: c_char = 0x2;
/// This bit is set if the stone in the given space is dead, and unset if it's
/// not. If `STONE_PRESENCE` is not set, then the value is unspecified.
const STONE_DEAD: c_char = 0x4;
/// This bit is set if the given space is worth a point, and unset if it's not
/// worth any points.
const STONE_SCORE: c_char = 0x8;
/// This bit is set if the given space is worth a point for white, and unset if
/// it's worth a point for black. If `STONE_SCORE` is unset, then the value of
/// this bit is unspecified.
const STONE_SCORE_COLOR: c_char = 0x10;


// Stone Accessors /////////////////////////////////////////////////////////////

/// Which player's stone.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Stone {
    /// Creates an empty point.
    pub fn new() -> Stone {
        Stone(0)
    }

    /// Determines whether a stone is actually present at this point.
    pub fn is_present(&self) -> bool {
        self.present_bit()
    }

    /// Makes a stone present or not present. This leaves the color and score
    /// the same as it was, but unobservable.
    pub fn set_present(&mut self, present: bool) {
        self.set_present_bit(present);
    }

    /// Determines whether or not the given stone is dead. If the stone is not
    /// present, it will not return as alive independent of what the bit says.
    pub fn is_dead(&self) -> bool {
        self.is_present() && self.dead_bit()
    }

    /// Can label a stone as alive or dead. This sets the underlying bit whether
    /// or not there is currently a stone at that point.
    pub fn set_dead(&mut self, dead: bool) {
        self.set_dead_bit(dead);
    }

    /// Returns the color of the stone, or `None` if there is no stone present.
    pub fn color(&self) -> Option<Color> {
        if self.is_present() {
            if self.color_bit() {
                Some(Color::White)
            } else {
                Some(Color::Black)
            }
        } else {
            None
        }
    }

    /// Allows you to set the color of the stone if one is present. If a stone
    /// is not present, then passing `Some(color)` will make the stone present.
    /// Passing `None` will set the stone to not present.
    pub fn set_color(&mut self, color: Option<Color>) {
        match color {
            None => self.set_present_bit(false),
            Some(Color::Black) => {
                self.set_present_bit(true);
                self.set_color_bit(false);
            }
            Some(Color::White) => {
                self.set_present_bit(true);
                self.set_color_bit(true);
            }
        }
    }

    /// Tells who the point is worth points for. If the spot is not worth points
    /// for either player, then return `None`.
    pub fn score(&self) -> Option<Color> {
        if self.score_bit() {
            if self.score_color_bit() {
                Some(Color::White)
            } else {
                Some(Color::Black)
            }
        } else {
            None
        }
    }

    /// Sets the score for the point.
    pub fn set_score(&mut self, color: Option<Color>) {
        match color {
            None => self.set_score_bit(false),
            Some(Color::Black) => {
                self.set_score_bit(true);
                self.set_score_color_bit(false);
            }
            Some(Color::White) => {
                self.set_score_bit(true);
                self.set_score_color_bit(true);
            }
        }
    }
}


// Bitflag Accessors ///////////////////////////////////////////////////////////

macro_rules! bitflag_getter_setter {
    ($BIT_CONST:ident , $get_name:ident , $set_name:ident) => {
        /// Gets the bit
        pub fn $get_name(&self) -> bool {
            (self.0 & $BIT_CONST) != 0
        }

        /// Sets the bit
        pub fn $set_name(&mut self, bit: bool) {
            if bit {
                self.0 |= $BIT_CONST;
            } else {
                self.0 &= !$BIT_CONST;
            }
        }
    }
}

impl Stone {
    bitflag_getter_setter!(STONE_PRESENCE, present_bit, set_present_bit);
    bitflag_getter_setter!(STONE_DEAD, dead_bit, set_dead_bit);
    bitflag_getter_setter!(STONE_COLOR, color_bit, set_color_bit);
    bitflag_getter_setter!(STONE_SCORE, score_bit, set_score_bit);
    bitflag_getter_setter!(STONE_SCORE_COLOR, score_color_bit, set_score_color_bit);
}



// Tests ///////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_accessors() {
        let mut stone = Stone(0);
        assert!(!stone.is_present());
        assert!(!stone.is_dead());
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
                        assert_eq!(stone.is_dead(), dead && stone.is_present());

                        assert_eq!(stone.is_present(), color.is_some());

                        stone.set_present(present);
                        assert_eq!(stone.is_present(), present);

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
