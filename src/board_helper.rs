use crate::bitschess::board::CHESSBOARD_WIDTH;

/// # Has functions to help with board indexes and bit manipulation.
pub struct BoardHelper;

#[repr(u32)]
#[allow(dead_code)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,

    INVALID = std::u32::MAX
}

impl Square {
    /// Constructs a [Square] from [u32].
    #[inline(always)]
    pub const fn from_u32(f: u32) -> Self {
        unsafe {
            std::mem::transmute(f & 0b111111)
        }
    }
}

impl BoardHelper {
    /// Constructs a square index from a text.
    /// # Examples  
    /// ```rust
    /// use bitschess::BoardHelper;
    /// assert_eq!(BoardHelper::text_to_square("a1"), 0);
    /// assert_eq!(BoardHelper::text_to_square("B2"), 9);
    /// assert_eq!(BoardHelper::text_to_square("1"), -1); // invalid
    /// assert_eq!(BoardHelper::text_to_square("a"), -1); // invalid
    /// ```
    #[must_use]
    pub const fn text_to_square(uci_cmd: &str) -> i32 {
        if uci_cmd.len() < 2 {
            return -1;
        }

        let file_char = uci_cmd.as_bytes()[0] as char;
        let rank_char = uci_cmd.as_bytes()[1] as char;
        let file = Self::file_to_idx(file_char);
        let rank = Self::rank_to_idx(rank_char);
        Self::file_rank_to_idx(file, rank)
    }

    // 'a','1' -> 0, 'B', '2' -> 9
    #[must_use]
    pub const fn chars_to_square(file: char, rank: char) -> i32 {
        let file = Self::file_to_idx(file);
        let rank = Self::rank_to_idx(rank);
        Self::file_rank_to_idx(file, rank)
    }

    #[must_use]
    #[inline(always)]
    pub const fn file_to_idx(file: char) -> i32 {
        let file = file.to_ascii_lowercase();
        if file < 'a' || file > 'h' {
            return -1;
        }
        (7u8 - (b'h' - (file.to_ascii_lowercase() as u8))) as i32
    }

    #[must_use]
    #[inline(always)]
    pub const fn rank_to_idx(rank: char) -> i32 {
        let rank = rank.to_ascii_lowercase();
        if rank < '1' || rank > '8' {
            return -1;
        }
        if let Some(num) = rank.to_digit(10) {
            return (num as i32) - 1;
        }
        -1
    }

