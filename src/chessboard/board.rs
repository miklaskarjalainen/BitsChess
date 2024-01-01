pub mod fen;
pub mod move_generation;
pub mod perft;
pub mod zobrist;

use std::collections::HashMap;

use super::bitboard::BitBoard;
use super::board_helper::BoardHelper;
use super::chessmove::ReversibleMove;
use super::piece::{Piece, PieceType, PieceColor};
use super::board_helper::Square;
use super::chessmove::{Move, MoveFlag};

pub const CHESSBOARD_WIDTH: i32 = 8;

#[derive(Clone, Debug)]
pub struct ChessBoard {
    // Board representation
    // "masks" for every different type of piece
    pub bitboards: [BitBoard; 12], // 0 = white pawns, 1 = white knights ... 6 = black pawns, etc
    pub side_bitboards: [BitBoard; 2],
    pub board: [Piece; 64],

    // Pieces with squares, -1 is none and 0 >= is the square
    white_pieces : [i32; 16], // contains every piece
    white_pawns  : [i32; 8],
    white_knights: [i32; 10],
    white_bishops: [i32; 10],
    white_rooks  : [i32; 10],
    white_queens : [i32; 9],
    white_kings  : [i32; 1],
    
    black_pieces : [i32; 16], // contains every piece
    black_pawns  : [i32; 8],
    black_knights: [i32; 10],
    black_bishops: [i32; 10],
    black_rooks  : [i32; 10],
    black_queens : [i32; 9],
    black_kings  : [i32; 1],

    // flags
    pub turn: PieceColor,
    pub en_passant: i32,
    /// lines up with fen's "KQkq" -> [white_king_side, white_queen_side, black_king_side, black_queen_side]
    pub castling_rights: [bool; 4],  
    pub half_move: u8,
    pub full_move: u16,

    repetitions: HashMap<u64, u8>,
    move_history: Vec<ReversibleMove>
}

impl PartialEq for ChessBoard {
    /// !Square look up tables are not being compared, because they are not guaranteed to be in the same order.
    /// Equality is mostly used in tests anyways...

    fn eq(&self, other: &Self) -> bool {
        self.bitboards == other.bitboards && 
        self.side_bitboards == other.side_bitboards && 
        self.board == other.board && 

        self.turn == other.turn && 
        self.en_passant == other.en_passant && 
        self.castling_rights == other.castling_rights && 
        self.move_history == other.move_history &&
        self.half_move == other.half_move &&
        self.full_move == other.full_move
    }
}

impl std::fmt::Display for ChessBoard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("");
        
        str.push_str("   a b c d e f g h\n");
        for y in (0..8).rev() {
            str.push_str((y+1).to_string().as_str());
            str.push(' ');
            str.push('|');
            for x in 0..8 {
                let piece = self.get_piece(y * 8 + x);
                str.push(piece.to_char());
                str.push('|');
            }
            str.push(' ');
            str.push_str((y+1).to_string().as_str());
            str.push('\n');
        }
        str.push_str("   a b c d e f g h\n\n");

        let turn = self.get_turn();
        str.push_str(format!("turn: {:?}\n", self.turn).as_str());
        str.push_str(format!("is in check: {}\n", self.is_king_in_check(turn)).as_str());
        str.push_str(format!("castle rights: {:?}\n", self.castling_rights).as_str());
        str.push_str(format!("en_passant: {}\n", self.en_passant).as_str());
        str.push_str(format!("half move: {}\n", self.half_move).as_str());
        str.push_str(format!("full move: {}\n", self.full_move).as_str());
        str.push_str(format!("zobrist: {}\n", self.create_zobrist_hash()).as_str());
        str.push_str(format!("repetitions: {:?}\n", self.repetitions).as_str());
        str.push('[');
        for m in &self.move_history {
            str.push_str(format!("'{}', ", &m.board_move.to_uci()).as_str());
        }
        str.push(']');

        formatter.pad(str.as_str())
    }
}

impl ChessBoard {
    pub fn clear(&mut self) {
        for idx in 0..64 {
            self.set_piece(idx, Piece::new(0));
        }
        
        self.turn = PieceColor::White;
        self.move_history.clear();
        self.repetitions.clear();
        self.en_passant = -1;
        self.full_move = 1;
        self.half_move = 0;
    }

