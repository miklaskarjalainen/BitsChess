#![allow(clippy::inline_always)]

#![doc = include_str!("../README.md")]

mod bitschess;
mod piece;
mod chess_move;
mod board_helper;
pub mod prelude {
    pub use super::board_helper::*;
    
    pub use super::bitschess::board::*;
    pub use super::bitschess::bitboard::*;
    pub use super::bitschess::board::fen::*;
    
    pub use super::chess_move::*;
    
    pub use super::piece::*;
}

