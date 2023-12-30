use lazy_static::lazy_static;

//https://www.chessprogramming.org/Bitboard_Board-Definition
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitBoard(u64);

pub const A_FILE: u64 = 0x0101_0101_0101_0101;
pub const B_FILE: u64 = A_FILE << 1;
pub const AB_FILE: u64 = A_FILE | B_FILE;
pub const NOT_A_FILE: u64 = !A_FILE; 
pub const NOT_AB_FILE: u64 = !AB_FILE;

pub const H_FILE: u64 = A_FILE << 7;
pub const G_FILE: u64 = A_FILE << 6;
pub const HG_FILE: u64 = H_FILE | G_FILE;
pub const NOT_H_FILE: u64 = !H_FILE;
pub const NOT_HG_FILE: u64 = !HG_FILE;

use super::piece::PieceColor;


lazy_static! {
    pub static ref PAWN_ATTACKS: [[u64; 64]; 2] = {
        let mut map = [[0; 64]; 2];
        for square in 0..64 {
            map[0][square] = BitBoard::get_pawn_attack(PieceColor::White, square as i32);
            map[1][square] = BitBoard::get_pawn_attack(PieceColor::Black, square as i32);
        }
        map
    };

    pub static ref KNIGHT_ATTACKS: [u64; 64] = {
        let mut map = [0; 64];
        for square in 0..64 {
            map[square] = BitBoard::get_knight_attack(square as i32);
        }
        map
    };

    pub static ref KING_ATTACKS: [u64; 64] = {
        let mut map = [0; 64];
        for square in 0..64 {
            map[square] = BitBoard::get_king_attack(square as i32);
        }
        map
    };
}

impl std::fmt::Display for BitBoard {

    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("");
        
        str.push_str("  a b c d e f g h\n");
        for y in (0..8).rev() {
            str.push_str((y+1).to_string().as_str());
            str.push(' ');
            for x in 0..8 {
                let bit = self.get_bit(y * 8 + x);
                str.push(if bit {'1'} else {'0'});
                str.push('|');
            }
            str.push('\n');
        }
        str.push_str("  a b c d e f g h\n");

        formatter.pad(str.as_str())
    }

}

impl BitBoard {
    #[inline(always)]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    #[inline(always)]
    pub const fn get_bit(&self, bit: i32) -> bool {
        return ((self.0 >> bit) & 0b1) == 1;
    }

    #[inline(always)]
    pub fn set_bit(&mut self, bit: i32) {
        self.0 |= 0b1 << bit;
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, bit: i32) {
        self.0 &= !(0b1 << bit);
    }

    #[inline(always)]
    pub fn toggle_bit(&mut self, bit: i32) {
        self.0 ^= 0b1 << bit;
    }

    #[inline(always)]
    pub fn set_bits(&mut self, bits: u64) {
        self.0 |= bits;
    }

    #[inline(always)]
    pub fn clear_bits(&mut self, bits: u64) {
        self.0 &= !bits;
    }

    #[inline(always)]
    pub fn toggle_bits(&mut self, bits: u64) {
        self.0 ^= bits;
    }

    #[inline(always)]
    pub fn get_masked(&self, mask: u64) -> u64 {
        return self.0 & mask;
    }

    #[inline(always)]
    pub fn get_bits(&self) -> u64 {
        return self.0;
    }

    fn get_pawn_attack(side: PieceColor, square: i32) -> u64 {
        // https://www.youtube.com/watch?v=OTWG4dERdSc&list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs&index=3&ab_channel=ChessProgramming
        let mut attacks = 0u64;
        let mut bitboard = BitBoard::new(0);
        bitboard.set_bit(square);
    
        if side == PieceColor::White {
            if ((bitboard.get_bits() << 7) & NOT_H_FILE) != 0 { 
                attacks |= bitboard.0 << 7;
            }
            if ((bitboard.get_bits() << 9) & NOT_A_FILE) != 0 { 
                attacks |= bitboard.0 << 9;
            }
        }
        else {
            if ((bitboard.get_bits() >> 7) & NOT_A_FILE) != 0 { 
                attacks |= bitboard.0 >> 7;
            }
            if ((bitboard.get_bits() >> 9) & NOT_H_FILE) != 0 { 
                attacks |= bitboard.0 >> 9;
            }
        }
    
        attacks
    }

    pub fn get_knight_attack(square: i32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = BitBoard::new(0);

        bitboard.set_bit(square);

        if bitboard.get_masked(NOT_H_FILE << 17) != 0 {
            attacks |= bitboard.get_bits() >> 17;
        }
        if bitboard.get_masked(NOT_A_FILE << 15) != 0 {
            attacks |= bitboard.get_bits() >> 15;
        }
        if bitboard.get_masked(NOT_HG_FILE << 10) != 0 {
            attacks |= bitboard.get_bits() >> 10;
        }
        if bitboard.get_masked(NOT_AB_FILE << 6) != 0 {
            attacks |= bitboard.get_bits() >> 6;
        }

        if bitboard.get_masked(NOT_A_FILE >> 17) != 0 {
            attacks |= bitboard.get_bits() << 17;
        }
        if bitboard.get_masked(NOT_H_FILE >> 15) != 0 {
            attacks |= bitboard.get_bits() << 15;
        }
        if bitboard.get_masked(NOT_AB_FILE >> 10) != 0 {
            attacks |= bitboard.get_bits() << 10;
        }
        if bitboard.get_masked(NOT_HG_FILE >> 6) != 0 {
            attacks |= bitboard.get_bits() << 6;
        }

        attacks
    }