    #[must_use]
    #[inline(always)]
    pub const fn file_rank_to_idx(file: i32, rank: i32) -> i32 {
        if file < 0 || file > 7 || rank < 0 || rank > 7 {
            return -1;
        }
        rank * CHESSBOARD_WIDTH + file
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_rank(square: i32) -> i32 {
        square >> 3
    }

    #[must_use]
    #[inline(always)]
    pub const fn get_file(square: i32) -> i32 {
        square & 7
    }

    /// ```(file, rank)```
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn file_and_rank(square: i32 ) -> (i32, i32) {
        (Self::get_file(square), Self::get_rank(square))
    }

    /// ```(file, rank)```
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn square_to_chars(square: i32 ) -> (char, char) {
        if square == -1 {
            return ('-', '-');
        }

        let rank = (b'0' + (Self::get_rank(square) + 1) as u8) as char;
        let file = (b'a' + Self::get_file(square) as u8) as char;
        (file, rank)
    }

    /// ```outputs: "e4", "a1"```
    #[must_use]
    #[inline(always)]
    pub fn square_to_string(square: i32) -> String {
        if square == -1 {
            return String::from("");
        }

        let rank = (b'0' + (Self::get_rank(square) + 1) as u8) as char;
        let file = (b'a' + Self::get_file(square) as u8) as char;
        format!("{}{}", file, rank)
    }
    
    /// Determines the index of the least significant bit.
    /// Uses the De Bruijn Sequence.  
    /// <https://www.chessprogramming.org/BitScan>
    /// 
    /// # Examples
    /// ```rust
    /// use bitschess::BoardHelper;
    /// assert_eq!(BoardHelper::bitscan_forward(0b00001000), 3);
    /// assert_eq!(BoardHelper::bitscan_forward(0b00001100), 2);
    /// assert_eq!(BoardHelper::bitscan_forward(0b00001001), 0);
    /// assert_eq!(BoardHelper::bitscan_forward(0b00100000), 5);
    /// assert_eq!(BoardHelper::bitscan_forward(0b01011000), 3);
    /// assert_eq!(BoardHelper::bitscan_forward(0b11100000), 5);
    /// ```
    /// 
    /// # Panics
    /// BB cannot be 0!
    #[must_use]
    #[inline(always)]
    pub const fn bitscan_forward(bb: u64) -> i32 {
        assert!(bb != 0);
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
        INDEX64[(((bb ^ (bb-1)).wrapping_mul(DEBRUIJN64)) >> 58) as usize]
    }

    #[must_use]
    #[inline(always)]
    pub fn pop_lsb(bb: &mut u64) -> i32 {
        let pos = Self::bitscan_forward(*bb);
        *bb &= *bb - 1;
        pos
    }

    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub fn count_bits(mut b: u64) -> i32 {
        let mut count = 0;
        while b != 0 {
            count += 1;
            let _ = Self::pop_lsb(&mut b);
        }
        count
    }

    /// Checks if a string matches a syntax of a uci move.  
    /// 
    /// # Examples
    /// ```rust
    /// use bitschess::BoardHelper;
    /// assert_eq!(BoardHelper::is_valid_uci_move("a1a2"), true);
    /// assert_eq!(BoardHelper::is_valid_uci_move("a1a2q"), true); // not a valid chess move, but matches the expected syntax.
    ///  
    /// assert_eq!(BoardHelper::is_valid_uci_move("A1A2"), false);
    /// assert_eq!(BoardHelper::is_valid_uci_move("a1a2Q"), false);
    /// assert_eq!(BoardHelper::is_valid_uci_move("a1a2k"), false);
    /// assert_eq!(BoardHelper::is_valid_uci_move("a1"), false);
    /// assert_eq!(BoardHelper::is_valid_uci_move("11"), false);
    /// assert_eq!(BoardHelper::is_valid_uci_move("z2x2"), false);
    /// ```
    #[must_use]
    pub const fn is_valid_uci_move(uci_move: &str) -> bool {
        if uci_move.len() != 4 && uci_move.len() != 5 {
            return false;
        }
    
        let chars = uci_move.as_bytes();

        if BoardHelper::chars_to_square(chars[0] as char, chars[1] as char) == -1 {
            return false;
        }

        if BoardHelper::chars_to_square(chars[2] as char, chars[3] as char) == -1 {
            return false;
        }

        if chars.len() == 5 {
            let prom = chars[4].to_ascii_lowercase();
            return prom == b'q' || prom == b'r' || prom == b'b' || prom == b'n';
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
        assert_eq!(BoardHelper::text_to_square("A4"), Square::A4 as i32);
        assert_eq!(BoardHelper::text_to_square("h5"), Square::H5 as i32);
        assert_eq!(BoardHelper::text_to_square("E4"), Square::E4 as i32);
        assert_eq!(BoardHelper::text_to_square("h1"), Square::H1 as i32);
        assert_eq!(BoardHelper::text_to_square("H7"), Square::H7 as i32);
    }

    #[test]
    fn test_board_helper_square_to_string() {
        assert_eq!(BoardHelper::square_to_string(Square::A8 as i32), "a8");
        assert_eq!(BoardHelper::square_to_string(Square::B7 as i32), "b7");
        assert_eq!(BoardHelper::square_to_string(Square::C6 as i32), "c6");
        assert_eq!(BoardHelper::square_to_string(Square::D5 as i32), "d5");
        assert_eq!(BoardHelper::square_to_string(Square::E4 as i32), "e4");
        assert_eq!(BoardHelper::square_to_string(Square::F3 as i32), "f3");
        assert_eq!(BoardHelper::square_to_string(Square::G2 as i32), "g2");
        assert_eq!(BoardHelper::square_to_string(Square::H1 as i32), "h1");
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
    fn test_board_helper_file_rank_to_idx() {
        assert_eq!(BoardHelper::file_rank_to_idx(0, 7), Square::A8 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(1, 6), Square::B7 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(2, 5), Square::C6 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(3, 4), Square::D5 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(4, 3), Square::E4 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(5, 2), Square::F3 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(6, 1), Square::G2 as i32);
        assert_eq!(BoardHelper::file_rank_to_idx(7, 0), Square::H1 as i32);
    }

    #[test]
    fn test_board_helper_rank_to_idx() {
        assert_eq!(BoardHelper::rank_to_idx('1'), 0);
        assert_eq!(BoardHelper::rank_to_idx('2'), 1);
        assert_eq!(BoardHelper::rank_to_idx('3'), 2);
        assert_eq!(BoardHelper::rank_to_idx('4'), 3);
        assert_eq!(BoardHelper::rank_to_idx('5'), 4);
        assert_eq!(BoardHelper::rank_to_idx('6'), 5);
        assert_eq!(BoardHelper::rank_to_idx('7'), 6);
        assert_eq!(BoardHelper::rank_to_idx('8'), 7);
    }

    #[test]
    fn test_board_helper_get_rank() {
        assert_eq!(BoardHelper::get_rank(Square::H1 as i32), 0);
        assert_eq!(BoardHelper::get_rank(Square::G2 as i32), 1);
        assert_eq!(BoardHelper::get_rank(Square::F3 as i32), 2);
        assert_eq!(BoardHelper::get_rank(Square::E4 as i32), 3);
        assert_eq!(BoardHelper::get_rank(Square::D5 as i32), 4);
        assert_eq!(BoardHelper::get_rank(Square::C6 as i32), 5);
        assert_eq!(BoardHelper::get_rank(Square::B7 as i32), 6);
        assert_eq!(BoardHelper::get_rank(Square::A8 as i32), 7);
    }

    #[test]
    fn test_board_helper_get_file() {
        assert_eq!(BoardHelper::get_file(Square::H1 as i32), 7);
        assert_eq!(BoardHelper::get_file(Square::G2 as i32), 6);
        assert_eq!(BoardHelper::get_file(Square::F3 as i32), 5);
        assert_eq!(BoardHelper::get_file(Square::E4 as i32), 4);
        assert_eq!(BoardHelper::get_file(Square::D5 as i32), 3);
        assert_eq!(BoardHelper::get_file(Square::C6 as i32), 2);
        assert_eq!(BoardHelper::get_file(Square::B7 as i32), 1);
        assert_eq!(BoardHelper::get_file(Square::A8 as i32), 0);
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

    #[test]
    fn test_board_helper_chars_to_square() {
        assert_eq!(BoardHelper::chars_to_square('A', '8'), Square::A8 as i32);
        assert_eq!(BoardHelper::chars_to_square('b', '7'), Square::B7 as i32);
        assert_eq!(BoardHelper::chars_to_square('C', '6'), Square::C6 as i32);
        assert_eq!(BoardHelper::chars_to_square('d', '5'), Square::D5 as i32);
        assert_eq!(BoardHelper::chars_to_square('E', '4'), Square::E4 as i32);
        assert_eq!(BoardHelper::chars_to_square('f', '3'), Square::F3 as i32);
        assert_eq!(BoardHelper::chars_to_square('G', '2'), Square::G2 as i32);
        assert_eq!(BoardHelper::chars_to_square('h', '1'), Square::H1 as i32);
    }

    #[test]
    fn test_board_helper_is_valid_uci_move() {
        // Lower
        assert_eq!(BoardHelper::is_valid_uci_move("a1a2"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e2e7"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("a1h8"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("h8a1"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8q"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8n"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8b"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8r"), true);
        
        
        // Upper
        assert_eq!(BoardHelper::is_valid_uci_move("A1A7"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E2E7"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("A1H8"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("H8A1"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E7E8Q"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E7E8N"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E7E8B"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E7E8R"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("E7E8r"), true);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8R"), true);
        
        // Garbage
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8Z"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("e7e8z"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("2e27"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("1234"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("abc"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("11111111"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("e2a"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("e2"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("q2x5"), false);
        assert_eq!(BoardHelper::is_valid_uci_move("z2e2"), false);
    }
}