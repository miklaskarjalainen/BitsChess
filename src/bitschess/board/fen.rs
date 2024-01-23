use super::{ChessBoard, CHESSBOARD_WIDTH};

use crate::board_helper::BoardHelper;
use crate::piece::{Piece, PieceColor, PieceType};

#[allow(dead_code)]
pub const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
#[allow(dead_code)]
pub const STARTPOS_FEN_BLACK: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

#[derive(Debug, PartialEq, Eq)]
pub enum FenParsingError {
    NoWhiteKing,
    NoBlackKing,
    OpponentInCheck
}

impl ChessBoard {

    pub fn parse_fen(&mut self, fen_whole: &str) -> Result<(), FenParsingError> {
        let mut args: Vec<&str> = fen_whole.split(' ').rev().collect();
        
        // Clear Board
        self.new_game();

        // Parse Position
        if let Some(fen_position) = args.pop() {
            let mut x = 0i32; 
            let mut y = 7i32;

            for ch in fen_position.chars() {
                // No piece
                if ch.is_ascii_digit() {
                    let num = ch.to_digit(10).unwrap();
                    x += num as i32;
                }
                else if ch != '/' {
                    let _ =  self.set_piece(y * CHESSBOARD_WIDTH + x, Piece::from_char(ch));
                    x += 1;
                }

                // overflow
                if x >= 8 {
                    x = 0;
                    y -= 1;
                }
            }
        }
        
        // Parse turn
        if let Some(fen_turn) = args.pop() {
            if fen_turn == "w" {
                self.set_turn(PieceColor::White);
            }
            else if fen_turn == "b" {
                self.set_turn(PieceColor::Black);
            }
        }
        
        // Parse castling right
        if let Some(castling_rights) = args.pop() {
            self.castling_rights[0] = castling_rights.contains('K');
            self.castling_rights[1] = castling_rights.contains('Q');
            self.castling_rights[2] = castling_rights.contains('k');
            self.castling_rights[3] = castling_rights.contains('q');
        }

        // Parse en passant square
        if let Some(en_passant) = args.pop() {
            if en_passant != "-" {
                self.en_passant = BoardHelper::text_to_square(en_passant);     
            }
        }

        // Parse half move
        if let Some(half_move) = args.pop() {
            if let Ok(parsed) = half_move.parse::<u8>() {
                self.half_move = parsed;     
            }
        }

        // Parse full move
        if let Some(full_move) = args.pop() {
            if let Ok(parsed) = full_move.parse::<u16>() {
                self.full_move = parsed;     
            }
        }
        
        // Error checking
        if self.bitboards[PieceType::King.get_side_index(PieceColor::White)].get_bits() == 0u64 {
            self.clear();
            return Err(FenParsingError::NoWhiteKing);
        }
        if self.bitboards[PieceType::King.get_side_index(PieceColor::Black)].get_bits() == 0u64 {
            self.clear();
            return Err(FenParsingError::NoBlackKing);
        }

        if self.is_king_in_check(self.get_turn().flipped()) {
            self.clear();
            return Err(FenParsingError::OpponentInCheck);
        }

        let hash = self.create_zobrist_hash();
        self.repetitions.increment_repetition(hash);
        self.zobrist_hash = hash;
        Ok(())
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        
        let mut empty_counter = 0u8;

        // Board
        for y in (0..8).rev() {
            for x in 0..8 {
                let idx = y * 8 + x;
                let piece = self.get_piece(idx);

                if piece.is_none() {
                    empty_counter += 1;
                }
                else {
                    let piece_char = piece.to_char();
                    if empty_counter != 0 {
                        fen.push(((b'0') + empty_counter) as char);
                        empty_counter = 0;
                    }
                    fen.push(piece_char);
                }
            }
            if empty_counter != 0 {
                fen.push((b'0' + empty_counter) as char);
                empty_counter = 0;
            }
            fen.push('/')
        }
        fen.pop().unwrap(); // pop last '/'

        // Turn
        fen.push(' ');
        fen.push(if self.get_turn() == PieceColor::White {'w'} else {'b'});
        
        // Castling rights
        fen.push(' ');
        if self.castling_rights[0] || self.castling_rights[1] || self.castling_rights[2] || self.castling_rights[3] {
            if self.castling_rights[0] { fen.push('K'); }
            if self.castling_rights[1] { fen.push('Q'); }
            if self.castling_rights[2] { fen.push('k'); }
            if self.castling_rights[3] { fen.push('q'); }
        }
        else {
            fen.push('-');
        }

        // En passant square
        fen.push(' ');
        if self.en_passant != -1 {
            let (file, rank) = BoardHelper::square_to_chars(self.en_passant);
            fen.push(file);
            fen.push(rank);
        }
        else {
            fen.push('-');
        }

        // Half & Full -moves
        fen.push(' ');
        fen.push_str(&self.half_move.to_string());
        fen.push(' ');
        fen.push_str(&self.full_move.to_string());


        fen
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_helper::BoardHelper;
    use crate::piece::PieceType;

    #[test]
    fn test_parse_fen_basic1() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/2P5/4K3/8/8/8/5p2/8 b - - 0 1").expect("valid fen");

        assert_eq!(board.get_turn(), PieceColor::Black);

        let piece = board.get_piece(BoardHelper::text_to_square("f2"));
        assert_eq!(piece.get_piece_type(), PieceType::Pawn);
        assert_eq!(piece.get_color(), PieceColor::Black);
    }

    #[test]
    fn test_parse_fen_parsing_error_opponent_in_check() {
        let mut board = ChessBoard::new();
        assert_eq!(board.parse_fen("k7/4n3/8/3K4/8/1N6/8/8 b - - 0 1"), Err(FenParsingError::OpponentInCheck));
        assert_eq!(board.parse_fen("k7/8/1N6/3K4/8/1n6/8/8 w - - 0 1"), Err(FenParsingError::OpponentInCheck));
    }

    #[test]
    fn test_parse_fen_parsing_no_white_king() {
        let mut board = ChessBoard::new();
        assert_eq!(board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w HAkq - 0 1"), Err(FenParsingError::NoWhiteKing));
    }

    #[test]
    fn test_parse_fen_parsing_no_black_king() {
        let mut board = ChessBoard::new();
        assert_eq!(board.parse_fen("rnbq1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQha - 0 1"), Err(FenParsingError::NoBlackKing));
    }

    #[test]
    fn test_to_fen_start_pos() {
        let mut board = ChessBoard::new();
        board.parse_fen(STARTPOS_FEN).expect("valid fen");
        board.make_move_uci("e2e4").expect("valid move");
        assert_eq!(board.to_fen(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    }

    #[test]
    fn test_parse_fen_en_passant() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1").expect("valid fen");
        assert_eq!(board.en_passant, BoardHelper::text_to_square("g6"));
    }

    #[test]
    fn test_parse_fen_half_and_full_moves() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/4k3/3p1p2/2pP1Pp1/2P1K1P1/8/8/8 w - - 69 420").expect("valid fen");
        assert_eq!(board.half_move, 69);
        assert_eq!(board.full_move, 420);
    }
}