use libc::{c_char, size_t, c_int};
use core::{self, slice};
use super::{Board, Stone};

#[no_mangle]
pub extern fn guess_dead_tiles(data: *mut c_char, width: size_t, height: size_t) {
    let mut board = Board {
        stones: unsafe { slice::from_raw_parts_mut(data as *mut Stone, (width * height) as usize) },
        width: width as usize,
        height: height as usize,
    };
    super::guess_dead_tiles(&mut board);
}

#[no_mangle]
pub extern fn score_tiles(data: *mut c_char, width: size_t, height: size_t) {
    let mut board = Board {
        stones: unsafe { slice::from_raw_parts_mut(data as *mut Stone, (width * height) as usize) },
        width: width as usize,
        height: height as usize,
    };
    super::score_tiles(&mut board);
}

#[no_mangle]
pub extern fn score_sums(data: *mut c_char, width: size_t, height: size_t,
                         komi: c_int, white: *mut c_int, black: *mut c_int) {
    let board = Board {
        stones: unsafe { slice::from_raw_parts_mut(data as *mut Stone, (width * height) as usize) },
        width: width as usize,
        height: height as usize,
    };
    let (w, b) = super::score_sums(&board, komi as u32);
    if white != core::ptr::null_mut() {
        unsafe { *white = w as c_int; }
    }
    if black != core::ptr::null_mut() {
        unsafe { *black = b as c_int; }
    }
}
