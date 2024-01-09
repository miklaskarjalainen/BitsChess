use super::board::CHESSBOARD_WIDTH;

pub struct BoardHelper;

#[repr(u32)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {
    #[inline(always)]
    pub const fn from_u32(f: u32) -> Square {
        unsafe {
            std::mem::transmute(f & 0b111111)
        }
    }
}

impl BoardHelper {
    // "a1" -> 0, "B2" -> 9
    pub fn text_to_square(uci_cmd: &str) -> i32 {
        let file = Self::file_to_idx(uci_cmd.chars().nth(0).unwrap());
        let rank = uci_cmd.chars().nth(1).unwrap().to_digit(10).unwrap() as i32 - 1;
        let target_idx = rank * CHESSBOARD_WIDTH + file;
        target_idx
    }

    #[inline(always)]
    pub const fn file_to_idx(file: char) -> i32 {
        (7u8 - (b'h' - (file.to_ascii_lowercase() as u8))) as i32
    }

    #[inline(always)]
    pub const fn get_rank(square: i32) -> i32 {
        square / CHESSBOARD_WIDTH
    }

    #[inline(always)]
    pub const fn get_file(square: i32) -> i32 {
        square % CHESSBOARD_WIDTH
    }

    /// ```(file, rank)```
    #[inline(always)]
    pub const fn file_and_rank(square: i32 ) -> (i32, i32) {
        (BoardHelper::get_file(square), BoardHelper::get_rank(square))
    }

    /// ```(file, rank)```
    #[inline(always)]
    pub const fn square_to_chars(square: i32 ) -> (char, char) {
        if square == -1 {
            return ('-', '-');
        }

        let rank = (b'0' + (Self::get_rank(square) + 1) as u8) as char;
        let file: char = (b'a' + Self::get_file(square) as u8) as char;
        (file, rank)
    }
    
    // https://www.chessprogramming.org/BitScan
    #[inline(always)]
    pub const fn bitscan_forward(bb: u64) -> i32 {
        const INDEX64: [i32; 64] = [
            0, 47,  1, 56, 48, 27,  2, 60,
            57, 49, 41, 37, 28, 16,  3, 61,
            54, 58, 35, 52, 50, 42, 21, 44,
            38, 32, 29, 23, 17, 11,  4, 62,
            46, 55, 26, 59, 40, 36, 15, 53,
            34, 51, 20, 43, 31, 22, 10, 45,
            25, 39, 14, 33, 19, 30,  9, 24,
            13, 18,  8, 12,  7,  6,  5, 63
        ];
        const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;
        assert!(bb != 0);
        return INDEX64[(((bb ^ (bb-1)).wrapping_mul(DEBRUIJN64)) >> 58) as usize];
    }

    #[inline(always)]
    pub fn pop_rsb(bb: &mut u64) -> i32 {
        let pos = Self::bitscan_forward(*bb);
        *bb &= *bb - 1;
        pos
    }

    #[inline(always)]
    pub fn count_bits(mut b: u64) -> i32 {
        let mut count = 0;
        while b != 0 {
            count += 1;
            Self::pop_rsb(&mut b);
        }
        count
    }

