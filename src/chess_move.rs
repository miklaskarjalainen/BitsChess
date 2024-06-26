#![allow(dead_code)]

use crate::board_helper::BoardHelper;
use crate::piece::Piece;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveFlag {
    None          = 0,
    EnPassant     = 1,
    PawnTwoUp     = 2,
    Castle        = 3,
    PromoteKnight = 4,
    PromoteBishop = 5,
    PromoteRook   = 6,
    PromoteQueen  = 7 
}

impl MoveFlag {
    #[must_use]
    #[inline(always)]
    pub const fn from_u8(f: u8) -> Self {
        unsafe {
            std::mem::transmute(f & 0b111)
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn eq_const(self, other: Self) -> bool {
        self.to_u8() == other.to_u8()
    }
}

/// # Move is represented with 16 bits
/// Where the first 0..=5 bits represent the square a piece is moving from (source/from),
/// 6..=11 bits represent the square the piece is moving to (destination/to) and
/// 12..=15 bits represent the flags of the move.  
/// 
///    111111  
///    5432109876543210  
/// (0bFFFFDDDDDDSSSSSS) -> S = source_square D = destination_square F = flag
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move(pub u16);

impl Move {
    /// Creates a new move.
    /// 
    /// * `from: i32` - Represents the square to move from, should be in range of (0..=63).
    /// * `to: i32`   - Represents the destination square to move to, should be in range of (0..=63).
    /// * `flag: MoveFlag` - Additional flags for the move, should be defaulted to [MoveFlag::None].
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Move, MoveFlag};
    /// let m = Move::new(12,28, MoveFlag::PawnTwoUp); // e2->e4
    /// assert_eq!(m.get_from_idx(), 12);
    /// assert_eq!(m.get_to_idx(), 28);
    /// assert_eq!(m.get_flag(), MoveFlag::PawnTwoUp);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn new(from: i32, to: i32, flag: MoveFlag) -> Self {
        let mut m = 0u16;
        m |= (from as u16) & 0b111111;
        m |= ((to as u16) & 0b111111) << 6;
        m |= (flag.to_u8() as u16) << 12;
        Self(m)
    }

    /// Returns the flag of the move.
    ///
    /// # Examples  
    /// ```rust
    /// use bitschess::{Move, MoveFlag};
    /// let m = Move::new(0,0, MoveFlag::Castle);
    /// assert_eq!(m.get_flag(), MoveFlag::Castle);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_flag(self) -> MoveFlag {
        let flags = (self.0 >> 12) & 0b111;
        MoveFlag::from_u8(flags as u8)
    }

    /// Returns the source square.
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Move, MoveFlag};
    /// let m = Move::new(12,28, MoveFlag::PawnTwoUp);
    /// assert_eq!(m.get_from_idx(), 12);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_from_idx(self) -> i32 {
        (self.0 & 0b111111) as i32
    }

    /// Returns the destination square.
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Move, MoveFlag};
    /// let m = Move::new(12,28, MoveFlag::PawnTwoUp);
    /// assert_eq!(m.get_to_idx(), 28);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_to_idx(self) -> i32 {
        ((self.0 >> 6) & 0b111111) as i32
    }

    /// Helper function to check if a move does en passant.
    /// # Examples  
    /// ```rust
    /// use bitschess::{Move, MoveFlag};
    /// let m1 = Move::new(12,28, MoveFlag::PawnTwoUp);
    /// let m2 = Move::new(27,20, MoveFlag::EnPassant);
    /// assert_eq!(m1.is_en_passant(), false);
    /// assert_eq!(m2.is_en_passant(), true);
    /// ```
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_en_passant(self) -> bool {
        self.get_flag().eq_const(MoveFlag::EnPassant)
    }
    
    /// Helper function to check if a move castles.
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_castle(self) -> bool {
        self.get_flag().eq_const(MoveFlag::Castle)
    }

    /// Helper function to check if a pawn moved 2 up.
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_two_pawn_up(self) -> bool {
        self.get_flag().eq_const(MoveFlag::PawnTwoUp)
    }


    /// Converts the move into a [UCI (Universal Chess Interface)](https://en.wikipedia.org/wiki/Universal_Chess_Interface)
    /// command.
    /// 
    /// # Returns
    /// * basic: "e2e4" a piece moved from e2 to e4
    /// * promotion: "e7e8q" last character tells us the promotion piece. See [Piece::to_char].
    #[must_use]
    pub fn to_uci(self) -> String {
        let (frank, ffile) = BoardHelper::square_to_chars(self.get_from_idx());
        let (trank, tfile) = BoardHelper::square_to_chars(self.get_to_idx());
        
        let mut str = format!("{frank}{ffile}{trank}{tfile}");

        match self.get_flag() {
            MoveFlag::PromoteQueen => { str.push('q'); }
            MoveFlag::PromoteRook => { str.push('r'); }
            MoveFlag::PromoteBishop => { str.push('b'); }
            MoveFlag::PromoteKnight => { str.push('n'); }
            _ => {}
        }

        str
    }
    
    /// Converts the move into a [UCI (Universal Chess Interface)](https://en.wikipedia.org/wiki/Universal_Chess_Interface)
    /// command.
    /// 
    /// # Valid Inputs
    /// * basic: "e2e4" a piece moved from e2 to e4
    /// * promotion: "e7e8q" last character tells us the promotion piece. See [Piece::from_char].
    /// 
    /// # Examples
    /// ```
    /// use bitschess::{Move, MoveFlag};
    /// let m1 = Move::from_uci("e2e4");
    /// let m2 = Move::from_uci("e7e8q");
    /// assert_eq!(m1.get_from_idx(), 12);
    /// assert_eq!(m1.get_to_idx(), 28);
    /// assert_eq!(m2.get_from_idx(), 52);
    /// assert_eq!(m2.get_to_idx(), 60);
    /// assert_eq!(m2.get_flag(), MoveFlag::PromoteQueen);
    /// ```
    /// 
    /// # Panics
    /// If `uci`'s length is not in range of 4..=5
    /// 
    #[must_use]
    #[allow(dead_code)]
    pub const fn from_uci(uci: &str) -> Self {
        assert!(uci.len() >= 4);

        let bytes = uci.as_bytes();
        let from = BoardHelper::chars_to_square(bytes[0] as char, bytes[1] as char);
        let to = BoardHelper::chars_to_square(bytes[2] as char, bytes[3] as char);

        // flags
        let mut flag = MoveFlag::None;
        if uci.len() > 4 {
            match bytes[4] as char {
                // Promotions
                'n' => {
                    flag = MoveFlag::PromoteKnight;
                }

                'b' => {
                    flag = MoveFlag::PromoteBishop;
                }

                'r' => {
                    flag = MoveFlag::PromoteRook;
                }

                'q' => {
                    flag = MoveFlag::PromoteQueen;
                }

                _ => {}
            }
        }

        Self::new(from, to, flag)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReversibleMove {
    pub board_move: Move,
    pub captured: Piece,

    // board states
    pub zobrist_hash: u64,
    pub en_passant_square: i32,
    pub castling: [bool; 4],
    pub half_move: u8,
    pub repetition_saved: bool,
}

impl ReversibleMove {
    #[must_use]
    pub const fn new(board_move: Move, captured: Piece, en_passant_square: i32, castling: [bool; 4], half_move: u8, zobrist_hash: u64, repetition_saved: bool) -> Self { 
        Self {
            board_move, 
            captured,
            en_passant_square,
            castling,
            half_move,
            zobrist_hash,
            repetition_saved
        }
    }
}

pub struct MoveContainer {
    // Most legal moves in a chess position is 218 in this position:
    // fen: R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - -
    moves: [Move; 218], 
    size: usize 
}

pub struct MoveContainerIterator<'a> {
    container: &'a MoveContainer,
    index: usize
}

