#![warn(missing_docs)]

extern crate libc;

use libc::c_char;

/// Contains bindings intended to be called from C
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

/// Which player's stone.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    #[allow(missing_docs)]
    Black,
    #[allow(missing_docs)]
    White,
}


// Core Functions //////////////////////////////////////////////////////////////

/// Flags all the stones that it believes are dead.
pub fn guess_dead_stones(_board: &mut Board) {
}

/// Assigns scores to each stone, taking into account which stones are dead. A
/// stone is worth a point for a given player if it's occupied by that player's
/// living stone, or if it cannot reach a living stone of the opposing player.
pub fn score_stones(board: &mut Board) {
    // Reset all the scores
    for stone in board.stones.iter_mut() {
        stone.set_score(None);
        stone.set_processed_bit(false);
    }

    for stone in board.stones.iter_mut() {
        if stone.is_present() && !stone.is_dead() {
            let color = stone.color();
            stone.set_score(color);
            stone.set_processed_bit(true);
        }
    }

    let mut queue = Vec::new();
    let mut seen = std::collections::HashSet::<usize>::new();
    for i in 0..(board.width * board.height) {
        if !board.stones[i].processed_bit() {
            queue.clear();
            seen.clear();
            queue.push(i);
            let mut boundary_color = None::<Color>;
            let mut both = false;
            while let Some(i) = queue.pop() {
                seen.insert(i);
                let neighbors = board.neighbors(i);
                for neighbor in neighbors {
                    if seen.contains(&neighbor) {
                        continue;
                    }

                    if board.stones[neighbor].is_dead() {
                        queue.push(neighbor);
                        continue;
                    }

                    if let Some(color) = board.stones[neighbor].color() {
                        if both {
                            continue;
                        }
                        match boundary_color {
                            Some(c) if c != color => both = true,
                            Some(_) => (),
                            None => boundary_color = Some(color),
                        }
                    } else {
                        queue.push(neighbor);
                    }
                }
            }
            for &idx in &seen {
                if !both {
                    board.stones[idx].set_score(boundary_color);
                }
                board.stones[idx].set_processed_bit(true);
            }
        }
    }

    for stone in board.stones.iter_mut() {
        stone.set_processed_bit(false);
    }
}

/// Computes the sum of all the scores on the board, accounting for komi. Komi
/// is the handicap and tie-breaker in Go. White always gets at least 1/2 point,
/// the standard Japanese komi is 6.5 points, most other countries use this
/// value as well. For the purpose of this function, pass `komi` rounded down
/// (so the standard 6.5 should be given as 6).
///
/// Values are returned as `(black, white, winner)`.
/// When `black == white`, white wins due to the implicit half-point advantage.
pub fn score_sums(board: &Board, komi: u32) -> (u32, u32, Color) {
    let mut black = 0;
    let mut white = komi;
    for stone in board.stones.iter() {
        match stone.score() {
            None => (),
            Some(Color::Black) => black += 1,
            Some(Color::White) => white += 1,
        }
    }
    let winner = if black > white {
        Color::Black
    } else {
        Color::White
    };
    (black, white, winner)
}


// Board Operations ////////////////////////////////////////////////////////////

impl<'a> Board<'a> {
    pub fn neighbors(&self, i: usize) -> Vec<usize> {
        let mut neighborhood = Vec::with_capacity(4);
        if i / self.width != 0 {
            neighborhood.push(i - self.width);
        }
        if i % self.width != self.width - 1 {
            neighborhood.push(i + 1);
        }
        if i / self.width != self.height - 1 {
            neighborhood.push(i + self.width);
        }
        if i % self.width != 0 {
            neighborhood.push(i - 1);
        }
        neighborhood
    }
}


// Stone Accessors /////////////////////////////////////////////////////////////

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
/// This bit is internal to the algorithms. It is set once it has been processed.
const STONE_PROCESSED: c_char = 0x20;


// Bitflag Accessors ///////////////////////////////////////////////////////////

macro_rules! bitflag_getter_setter {
    ($BIT_CONST:ident , $get_name:ident , $set_name:ident) => {
        #[allow(missing_docs)]
        pub fn $get_name(&self) -> bool {
            (self.0 & $BIT_CONST) != 0
        }

        #[allow(missing_docs)]
        pub fn $set_name(&mut self, bit: bool) {
            if bit {
                self.0 |= $BIT_CONST;
            } else {
                self.0 &= !$BIT_CONST;
            }
        }
    }
}

/// Raw accessors to the underlying bitflags.
impl Stone {
    bitflag_getter_setter!(STONE_PRESENCE, present_bit, set_present_bit);
    bitflag_getter_setter!(STONE_DEAD, dead_bit, set_dead_bit);
    bitflag_getter_setter!(STONE_COLOR, color_bit, set_color_bit);
    bitflag_getter_setter!(STONE_SCORE, score_bit, set_score_bit);
    bitflag_getter_setter!(STONE_SCORE_COLOR, score_color_bit, set_score_color_bit);
    bitflag_getter_setter!(STONE_PROCESSED, processed_bit, set_processed_bit);
}


// Tests ///////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflag_accessors() {
        let mut stone = Stone::new();
        assert!(!stone.present_bit());
        assert!(!stone.dead_bit());
        assert!(!stone.color_bit());
        assert!(!stone.score_bit());
        assert!(!stone.score_color_bit());

        let bools = [false, true];
        for &color in &bools {
            for &score in &bools {
                for &present in &bools {
                    for &dead in &bools {
                        for &score_color in &bools {
                            stone.set_present_bit(present);
                            stone.set_dead_bit(dead);
                            stone.set_color_bit(color);
                            stone.set_score_bit(score);
                            stone.set_score_color_bit(score_color);

                            assert_eq!(stone.present_bit(), present);
                            assert_eq!(stone.dead_bit(), dead);
                            assert_eq!(stone.color_bit(), color);
                            assert_eq!(stone.score_bit(), score);
                            assert_eq!(stone.score_color_bit(), score_color);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_stone_accessors() {
        let mut stone = Stone::new();
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

    #[test]
    fn test_neighborhood() {
        let mut stones = [Stone::new()];
        let board = Board {
            stones: &mut stones,
            width: 1,
            height: 1,
        };
        assert_eq!(board.neighbors(0), vec![]);

        let mut stones = [Stone::new(); 4];
        let board = Board {
            stones: &mut stones,
            width: 2,
            height: 2,
        };
        assert_eq!(board.neighbors(0), vec![1, 2]);

        let mut stones = [Stone::new(); 9];
        let board = Board {
            stones: &mut stones,
            width: 3,
            height: 3,
        };
        assert_eq!(board.neighbors(4), vec![1, 5, 7, 3]);
    }
}