    pub fn get_king_attack(square: i32) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = BitBoard::new(0);

        bitboard.set_bit(square);

        if (bitboard.get_bits() >> 8) != 0 {
            attacks |= bitboard.get_bits() >> 8;
        }
        if ((bitboard.get_bits() >> 7) & NOT_A_FILE) != 0 {
            attacks |= bitboard.get_bits() >> 7;
        }
        if ((bitboard.get_bits() >> 9) & NOT_H_FILE) != 0 {
            attacks |= bitboard.get_bits() >> 9;
        }
        if ((bitboard.get_bits() >> 1) & NOT_H_FILE) != 0 {
            attacks |= bitboard.get_bits() >> 1;
        }

        if (bitboard.get_bits() << 8) != 0 {
            attacks |= bitboard.get_bits() << 8;
        }
        if ((bitboard.get_bits() << 7) & NOT_H_FILE) != 0 {
            attacks |= bitboard.get_bits() << 7;
        }
        if ((bitboard.get_bits() << 9) & NOT_A_FILE) != 0 {
            attacks |= bitboard.get_bits() << 9;
        }
        if ((bitboard.get_bits() << 1) & NOT_A_FILE) != 0 {
            attacks |= bitboard.get_bits() << 1;
        }

        attacks
    }

    pub fn get_bishop_attack_mask(square: i32, blockers: u64) -> BitBoard {
        let tr = square / 8;
        let tf = square % 8;

        let mut attacks = 0;

        for (r, f) in ((tr + 1)..8).zip((tf + 1)..8) {
            attacks |= 1u64 << (r * 8 + f);
            if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
        }
        for (r, f) in (0..tr).rev().zip((tf+1)..8) {
            attacks |= 1u64 << (r * 8 + f);
            if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
        }
        for (r, f) in ((tr + 1)..8).zip((0..tf).rev()) {
            attacks |= 1u64 << (r * 8 + f);
            if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
        }
        for (r, f) in (0..tr).rev().zip((0..tf).rev()) {
            attacks |= 1u64 << (r * 8 + f);
            if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
        }
        BitBoard(attacks)
    }

    pub fn get_rook_attack_mask(square: i32, blockers: u64) -> BitBoard {
        let tr = square / 8;
        let tf = square % 8;

        let mut attacks = 0;

        for r in (tr + 1)..8 {
            attacks |= 1u64 << (r * 8 + tf);
            if ((1 << (r * 8 + tf)) & blockers) != 0 { break; }
        }
        for r in (0..tr).rev() {
            attacks |= 1u64 << (r * 8 + tf);
            if ((1 << (r * 8 + tf)) & blockers) != 0 { break; }
        }
        for f in (tf + 1)..8 {
            attacks |= 1u64 << (tr * 8 + f);
            if ((1 << (tr * 8 + f)) & blockers) != 0 { break; }
        }
        for f in (0..tf).rev() {
            attacks |= 1u64 << (tr * 8 + f);
            if ((1 << (tr * 8 + f)) & blockers) != 0 { break; }
        }
        BitBoard(attacks)
    }

    pub fn get_queen_attack_mask(square: i32, blockers: u64) -> BitBoard {
        let bishop = Self::get_bishop_attack_mask(square, blockers);
        let rook = Self::get_rook_attack_mask(square, blockers);
        
        BitBoard(bishop.get_bits() | rook.get_bits())
    }
}

#[cfg(test)]
mod bitboard_tests {
    use super::BitBoard;

    #[test]
    fn test_set_bits_0() {
        let mut bits = BitBoard(0);
        bits.set_bit(5);
        assert_eq!(bits.0, 0b100000);
        bits.set_bit(4);
        assert_eq!(bits.0, 0b110000);
        bits.set_bit(3);
        assert_eq!(bits.0, 0b111000);
        bits.set_bit(2);
        assert_eq!(bits.0, 0b111100);
        bits.set_bit(1);
        assert_eq!(bits.0, 0b111110);
        bits.set_bit(0);
        assert_eq!(bits.0, 0b111111);
    }

    #[test]
    fn test_clear_bits_0() {
        let mut bits = BitBoard(0b111111);
        
        bits.clear_bit(5);
        assert_eq!(bits.0, 0b011111);
        bits.clear_bit(4);
        assert_eq!(bits.0, 0b001111);
        bits.clear_bit(3);
        assert_eq!(bits.0, 0b000111);
        bits.clear_bit(2);
        assert_eq!(bits.0, 0b000011);
        bits.clear_bit(1);
        assert_eq!(bits.0, 0b000001);
        bits.clear_bit(0);
        assert_eq!(bits.0, 0b000000);
    }

    #[test]
    fn test_get_bits_0() {
        let bits = BitBoard(0b101010);
        
        assert_eq!(bits.get_bit(5), true);
        assert_eq!(bits.get_bit(4), false);
        assert_eq!(bits.get_bit(3), true);
        assert_eq!(bits.get_bit(2), false);
        assert_eq!(bits.get_bit(1), true);
        assert_eq!(bits.get_bit(0), false);
    }
}
