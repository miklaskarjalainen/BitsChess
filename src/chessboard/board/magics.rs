use lazy_static::lazy_static;

use super::*;

// https://www.chessprogramming.org/Looking_for_Magics
const ROOK_MAGICS: [u64; 64] = [0x80088250204000, 0x1040400010002000, 0x3080100008802000, 0x200060010082040, 0x900100402080100, 0x6200100104880200, 0x480010018800200, 0x28001c025000080, 0x2002100820044, 0x82401000402000, 0x29001100200842, 0x400a0010400a0022, 0x2000408120020, 0xa000850020004, 0x13000e00030084, 0x143000214824300, 0x244024800840008b, 0x110004020004000, 0x2a02020010842040, 0x284200100a0020, 0x18004004004200, 0x1401010002080400, 0x500040008100201, 0x100120000844d04, 0x480004040002000, 0x20004040003000, 0x1020200100104100, 0x14090100100220, 0x808080100041100, 0x801004900040002, 0x800080400100102, 0x1080110200006084, 0x400022800088, 0x106401004402000, 0x402001001900, 0x23214202001008, 0xe21000801000410, 0x102001002000805, 0x14080208040010c1, 0x23000041002082, 0x7001008000410021, 0x231004200820024, 0x102031021820041, 0x401001000210008, 0x1000800050011, 0x40d1000804010002, 0x404080210040001, 0x210128408420007, 0x80004002200340, 0x204500920200, 0x21154020010100, 0x635002108100100, 0x400080100100500, 0xa34000200800480, 0x1420013048820400, 0x108c289144030200, 0x2921002010800041, 0x20104000210089, 0x3088104500200009, 0x2000810044022, 0x2802000810042002, 0x1003000400080201, 0x204400a210110804, 0x220090420804402 ];
const ROOK_SHIFTS: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

const BISHOP_MAGICS: [u64; 64] = [0x36c2100401140020, 0x89281104002020, 0x1424a0a43000406, 0x848204840400c10, 0x84042002884010, 0x22021006050061, 0x102015002110008, 0x1806210042202041, 0x5604210264080, 0x8443042320028, 0x20e221600520180, 0x280080a12200008, 0x580240420880000, 0x20084108200000, 0x4060020101201000, 0x1000022401080880, 0x120101044810804, 0x620040541141b00, 0x80210015142a8, 0x4000840400810, 0x1004008822080800, 0x201000020884040, 0xc000041445001, 0x2010210210840408, 0x4002200313041000, 0x848084120010140, 0x2201010010084200, 0x105040014440080, 0x1005010004104002, 0x3208008000406004, 0x88012000410820, 0x5201050000404800, 0x2090480500200400, 0x1822029001200100, 0x2024040204010200, 0x32b2600800090106, 0x40104010110100, 0x30110040020040, 0x5004108402022103, 0x1218010a58002200, 0x140d084904104002, 0x20a0a012005050a, 0x2002034004800, 0x14a002018042100, 0x204210202002c20, 0x1021201800800042, 0x1489420400490400, 0x8c240260414202, 0x400c0a1804040180, 0x10412401205800, 0x200084404041000, 0x4080080104090122, 0x88000200a442000, 0x2c80042058024200, 0x3023084f040012, 0x203101401004802, 0x80610c04844000, 0x4142088c2004, 0x242012200520819, 0x102040000e420228, 0x203020025401, 0x4030021224, 0x2000043004011420, 0x2040040100420084 ];
const BISHOP_SHIFTS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

lazy_static!{
    pub static ref ROOK_MASK: [u64; 64] = {
        let mut mask = [0u64; 64];
        for square in 0..64 {
            mask[square as usize] = rook_mask(square);
        }
        mask
    };

    pub static ref ROOK_ATTACK_MAP: Vec<[u64; 4096]> = {
        let mut map = vec![[0u64; 4096]; 64];

        for square in 0..64 {
            let mask = rook_mask(square);
            let blockers = generate_blocker_bitboards(mask);
            
            for b in blockers {
                let key = magic_index(ROOK_MAGICS[square as usize], b, ROOK_SHIFTS[square as usize]);
                map[square as usize][key as usize] = BitBoard::get_rook_attack_mask(square, b).get_bits();
            }
        }
        map
    };

    pub static ref BISHOP_MASK: [u64; 64] = {
        let mut mask = [0u64; 64];
        for square in 0..64 {
            mask[square as usize] = bishop_mask(square);
        }
        mask
    };

    pub static ref BISHOP_ATTACK_MAP: Vec<[u64; 512]> = {
        let mut map = vec![[0u64; 512]; 64];

        for square in 0..64 {
            let mask = bishop_mask(square);
            let blockers = generate_blocker_bitboards(mask);
            
            for b in blockers {
                let key = magic_index(BISHOP_MAGICS[square as usize], b, BISHOP_SHIFTS[square as usize]);
                map[square as usize][key as usize] = BitBoard::get_bishop_attack_mask(square, b).get_bits();
            }
        }
        map
    };
}

#[inline(always)]
pub const fn magic_index(magic: u64, blockers: u64, shift: u64) -> usize {
    ((magic.wrapping_mul(blockers)) >> (64 - shift)) as usize
}

#[inline(always)]
pub fn get_bishop_magic(square: i32, blockers: u64) -> u64 {
    let magic = BISHOP_MAGICS[square as usize];
    let shift = BISHOP_SHIFTS[square as usize];
    let mask: u64 = BISHOP_MASK[square as usize];
    BISHOP_ATTACK_MAP[square as usize][magic_index(magic, blockers & mask, shift)]
}

