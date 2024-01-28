use const_for::const_for;

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

use crate::piece::PieceColor;

pub const PAWN_ATTACKS: [[u64; 64]; 2] = {
    let mut map = [[0; 64]; 2];
    const_for!(square in 0..64 => {
        map[0][square] = BitBoard::get_pawn_attack(PieceColor::White, square as i32);
        map[1][square] = BitBoard::get_pawn_attack(PieceColor::Black, square as i32);
    });
    map
};

pub const KNIGHT_ATTACKS: [u64; 64] = {
    let mut map = [0; 64];
    const_for!(square in 0..64 => {
        map[square] = BitBoard::get_knight_attack(square as i32);
    });
    map
};

pub const KING_ATTACKS: [u64; 64] = {
    let mut map = [0; 64];
    const_for!(square in 0..64 => {
        map[square] = BitBoard::get_king_attack(square as i32);
    });
    map
};

pub const BETWEENS: [[u64; 64]; 64] = {
    let mut map = [[0u64; 64]; 64];

    const_for!(from in 0..64 => {
        const_for!(to in 0..64 => {
            let occupancy = 1u64 << from;
            let to_bishop_mask   = BitBoard::get_bishop_attack_mask(to, occupancy);
            let to_rook_mask     = BitBoard::get_rook_attack_mask(to, occupancy);

            let occupancy_2 = 1u64 << to;
            let from_bishop_mask = BitBoard::get_bishop_attack_mask(from, occupancy_2);
            let from_rook_mask = BitBoard::get_rook_attack_mask(from, occupancy_2);
        
            if (to_bishop_mask & occupancy) != 0 {
                map[from as usize][to as usize] = to_bishop_mask & from_bishop_mask;
            }
            else if (to_rook_mask & occupancy) != 0 {
                map[from as usize][to as usize] = to_rook_mask & from_rook_mask;
            }
        });
    });

    map
};

pub struct BitBoard;

impl BitBoard {
    pub fn pretty(bits: u64) -> String {
        let mut str = String::from("");
        
        str.push_str("  a b c d e f g h\n");
        for y in (0..8).rev() {
            str.push_str((y+1).to_string().as_str());
            str.push(' ');
            for x in 0..8 {
                let bit = (bits >> (y * 8 + x)) & 1 == 1;
                str.push(if bit {'1'} else {'0'});
                str.push('|');
            }
            str.push('\n');
        }
        str.push_str("  a b c d e f g h\n");
        str
    }

    const fn get_pawn_attack(side: PieceColor, square: i32) -> u64 {
        let mut attacks = 0u64;
        let bitboard = 1u64 << square;
    
        if side.eq_const(PieceColor::White) {
            if ((bitboard << 7) & NOT_H_FILE) != 0 { 
                attacks |= bitboard << 7;
            }
            if ((bitboard << 9) & NOT_A_FILE) != 0 { 
                attacks |= bitboard << 9;
            }
        }
        else {
            if ((bitboard >> 7) & NOT_A_FILE) != 0 { 
                attacks |= bitboard >> 7;
            }
            if ((bitboard >> 9) & NOT_H_FILE) != 0 { 
                attacks |= bitboard >> 9;
            }
        }
    
        attacks
    }

    pub const fn get_knight_attack(square: i32) -> u64 {
        let mut attacks = 0u64;
        let bitboard = 1u64 << square;

        if bitboard & (NOT_H_FILE << 17) != 0 {
            attacks |= bitboard >> 17;
        }
        if bitboard & (NOT_A_FILE << 15) != 0 {
            attacks |= bitboard >> 15;
        }
        if bitboard & (NOT_HG_FILE << 10) != 0 {
            attacks |= bitboard >> 10;
        }
        if bitboard & (NOT_AB_FILE << 6) != 0 {
            attacks |= bitboard >> 6;
        }

        if bitboard & (NOT_A_FILE >> 17) != 0 {
            attacks |= bitboard << 17;
        }
        if bitboard & (NOT_H_FILE >> 15) != 0 {
            attacks |= bitboard << 15;
        }
        if bitboard & (NOT_AB_FILE >> 10) != 0 {
            attacks |= bitboard << 10;
        }
        if bitboard & (NOT_HG_FILE >> 6) != 0 {
            attacks |= bitboard << 6;
        }

        attacks
    }

    pub const fn get_king_attack(square: i32) -> u64 {
        let mut attacks = 0u64;
        let bitboard = 1u64 << square;

        if (bitboard >> 8) != 0 {
            attacks |= bitboard >> 8;
        }
        if ((bitboard >> 7) & NOT_A_FILE) != 0 {
            attacks |= bitboard >> 7;
        }
        if ((bitboard >> 9) & NOT_H_FILE) != 0 {
            attacks |= bitboard >> 9;
        }
        if ((bitboard >> 1) & NOT_H_FILE) != 0 {
            attacks |= bitboard >> 1;
        }

        if (bitboard << 8) != 0 {
            attacks |= bitboard << 8;
        }
        if ((bitboard << 7) & NOT_H_FILE) != 0 {
            attacks |= bitboard << 7;
        }
        if ((bitboard << 9) & NOT_A_FILE) != 0 {
            attacks |= bitboard << 9;
        }
        if ((bitboard << 1) & NOT_A_FILE) != 0 {
            attacks |= bitboard << 1;
        }

        attacks
    }
    
    pub const fn get_rook_attack_mask(square: i32, blockers: u64) -> u64 {
        let tr = square / 8;
        let tf = square % 8;

        let mut attacks = 0;

        const_for!(r in (tr + 1)..8 => {
            attacks |= 1u64 << (r * 8 + tf);
            if ((1 << (r * 8 + tf)) & blockers) != 0 { break; }
        });
        const_for!(r in (0..tr).rev() => {
            attacks |= 1u64 << (r * 8 + tf);
            if ((1 << (r * 8 + tf)) & blockers) != 0 { break; }
        });
        const_for!(f in (tf + 1)..8 => {
            attacks |= 1u64 << (tr * 8 + f);
            if ((1 << (tr * 8 + f)) & blockers) != 0 { break; }
        });
        const_for!(f in (0..tf).rev() => {
            attacks |= 1u64 << (tr * 8 + f);
            if ((1 << (tr * 8 + f)) & blockers) != 0 { break; }
        });
        attacks
    }

    pub const fn get_bishop_attack_mask(square: i32, blockers: u64) -> u64 {
        let tr = square / 8;
        let tf = square % 8;
    
        let mut attacks = 0;
    
        {
            let mut r = tr + 1;
            let mut f = tf + 1;
            while r < 8 && f < 8 {
                attacks |= 1u64 << (r * 8 + f);
                if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
                r += 1;
                f += 1;
            }
        }
    
        {
            let mut r = tr - 1;
            let mut f = tf + 1;
            while r >= 0 && f < 8 {
                attacks |= 1u64 << (r * 8 + f);
                if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
                r -= 1;
                f += 1;
            }
        }
    
        {
            let mut r = tr + 1;
            let mut f = tf - 1;
            while r < 8 && f >= 0 {
                attacks |= 1u64 << (r * 8 + f);
                if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
                r += 1;
                f -= 1;
            }
        }
    
        {
            let mut r = tr - 1;
            let mut f = tf - 1;
            while r >= 0 && f >= 0 {
                attacks |= 1u64 << (r * 8 + f);
                if ((1 << (r * 8 + f )) & blockers) != 0 { break; }
                r -= 1;
                f -= 1;
            }
        }
    
        attacks
    }
}
