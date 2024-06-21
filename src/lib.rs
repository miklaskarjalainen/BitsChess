#![allow(clippy::inline_always)]

#![doc = include_str!("../README.md")]

mod bitschess;

mod board_helper;
pub use board_helper::*;

mod piece;
pub use piece::*;

mod chess_move;
pub use chess_move::*;

pub mod prelude {
    pub use super::board_helper::*;
    pub use super::bitschess::board::*;
    pub use super::bitschess::board::fen::*;
    pub use super::bitschess::bitboard::*;
    pub use super::chess_move::*;
    pub use super::piece::*;
}

