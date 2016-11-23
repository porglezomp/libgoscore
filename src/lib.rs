#![no_std]

extern crate libc;

use libc::{c_char, size_t};
use core::slice;

#[derive(Debug)]
struct Stone(c_char);

#[derive(Debug)]
struct Board<'a> {
    stones: &'a mut [Stone],
    width: usize,
    height: usize,
}

#[no_mangle]
pub extern fn guess_dead_tiles(data: *mut c_char, width: size_t, height: size_t) {
    let mut board = Board {
        stones: unsafe { slice::from_raw_parts_mut(data as *mut Stone, (width * height) as usize) },
        width: width as usize,
        height: height as usize,
    };
    rust_guess_dead_tiles(&mut board);
}

#[no_mangle]
pub extern fn score_tiles(data: *mut c_char, width: size_t, height: size_t) {
    let mut board = Board {
        stones: unsafe { slice::from_raw_parts_mut(data as *mut Stone, (width * height) as usize) },
        width: width as usize,
        height: height as usize,
    };
    rust_score_tiles(&mut board);
}

fn rust_guess_dead_tiles(board: &mut Board) {
    for stone in board.stones.iter_mut() {
        if (stone.0 & STONE_PRESENCE) != 0 {
            stone.0 |= STONE_DEAD;
        }
    }
}

fn rust_score_tiles(_board: &mut Board) {
    unimplemented!();
}

const STONE_PRESENCE: c_char = 0x1;
const STONE_COLOR: c_char = 0x2;
const STONE_DEAD: c_char = 0x4;
const STONE_SCORE: c_char = 0x8;
const STONE_SCORE_COLOR: c_char = 0x10;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