impl<'a> Iterator for MoveContainerIterator<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.container.len() {
            let result = Some(&self.container.moves[self.index]);
            self.index += 1;
            return result;
        }
        None
    }
}

pub struct MoveContainerIntoIterator {
    container: MoveContainer,
    index: usize
}

impl Iterator for MoveContainerIntoIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.container.len() {
            let result = Some(self.container.moves[self.index]);
            self.index += 1;
            return result;
        }
        None
    }
}

impl IntoIterator for MoveContainer {
    type Item = Move;
    type IntoIter = MoveContainerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            container: self,
            index: 0
        }
    }
}

impl FromIterator<Move> for MoveContainer {
    fn from_iter<I: IntoIterator<Item=Move>>(iter: I) -> Self {
        let mut c = MoveContainer::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl MoveContainer {
    #[inline(always)]
    pub fn new() -> Self {
        MoveContainer {
            moves: [Move(0); 218],
            size: 0
        }
    }
        
    #[inline(always)]
    pub fn iter(&self) -> MoveContainerIterator {
        MoveContainerIterator {
            container: self,
            index: 0
        }
    }
    
    #[inline(always)]
    pub fn get(&self, i: usize) -> Option<Move> {
        if i < self.size {
            return Some( unsafe { self.get_unchecked(i) } );
        }
        None
    }

    #[inline(always)]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut Move> {
        if i < self.size {
            return Some(unsafe { self.get_unchecked_mut(i) });
        }
        None
    }
    
    // For some it might be a bit silly not to return a reference here.
    // But remember, Move is only 16bits so using an address would take 32 or 64 bits so... more.
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, i: usize) -> Move {
        unsafe {
            *self.moves.get_unchecked(i)
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> &mut Move {
        unsafe {
            self.moves.get_unchecked_mut(i)
        }
    }

    #[inline(always)]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.moves.swap(a, b);
    }

    #[inline(always)]
    pub unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        unsafe {
            // Xor swapping most likely will not improve performance since MOV instructions are supposed to be "0-latency".
            let hold = self.get_unchecked(a);
            *self.get_unchecked_mut(a) = self.get_unchecked(b);
            *self.get_unchecked_mut(b) = hold;
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.size;
    }
    
    #[inline(always)]
    pub fn push(&mut self, chess_move: Move) {
        self.moves[self.size] = chess_move;
        self.size += 1;
    }
    
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_from_uci_basic() {
        let m = Move::from_uci("a2a4");
        assert_eq!(m.get_flag(), MoveFlag::None);
        assert_eq!(m.get_from_idx(), 8);
        assert_eq!(m.get_to_idx(), 24);
    }

    #[test]
    fn test_move_from_uci_promotion_queen_flag() {
        let m = Move::from_uci("e7e8q");
        assert_eq!(m.get_from_idx(), 52);
        assert_eq!(m.get_to_idx(), 60);
        assert_eq!(m.get_flag(), MoveFlag::PromoteQueen);
    }

    #[test]
    fn test_move_from_uci_promotion_rook_flag() {
        let m = Move::from_uci("e7e8r");
        assert_eq!(m.get_flag(), MoveFlag::PromoteRook);
    }

    #[test]
    fn test_move_from_uci_promotion_bishop_flag() {
        let m = Move::from_uci("e7e8b");
        assert_eq!(m.get_flag(), MoveFlag::PromoteBishop);
    }

    #[test]
    fn test_move_from_uci_promotion_knight_flag() {
        let m = Move::from_uci("e7e8n");
        assert_eq!(m.get_flag(), MoveFlag::PromoteKnight);
    }
}
