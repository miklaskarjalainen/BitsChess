use super::board_helper::BoardHelper;
use super::piece::Piece;

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

// 0-5 bit from
// 6-11 bits to
// 12-15 Flags 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move(pub u16);

impl Move {
    #[must_use]
    #[inline(always)]
    pub const fn new(from: i32, to: i32, flag: MoveFlag) -> Self {
        let mut m = 0u16;
        m |= (from as u16) & 0b111111;
        m |= ((to as u16) & 0b111111) << 6;
        m |= (flag.to_u8() as u16) << 12;
        Self(m)
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_flag(self) -> MoveFlag {
        let flags = (self.0 >> 12) & 0b111;
        MoveFlag::from_u8(flags as u8)
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_from_idx(self) -> i32 {
        (self.0 & 0b111111) as i32
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_to_idx(self) -> i32 {
        ((self.0 >> 6) & 0b111111) as i32
    }

    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_en_passant(self) -> bool {
        self.get_flag().eq_const(MoveFlag::EnPassant)
    }
    
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_castle(self) -> bool {
        self.get_flag().eq_const(MoveFlag::Castle)
    }

    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_two_pawn_up(self) -> bool {
        self.get_flag().eq_const(MoveFlag::PawnTwoUp)
    }

    /// <https://en.wikipedia.org/wiki/Universal_Chess_Interface>
    /// Outputs: "e2e4", "e7e8q" 
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
    
    /// Correct inputs: "e2e4", "e7e8q" 
    /// <https://en.wikipedia.org/wiki/Universal_Chess_Interface>
    #[must_use]
    #[allow(dead_code)]
    pub fn from_uci(uci: &str) -> Self {
        assert!(uci.len() >= 4);

        let from = BoardHelper::text_to_square(&uci[0..2]);
        let to = BoardHelper::text_to_square(&uci[2..4]);

        // flags
        let mut flag = MoveFlag::None;
        if uci.len() > 4 {
            match uci.chars().nth(4) {
                // Promotions
                Some('n') => {
                    flag = MoveFlag::PromoteKnight;
                }

                Some('b') => {
                    flag = MoveFlag::PromoteBishop;
                }

                Some('r') => {
                    flag = MoveFlag::PromoteRook;
                }

                Some('q') => {
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
