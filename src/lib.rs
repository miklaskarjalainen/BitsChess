#![allow(clippy::inline_always)]

#![doc = include_str!("../README.md")]
pub mod bitschess;

pub mod prelude {
    pub use super::bitschess::bitboard::*;
    pub use super::bitschess::board::*;
    pub use super::bitschess::board::fen::*;
    pub use super::bitschess::board_helper::*;
    pub use super::bitschess::chessmove::*;
    pub use super::bitschess::piece::*;
}