    pub fn new_game(&mut self) {
        self.clear();
    }

    /// Only does legal moves.
    pub fn make_move_uci(&mut self, uci: &str) -> Option<()> {
        let from = BoardHelper::text_to_square(&uci[0..2]);
        let legal_moves = self.get_legal_moves_for_square(from);
        let mut filtered_moves: Vec<Move> = legal_moves.into_iter().filter(|m| { return m.to_uci() == uci}).collect();
        if filtered_moves.len() == 0 {
            println!("illegal move! :^(");
            return None;
        }
        let m = filtered_moves.pop().expect("?");
        self.make_move(m, false);
        
        Some(())
    }

    pub fn make_move(&mut self, chess_move: Move, is_in_search: bool) {
        let from = chess_move.get_from_idx();
        let to = chess_move.get_to_idx();
        let mut moving_piece = self.get_piece(from);
        
        if moving_piece.is_none() { return; }

        // Handle en passant
        let en_passant_hold = self.en_passant;
        self.en_passant = -1;
        self.full_move += self.turn as u16; // white = 0, black = 1
        self.turn.flip();
        
        match chess_move.get_flag() {
            MoveFlag::None => { }
            MoveFlag::EnPassant => { 
                let en_passant_dir = if moving_piece.get_color() == PieceColor::Black { 8 } else { -8 };

                // Move
                self.set_piece(from, Piece::new(0));
                self.set_piece(to, moving_piece);
                
                // Capture
                let captured = self.set_piece(to + en_passant_dir, Piece::new(0));

                // Save to history
                let reversible = ReversibleMove::new(chess_move, captured, en_passant_hold, self.castling_rights, self.half_move);
                self.move_history.push(reversible);
                self.half_move = 0;
                
                let hash = self.create_zobrist_hash();
                self.repetitions.entry(hash)
                    .and_modify(|h| *h += 1)
                    .or_insert(1);
                return;
            }
            MoveFlag::PawnTwoUp => {
                let en_passant_dir = if moving_piece.get_color() == PieceColor::White { 8 } else { -8 };
                self.en_passant = from + en_passant_dir;
            }
            MoveFlag::Castle => {
                match Square::from_u32(to as u32) {
                    // White king side
                    Square::G1 => {
                        let rook = self.set_piece(Square::H1 as i32, Piece::new(0));
                        self.set_piece(Square::F1 as i32, rook);
                    }
                    // White queen side
                    Square::C1 => {
                        let rook = self.set_piece(Square::A1 as i32, Piece::new(0));
                        self.set_piece(Square::D1 as i32, rook);
                    }

                    // Black king side
                    Square::G8 => {
                        let rook = self.set_piece(Square::H8 as i32, Piece::new(0));
                        self.set_piece(Square::F8 as i32, rook);
                    }
                    // Black queen side
                    Square::C8 => {
                        let rook = self.set_piece(Square::A8 as i32, Piece::new(0));
                        self.set_piece(Square::D8 as i32, rook);
                    }

                    _ => { panic!("huh????? {}", to); }
                }
            }
            
            MoveFlag::PromoteQueen  => { moving_piece.set_piece(PieceType::Queen); }
            MoveFlag::PromoteRook   => { moving_piece.set_piece(PieceType::Rook); }
            MoveFlag::PromoteBishop => { moving_piece.set_piece(PieceType::Bishop); }
            MoveFlag::PromoteKnight => { moving_piece.set_piece(PieceType::Knight); }
            
            #[allow(unreachable_patterns)]
            _ => { panic!("MoveFlag: {}", chess_move.get_flag() as u8); }
        }
        
        // Move & Capture
        self.set_piece(from, Piece::new(0));
        let captured = self.set_piece(to, moving_piece);
        
        // Save to history
        let reversible = ReversibleMove::new(chess_move, captured, en_passant_hold, self.castling_rights, self.half_move);
        self.move_history.push(reversible);

        // Half move
        if !captured.is_none() || moving_piece.get_piece_type() == PieceType::Pawn {
            self.half_move = 0;
        } else {
            self.half_move += 1
        }        

        // Disable castling rights
        match moving_piece.get_piece_type() {
            PieceType::King => {
                if moving_piece.get_color() == PieceColor::White {
                    self.castling_rights[0] = false;
                    self.castling_rights[1] = false;
                }
                else {
                    self.castling_rights[2] = false;
                    self.castling_rights[3] = false;
                }
            }

            PieceType::Rook => {
                match Square::from_u32(from as u32) {
                    Square::H1 => {
                        self.castling_rights[0] = false;
                    }
                    Square::A1 => {
                        self.castling_rights[1] = false;
                    }
                    Square::H8 => {
                        self.castling_rights[2] = false;
                    }
                    Square::A8 => {
                        self.castling_rights[3] = false;
                    }
                    _ => {}
                }
            }

            _ => {}
        }

        let hash = self.create_zobrist_hash();
        self.repetitions.entry(hash)
            .and_modify(|h| *h += 1)
            .or_insert(1);
    }