#[inline(always)]
pub fn get_rook_magic(square: i32, blockers: u64) -> u64 {
    let magic = ROOK_MAGICS[square as usize];
    let shift = ROOK_SHIFTS[square as usize];
    let mask: u64 = ROOK_MASK[square as usize];
    ROOK_ATTACK_MAP[square as usize][magic_index(magic, blockers & mask, shift)]
}

pub fn bishop_mask(square: i32) -> u64 {
    let tr = square / 8;
    let tf = square % 8;

    let mut attacks = 0;

    for (r, f) in ((tr + 1)..7).zip((tf + 1)..7) {
        attacks |= 1u64 << (r * 8 + f);
    }
    for (r, f) in (1..tr).rev().zip((tf+1)..7) {
        attacks |= 1u64 << (r * 8 + f);
    }
    for (r, f) in ((tr + 1)..7).zip((1..tf).rev()) {
        attacks |= 1u64 << (r * 8 + f);
    }
    for (r, f) in (1..tr).rev().zip((1..tf).rev()) {
        attacks |= 1u64 << (r * 8 + f);
    }
    attacks
}

pub fn rook_mask(square: i32) -> u64 {
    let mut attacks = 0;
    let tr = square / 8;
    let tf = square % 8;

    for r in (tr + 1)..7 { 
        attacks |= 1u64 << (r * 8 + tf)
    }
    for r in (1..tr).rev() { 
        attacks |= 1u64 << (r * 8 + tf)
    }
    for f in (tf + 1)..7 { 
        attacks |= 1u64 << (tr * 8 + f);
    }
    for f in (1..tf).rev() { 
        attacks |= 1u64 << (tr * 8 + f);
    }
    return attacks;
}


fn generate_blocker_bitboards(mask: u64) -> Vec<u64> {
    let mut move_square_indices: Vec<i32> = vec![];

    for square in 0..64 {
        if ((mask >> square) & 1) == 1 {
            move_square_indices.push(square);
        }
    }

    let num_patterns = 1 << move_square_indices.len();
    let mut blocker_bitboards = Vec::<u64>::with_capacity(num_patterns);

    for pattern_index in 0..num_patterns {
        blocker_bitboards.push(0u64);

        for bit_index in 0..move_square_indices.len() {
            let bit = (pattern_index >> bit_index) & 0b1;
            blocker_bitboards[pattern_index] |= (bit as u64) << move_square_indices[bit_index];
        }
    }

    return blocker_bitboards;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_magic_bitboard_rook_storing_correct_mask() {
        for square in 0..64 {
            let mask = rook_mask(square);
            let blockers = generate_blocker_bitboards(mask);

            for b in blockers {
                let idx = magic_index(ROOK_MAGICS[square as usize], b, ROOK_SHIFTS[square as usize]);

                let magic_mask = BitBoard::new(ROOK_ATTACK_MAP[square as usize][idx]);
                let expected_mask = BitBoard::get_rook_attack_mask(square, b);
                
                if magic_mask != expected_mask {
                    println!("expected: \n{}", expected_mask);
                    println!("got: \n{}", magic_mask);
                    panic!();
                }
                
            }
        }
    }

    #[test]
    fn test_magic_bitboard_rook_index_collision() {
        for square in 0..64 {
            let mut square_indexes: HashMap<usize, u64> = HashMap::new();

            let mask = rook_mask(square);
            let blockers = generate_blocker_bitboards(mask);

            for b in blockers {
                let idx = magic_index(ROOK_MAGICS[square as usize], b, ROOK_SHIFTS[square as usize]);
                let mask = BitBoard::get_rook_attack_mask(square, b).get_bits();

                if let Some(other_mask) = square_indexes.get(&idx) {
                    assert_eq!(*other_mask, mask, "idx: [{}]", idx);
                }

                square_indexes.insert(idx, mask);
            }
        }
    }

    #[test]
    fn test_magic_bitboard_bishop_storing_correct_mask() {
        for square in 0..64 {
            let mask = bishop_mask(square);
            let blockers = generate_blocker_bitboards(mask);

            for b in blockers {
                let idx = magic_index(BISHOP_MAGICS[square as usize], b, BISHOP_SHIFTS[square as usize]);

                let magic_mask = BitBoard::new(BISHOP_ATTACK_MAP[square as usize][idx]);
                let expected_mask = BitBoard::get_bishop_attack_mask(square, b);
                
                if magic_mask != expected_mask {
                    println!("expected: \n{}", expected_mask);
                    println!("got: \n{}", magic_mask);
                    panic!();
                }
                
            }
        }
    }

    #[test]
    fn test_magic_bitboard_bishop_index_collision() {
        for square in 0..64 {
            let mut square_indexes: HashMap<usize, u64> = HashMap::new();

            let mask = bishop_mask(square);
            let blockers = generate_blocker_bitboards(mask);

            for b in blockers {
                let idx = magic_index(BISHOP_MAGICS[square as usize], b, BISHOP_SHIFTS[square as usize]);
                let mask = BitBoard::get_bishop_attack_mask(square, b).get_bits();

                if let Some(other_mask) = square_indexes.get(&idx) {
                    assert_eq!(*other_mask, mask, "idx: [{}]", idx);
                }

                square_indexes.insert(idx, mask);
            }
        }
    }
}