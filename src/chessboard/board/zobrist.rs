// https://www.chessprogramming.org/Zobrist_Hashing

use super::{Piece, PieceColor, ChessBoard, BoardHelper};

use lazy_static::lazy_static;

const ZOBRIST_SEED: u64 = 212832809410876;
const ZOBRIST_SIDE_IDX: usize = 64*12;
const ZOBRIST_CASTLING: usize = ZOBRIST_SIDE_IDX + 1; // + 4
const ZOBRIST_EN_PASSANT: usize = ZOBRIST_CASTLING + 5; // + 8 (1 for every file)

lazy_static! {
    pub static ref ZOBRIST_KEYS: [u64; 12*64 + 1 + 4 + 8] = {
        fastrand::seed(ZOBRIST_SEED);
        [0; 12*64 + 1 + 4 + 8].map(|_| fastrand::u64(..))
    };
}

impl Piece {
    pub fn get_hash(self, square: i32) -> u64 {
        assert!(!self.is_none());
        return ZOBRIST_KEYS[(square as usize) * 12 + self.get_piece_index()];
    }
}

impl ChessBoard {
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
        
        // en passant, not needed for 3-fold repetition. 
        /*
        if self.en_passant != -1 {
            let file = BoardHelper::get_file(self.en_passant) as usize;
            hash ^= ZOBRIST_KEYS[ZOBRIST_EN_PASSANT + file];
        }
        */ 
        
        hash
    }


}

#[cfg(test)]
mod tests {
    use super::*;

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
}