    pub fn unmake_move(&mut self) -> Option<Move> {
        if self.move_history.is_empty() { return None; }

        let hash = self.create_zobrist_hash();
        self.repetitions.entry(hash)
            .and_modify(|h| *h -= 1);

        let move_made = self.move_history.pop().unwrap();

        // Undo capture
        let mut moving_piece = self.set_piece(move_made.board_move.get_to_idx(), move_made.captured);
        
        // Do flags
        match move_made.board_move.get_flag() {
            MoveFlag::EnPassant => { 
                let en_passant_dir = if moving_piece.get_color() == PieceColor::Black { 8 } else { -8 };
                let captured_square = move_made.board_move.get_to_idx() + en_passant_dir;

                // Correctly undo capture
                self.set_piece(move_made.board_move.get_to_idx(), Piece::new(0));
                self.set_piece(captured_square, move_made.captured);
            }
            MoveFlag::Castle => {
                match Square::from_u32(move_made.board_move.get_to_idx() as u32) {
                    // White king side
                    Square::G1 => {
                        let rook = self.set_piece(Square::F1 as i32, Piece::new(0));
                        self.set_piece(Square::H1 as i32, rook);
                    }
                    // White queen side
                    Square::C1 => {
                        let rook = self.set_piece(Square::D1 as i32, Piece::new(0));
                        self.set_piece(Square::A1 as i32, rook);
                    }

                    // Black king side
                    Square::G8 => {
                        let rook = self.set_piece(Square::F8 as i32, Piece::new(0));
                        self.set_piece(Square::H8 as i32, rook);
                    }
                    // Black queen side
                    Square::C8 => {
                        let rook = self.set_piece(Square::D8 as i32, Piece::new(0));
                        self.set_piece(Square::A8 as i32, rook);
                    }

                    _ => { panic!("huh????? {:?}", move_made); }
                }
            }
            MoveFlag::PromoteQueen | MoveFlag::PromoteRook | MoveFlag::PromoteBishop | MoveFlag::PromoteKnight => { moving_piece.set_piece(PieceType::Pawn); }
            _ => { }
        }


        self.set_piece(move_made.board_move.get_from_idx(), moving_piece);

        /* Board flags */
        self.en_passant = move_made.en_passant_square;
        self.castling_rights = move_made.castling;
        self.half_move = move_made.half_move;
        self.turn.flip();
        if self.turn == PieceColor::Black { 
            self.full_move -= 1;
        }

        Some(move_made.board_move)
    }

    pub fn get_pieces_for(&self, color: PieceColor) -> &[i32] {
        return if color == PieceColor::White { &self.white_pieces } else { &self.black_pieces }
    }

    pub fn get_pieces_for_mut(&mut self, color: PieceColor) -> &mut [i32] {
        return if color == PieceColor::White { &mut self.white_pieces } else { &mut self.black_pieces }
    }

    // returns the piece that was on the square before
    pub fn set_piece(&mut self, square: i32, piece: Piece) -> Piece {
        // Remove the captured piece from all bitboards
        let captured = self.board[square as usize];
        if !captured.is_none() {
            self.remove_from_bitboards(captured, square);
        }

        // Add the new piece to all bitboards
        self.board[square as usize] = piece;
        if !piece.is_none() {
            self.add_to_bitboards(piece, square);
        }
        captured
    }

    pub fn get_piece(&self, square: i32) -> Piece {
        self.board[square as usize]
    }

    pub fn set_turn(&mut self, turn: PieceColor) { self.turn = turn; }
    pub fn get_turn(&self) -> PieceColor { self.turn }

