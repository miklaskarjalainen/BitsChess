use super::{ChessBoard, CHESSBOARD_WIDTH};

use crate::chessboard::board_helper::BoardHelper;
use crate::chessboard::piece::{Piece, PieceColor};

pub const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const STARTPOS_FEN_BLACK: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

impl ChessBoard {

    pub fn parse_fen(&mut self, fen_whole: &str) {
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
                    self.set_piece(y * CHESSBOARD_WIDTH + x, Piece::from_char(ch));
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
        
    }

    pub fn to_fen(board: ChessBoard) -> String {
        todo!();
        let mut fen = String::new();
        
        let mut empty_counter = 0u8;

        // Board
        for y in 0..8 {
            for x in 0..8 {
                let idx = y * 8 + x;
                // let piece = board.get_piece(idx).unwrap();

                /*
                if piece.is_none() {
                    empty_counter += 1;
                }
                else {
                    let piece_char = piece.to_char();
                    if empty_counter != 0 {
                        fen.push((('0' as u8) + empty_counter) as char);
                        empty_counter = 0;
                    }
                    fen.push(piece_char);
                }
                */
            }
            if empty_counter != 0 {
                fen.push((('0' as u8) + empty_counter) as char);
                empty_counter = 0;
            }
            fen.push('/')
        }
        fen.pop().unwrap(); // pop last '/'

        // Turn
        fen.push(' ');
        fen.push(if board.get_turn() == PieceColor::White {'w'} else {'b'});

        // Todo: Castling
        fen.push_str(" KQkq ");

        // En passant sqaure
        fen.push('-');
        
        /*
        if board.en_passant_square == -1 {
            
        }
        else {
            let (file, rank) = BoardHelper::square_to_chars(board.en_passant_square);
            fen.push(file);
            fen.push(rank);
        }
        */
        
        fen
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessboard::board_helper::BoardHelper;
    use crate::chessboard::piece::PieceType;

    #[test]
    fn test_parse_fen_basic1() {
        const TEST_PROMOTION_FEN: &str = "4k3/2P5/4K3/8/8/8/5p2/8 b - - 0 1";

        let mut board = ChessBoard::new();
        board.parse_fen(TEST_PROMOTION_FEN);

        assert_eq!(board.get_turn(), PieceColor::Black);

        let piece = board.get_piece(BoardHelper::text_to_square("f2"));
        assert_eq!(piece.get_piece_type(), PieceType::Pawn);
        assert_eq!(piece.get_color(), PieceColor::Black);
    }

    #[test]
    fn test_parse_fen_en_passant() {
        const TEST_EN_PASSANT_FEN: &str = "4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1";

        let mut board = ChessBoard::new();
        board.parse_fen(TEST_EN_PASSANT_FEN);

        assert_eq!(board.en_passant, BoardHelper::text_to_square("g6"));
    }
}