#![allow(clippy::inline_always)]

#![doc = include_str!("../README.md")]

mod bitschess;

mod piece;
pub use piece::*;

mod chess_move;
pub use chess_move::*;


mod board_helper;
pub use board_helper::*;

pub mod prelude {
    pub use super::bitschess::board::*;
    pub use super::bitschess::bitboard::*;
    pub use super::bitschess::board::fen::*;

    pub use super::piece::*;
}