    #[inline(always)]
    pub fn new() -> Self {
        let mut x = Self {
            board: [Piece::new(0x0); 64],
            bitboards: [BitBoard::new(0); 12],
            side_bitboards: [BitBoard::new(0); 2],

            white_pieces : [-1; 16],
            white_pawns  : [-1; 8],
            white_knights: [-1; 10],
            white_bishops: [-1; 10],
            white_rooks  : [-1; 10],
            white_queens : [-1; 9],
            white_kings  : [-1; 1],
            
            black_pieces : [-1; 16],
            black_pawns  : [-1; 8],
            black_knights: [-1; 10],
            black_bishops: [-1; 10],
            black_rooks  : [-1; 10],
            black_queens : [-1; 9],
            black_kings  : [-1; 1],

            turn: PieceColor::White,
            en_passant: -1,
            castling_rights: [true; 4],
            half_move: 0,
            full_move: 1,

            repetitions: HashMap::new(),
            move_history: vec![]
        };
        x.new_game();
        x
    }

    fn remove_from_bitboards(&mut self, piece: Piece, square: i32) {
        assert!(!piece.is_none());

        // Bitboard
        self.bitboards[piece.get_piece_index()].clear_bit(square);
        self.side_bitboards[piece.get_color() as usize].clear_bit(square);
        
        // Piece Arrays
        for e in self.get_array_for_piece(piece) {
            if e == &square {
                *e = -1;
                break;
            }
        }

        // Whole piece array
        for e in self.get_pieces_for_mut(piece.get_color()) {
            if e == &square {
                *e = -1;
                return;
            }
        }

        panic!("not found!");
    }

    fn get_side_mask(&self, side: PieceColor) -> u64 {
        self.side_bitboards[side as usize].get_bits()
    }

    fn add_to_bitboards(&mut self, piece: Piece, square: i32) {
        // Bitboard
        self.bitboards[piece.get_piece_index()].set_bit(square);
        self.side_bitboards[piece.get_color() as usize].set_bit(square);

        // Piece arrays
        let array = self.get_array_for_piece(piece);
        for e in array {
            if *e == -1  {
                *e = square;
                break;
            }
        }

        // Whole piece array
        for e in self.get_pieces_for_mut(piece.get_color()) {
            if *e == -1  {
                *e = square;
                return;
            }
        }

        panic!("no available slot! piece: {} square: {}", piece, square);
    }

