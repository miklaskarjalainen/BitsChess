// https://www.chessprogramming.org/Zobrist_Hashing

use super::{Piece, PieceColor, ChessBoard, BoardHelper};

use lazy_static::lazy_static;

const ZOBRIST_SEED: u64 = 212832809410876;
pub const ZOBRIST_TURN: usize = 64*12;
pub const ZOBRIST_CASTLING: usize = ZOBRIST_TURN + 1; // + 4
pub const ZOBRIST_EN_PASSANT: usize = ZOBRIST_CASTLING + 4; // + 8

lazy_static! {
    pub static ref ZOBRIST_KEYS: [u64; 12*64 + 1 + 4 + 8] = {
        fastrand::seed(ZOBRIST_SEED);
        [0; 12*64 + 1 + 4 + 8].map(|_| fastrand::u64(..))
    };
}

impl Piece {
    #[inline(always)]
    pub fn get_hash(self, square: i32) -> u64 {
        assert!(!self.is_none());
        ZOBRIST_KEYS[(square as usize) * 12 + self.get_piece_index()]
    }
}

impl ChessBoard {
    /// Creates a completely new zobrist_hash (independent from the member variable)
    pub fn create_zobrist_hash(&self) -> u64 {
        let mut hash = 0u64;
        
        // Add pieces
        let mut pieces = self.get_side_mask(PieceColor::White) | self.get_side_mask(PieceColor::Black);
        while pieces != 0 {
            let square = BoardHelper::bitscan_forward(pieces);
            pieces ^= 1u64 << square;
            hash ^= self.get_piece(square).get_hash(square);
        }

        // Castling rights
        for i in 0..4 {
            if self.castling_rights[i] {
                hash ^= ZOBRIST_KEYS[ZOBRIST_CASTLING + i];
            }
        }

        // En passant
        if self.en_passant != -1 {
            hash ^= ZOBRIST_KEYS[ZOBRIST_EN_PASSANT + (self.en_passant % 8) as usize];
        }

        if self.get_turn() == PieceColor::Black {
            hash ^= ZOBRIST_KEYS[ZOBRIST_TURN];
        }
        
        hash
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::fen::STARTPOS_FEN;
    use crate::board_helper::BoardHelper;

    #[test]
    fn test_verify_zobrist_keys() {
        // Checks that there isn't any same keys in the array
        for x in 0..ZOBRIST_KEYS.len() {
            for y in 0..ZOBRIST_KEYS.len() {
                if x == y { continue; }
                assert_ne!(ZOBRIST_KEYS[x], ZOBRIST_KEYS[y], "ZOBRIST_KEY contains 2 identical keys at {} and {}. Use a different SEED!", x ,y);
            }
        }
    }

    #[test]
    fn test_make_move_zobrist_updation_basic() {
        let mut board = ChessBoard::new();
        board.parse_fen(STARTPOS_FEN).expect("valid fen");
        board.make_move_uci("e2e4").expect("valid");
        board.make_move_uci("e7e5").expect("valid");
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }

    #[test]
    fn test_make_move_zobrist_updation_en_passant_square() {
        let mut board = ChessBoard::new();
        board.parse_fen(STARTPOS_FEN).expect("valid fen");
        board.make_move_uci("e2e4").expect("valid");
        assert_eq!(board.en_passant, BoardHelper::text_to_square("e3"));
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }

    #[test]
    fn test_en_passant_changes_hash() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1").expect("valid fen");
        let hash_with_en_passant = board.create_zobrist_hash();

        board.parse_fen("4k3/8/8/5Pp1/8/8/8/4K3 w - - 0 1").expect("valid fen");
        let hash_without_en_passant = board.create_zobrist_hash();
        assert_ne!(hash_with_en_passant, hash_without_en_passant);
    }

    #[test]
    fn test_make_undo_move_zobrist_updation_en_passant_square() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1").expect("valid fen");
        let copy = board.clone();
        board.make_move_uci("f5g6").expect("valid");
        board.unmake_move().expect("valid");
        assert_eq!(board, copy);
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }

    #[test]
    fn test_make_move_zobrist_updating_castling() {
        let mut board = ChessBoard::new();
        board.parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").expect("valid fen");
        board.make_move_uci("e1g1").expect("valid");
        board.make_move_uci("e8c8").expect("valid");
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    
        board.parse_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").expect("valid fen");
        board.make_move_uci("e8g8").expect("valid");
        board.make_move_uci("e1c1").expect("valid");
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }

    #[test]
    fn test_make_move_zobrist_updating_castling_rook_capture() {
        let mut board = ChessBoard::new();
        board.parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").expect("valid fen");
        board.make_move_uci("a1a8").expect("valid");

        assert_eq!(board.castling_rights, [true, false, true, false]);
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }

    #[test]
    fn test_make_undo_move_zobrist_updation_basic() {
        let mut board = ChessBoard::new();
        board.parse_fen(STARTPOS_FEN).expect("valid fen");
        board.make_move_uci("e2e4").expect("valid");
        board.make_move_uci("e7e5").expect("valid");
        board.unmake_move().expect("valid");
        board.unmake_move().expect("valid");
        assert_eq!(board.zobrist_hash, board.create_zobrist_hash());
    }
}