    /// e2e4 e7e8q
    pub fn is_valid_uci_move(uci_move: &str) -> bool {
        if uci_move.len() != 4 && uci_move.len() != 5 {
            return false;
        }
        
        let chars: Vec<char> = uci_move.chars().collect();


        if chars[0] < 'a' || chars[0] > 'h' {
            return false;
        }

        if chars[1] < '1' || chars[1] > '9' {
            return false;
        }

        if chars[2] < 'a' || chars[2] > 'h' {
            return false;
        }

        if chars[3] < '1' || chars[3] > '9' {
            return false;
        }

        if chars.len() == 5 && !(chars[4] == 'q' || chars[4] == 'r' || chars[4] == 'b' || chars[4] == 'n') {
            return false;
        }

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_helper_text_to_square() {
        assert_eq!(BoardHelper::text_to_square("a1"), Square::A1 as i32);
        assert_eq!(BoardHelper::text_to_square("a4"), Square::A4 as i32);
        assert_eq!(BoardHelper::text_to_square("h5"), Square::H5 as i32);
        assert_eq!(BoardHelper::text_to_square("e4"), Square::E4 as i32);
        assert_eq!(BoardHelper::text_to_square("h1"), Square::H1 as i32);
        assert_eq!(BoardHelper::text_to_square("h7"), Square::H7 as i32);
    }
    
    #[test]
    fn test_board_helper_file_to_idx() {
        assert_eq!(BoardHelper::file_to_idx('a'), Square::A1 as i32);
        assert_eq!(BoardHelper::file_to_idx('A'), Square::A1 as i32);
        assert_eq!(BoardHelper::file_to_idx('b'), Square::B1 as i32);
        assert_eq!(BoardHelper::file_to_idx('B'), Square::B1 as i32);
        assert_eq!(BoardHelper::file_to_idx('c'), Square::C1 as i32);
        assert_eq!(BoardHelper::file_to_idx('C'), Square::C1 as i32);
        assert_eq!(BoardHelper::file_to_idx('d'), Square::D1 as i32);
        assert_eq!(BoardHelper::file_to_idx('D'), Square::D1 as i32);
        assert_eq!(BoardHelper::file_to_idx('e'), Square::E1 as i32);
        assert_eq!(BoardHelper::file_to_idx('E'), Square::E1 as i32);
        assert_eq!(BoardHelper::file_to_idx('f'), Square::F1 as i32);
        assert_eq!(BoardHelper::file_to_idx('F'), Square::F1 as i32);
        assert_eq!(BoardHelper::file_to_idx('g'), Square::G1 as i32);
        assert_eq!(BoardHelper::file_to_idx('G'), Square::G1 as i32);
        assert_eq!(BoardHelper::file_to_idx('h'), Square::H1 as i32);
        assert_eq!(BoardHelper::file_to_idx('H'), Square::H1 as i32);
    }

    #[test]
    fn test_board_helper_get_rank() {
        assert_eq!(BoardHelper::get_rank(Square::A1 as i32), 0);
        assert_eq!(BoardHelper::get_rank(Square::B2 as i32), 1);
        assert_eq!(BoardHelper::get_rank(Square::C3 as i32), 2);
        assert_eq!(BoardHelper::get_rank(Square::D4 as i32), 3);
        assert_eq!(BoardHelper::get_rank(Square::E5 as i32), 4);
        assert_eq!(BoardHelper::get_rank(Square::F6 as i32), 5);
        assert_eq!(BoardHelper::get_rank(Square::G7 as i32), 6);
        assert_eq!(BoardHelper::get_rank(Square::H8 as i32), 7);
    }

    #[test]
    fn test_board_helper_get_file() {
        assert_eq!(BoardHelper::get_rank(Square::A1 as i32), 0);
        assert_eq!(BoardHelper::get_rank(Square::B2 as i32), 1);
        assert_eq!(BoardHelper::get_rank(Square::C3 as i32), 2);
        assert_eq!(BoardHelper::get_rank(Square::D4 as i32), 3);
        assert_eq!(BoardHelper::get_rank(Square::E5 as i32), 4);
        assert_eq!(BoardHelper::get_rank(Square::F6 as i32), 5);
        assert_eq!(BoardHelper::get_rank(Square::G7 as i32), 6);
        assert_eq!(BoardHelper::get_rank(Square::H8 as i32), 7);
    }

    #[test]
    fn test_board_helper_file_and_rank() {
        assert_eq!(BoardHelper::file_and_rank(Square::A8 as i32), (0, 7));
        assert_eq!(BoardHelper::file_and_rank(Square::B7 as i32), (1, 6));
        assert_eq!(BoardHelper::file_and_rank(Square::C6 as i32), (2, 5));
        assert_eq!(BoardHelper::file_and_rank(Square::D5 as i32), (3, 4));
        assert_eq!(BoardHelper::file_and_rank(Square::E4 as i32), (4, 3));
        assert_eq!(BoardHelper::file_and_rank(Square::F3 as i32), (5, 2));
        assert_eq!(BoardHelper::file_and_rank(Square::G2 as i32), (6, 1));
        assert_eq!(BoardHelper::file_and_rank(Square::H1 as i32), (7, 0));
    }

    #[test]
    fn test_board_helper_square_to_chars() {
        assert_eq!(BoardHelper::square_to_chars(Square::A8 as i32), ('a', '8'));
        assert_eq!(BoardHelper::square_to_chars(Square::B7 as i32), ('b', '7'));
        assert_eq!(BoardHelper::square_to_chars(Square::C6 as i32), ('c', '6'));
        assert_eq!(BoardHelper::square_to_chars(Square::D5 as i32), ('d', '5'));
        assert_eq!(BoardHelper::square_to_chars(Square::E4 as i32), ('e', '4'));
        assert_eq!(BoardHelper::square_to_chars(Square::F3 as i32), ('f', '3'));
        assert_eq!(BoardHelper::square_to_chars(Square::G2 as i32), ('g', '2'));
        assert_eq!(BoardHelper::square_to_chars(Square::H1 as i32), ('h', '1'));
    }
}