    pub fn get_array_for_piece(&mut self, piece: Piece) -> &mut [i32] {
        match piece.get_piece_type() {
            PieceType::Pawn => {
                if piece.is_white() { &mut self.white_pawns } else { &mut self.black_pawns }
            }
            PieceType::Knight => {
                if piece.is_white() { &mut self.white_knights } else { &mut self.black_knights }
            }
            PieceType::Bishop => {
                if piece.is_white() { &mut self.white_bishops } else { &mut self.black_bishops}
            }
            PieceType::Rook => {
                if piece.is_white() { &mut self.white_rooks } else { &mut self.black_rooks }
            }
            PieceType::Queen => {
                if piece.is_white() { &mut self.white_queens } else { &mut self.black_queens }
            }
            PieceType::King => {
                if piece.is_white() { &mut self.white_kings } else { &mut self.black_kings }
            }
            _ => { panic!("{:?}", piece); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::fen::{STARTPOS_FEN, STARTPOS_FEN_BLACK};
    use super::super::board_helper::BoardHelper;

    const TEST_PROMOTION_FEN: &str = "4k3/2P5/4K3/8/8/8/5p2/8 b - - 0 1";
    
    /* MakeMove Tests */
    #[test]
    fn test_chessboard_make_move_queen_promotion() {
        let mut board = ChessBoard::new();
        board.parse_fen(TEST_PROMOTION_FEN);

        board.make_move(Move::from_uci("f2f1q"), false);
        let piece = board.get_piece(BoardHelper::text_to_square("f1"));
        assert_eq!(piece.get_piece_type(), PieceType::Queen);
    }

    #[test]
    fn test_chessboard_make_move_rook_promotion() {
        let mut board = ChessBoard::new();
        board.parse_fen(TEST_PROMOTION_FEN);
        board.set_turn(PieceColor::White);

        board.make_move(Move::from_uci("f2f1r"), false);
        let piece = board.get_piece(BoardHelper::text_to_square("f1"));
        assert_eq!(piece.get_piece_type(), PieceType::Rook);
    }

    #[test]
    fn test_chessboard_make_move_bishop_promotion() {
        let mut board = ChessBoard::new();
        board.parse_fen(TEST_PROMOTION_FEN);

        board.make_move(Move::from_uci("f2f1b"), false);
        let piece = board.get_piece(BoardHelper::text_to_square("f1"));
        assert_eq!(piece.get_piece_type(), PieceType::Bishop);
    }

    #[test]
    fn test_chessboard_make_move_knight_promotion() {
        let mut board = ChessBoard::new();
        board.parse_fen(TEST_PROMOTION_FEN);

        board.make_move(Move::from_uci("f2f1n"), false);
        let piece = board.get_piece(BoardHelper::text_to_square("f1"));
        assert_eq!(piece.get_piece_type(), PieceType::Knight);
    }

    #[test]
    fn test_chessboard_make_move_pawn_2_up() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/6p1/8/5P2/8/8/8/4K3 b - - 0 1");
        board.make_move_uci("g7g5").unwrap();
        assert_eq!(board.en_passant, BoardHelper::text_to_square("g6"));
    }

    #[test]
    fn test_chessboard_make_move_en_passant_basic_white() {
        let mut board = ChessBoard::new();
        board.parse_fen("4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1");
        board.make_move_uci("f5g6").unwrap();

        assert_eq!(board.en_passant, -1);
        assert_eq!(board.get_piece(BoardHelper::text_to_square("g5")).is_none(), true); // Captured
    }

    #[test]
    fn test_chessboard_make_move_en_passant_basic_black() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/8/8/8/3pP3/k6K/8/8 b - e3 0 1");
        board.make_move_uci("d4e3").unwrap();

        assert_eq!(board.en_passant, -1);
        assert_eq!(board.get_piece(BoardHelper::text_to_square("e4")).is_none(), true); // Captured
    }

    /* UnMakeMove Tests */

    fn _test_unmake_move(fen: &str, uci_move: &str) {
        let mut board = ChessBoard::new();
        board.parse_fen(fen);

        let copy = board.clone();
        board.make_move_uci(uci_move);
        board.unmake_move();
        
        assert_eq!(board, copy, "\n\n\nexpected\n{}\n---------------------------\n got\n{}\n", copy, board);
    }

    #[test]
    fn test_chessboard_unmake_move_basic_1() {
        _test_unmake_move(STARTPOS_FEN, "e2e4");
    }

    #[test]
    fn test_chessboard_unmake_move_basic_2() {
        _test_unmake_move(STARTPOS_FEN_BLACK, "e7e5");
    }

    #[test]
    fn test_chessboard_unmake_move_basic_3() {
        _test_unmake_move(STARTPOS_FEN, "b1a3");
    }

    #[test]
    fn test_chessboard_unmake_move_en_passant_basic_white() {
        _test_unmake_move("4k3/8/8/5Pp1/8/8/8/4K3 w - g6 0 1", "f5g6");
    }

    #[test]
    fn test_chessboard_unmake_move_en_passant_basic_black() {
        _test_unmake_move("8/8/8/8/3pP3/k6K/8/8 b - e3 0 1", "d4e3");
    }

    #[test]
    fn test_chessboard_make_move_castle_white() {
        _test_unmake_move("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1g1");
        _test_unmake_move("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1c1");
    }

    #[test]
    fn test_chessboard_make_move_castle_black() {
        _test_unmake_move("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1", "e8g8");
        _test_unmake_move("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1", "e8c8");
    }

    #[test]
    fn test_chessboard_unmake_move_queen_promotion() {
        _test_unmake_move(TEST_PROMOTION_FEN, "f2f1q");
    }

    #[test]
    fn test_chessboard_unmake_move_rook_promotion() {
        _test_unmake_move(TEST_PROMOTION_FEN, "f2f1r");
    }

    #[test]
    fn test_chessboard_unmake_move_bishop_promotion() {
        _test_unmake_move(TEST_PROMOTION_FEN, "f2f1b");
    }

    #[test]
    fn test_chessboard_unmake_move_knight_promotion() {
        _test_unmake_move(TEST_PROMOTION_FEN, "f2f1n");
    }
}
