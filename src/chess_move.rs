use crate::bitschess::board_helper::BoardHelper;
use crate::piece::Piece;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
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
    pub const fn from_u8(f: u8) -> MoveFlag {
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
    pub const fn eq_const(self, other: MoveFlag) -> bool {
        self.to_u8() == other.to_u8()
    }
}


/// Move is represented with 16 bits (0bFFFFDDDDDDSSSSSS)
/// Where the first 0..=5 bits represent the square a piece is moving from (source/from),
/// 6..=11 bits represent the square the piece is moving to (destination/to) and
/// 12..=15 bits represent the flags of the move.  
#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn new(board_move: Move, captured: Piece, en_passant_square: i32, castling: [bool; 4], half_move: u8, zobrist_hash: u64, repetition_saved: bool) -> Self { 
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
