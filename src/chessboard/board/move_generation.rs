
use super::ChessBoard;

use crate::chessboard::bitboard::{BitBoard, PAWN_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS};
use crate::chessboard::board_helper::{BoardHelper, Square};
use crate::chessboard::chessmove::{Move,MoveFlag};
use crate::chessboard::piece::{Piece, PieceColor, PieceType};
use super::magics::{get_bishop_magic, get_rook_magic};

impl ChessBoard {
    fn filter_legal_moves(&mut self, moves: Vec<Move>) -> Vec<Move> {
        let turn = self.get_turn();
        moves.into_iter().filter(|m| {
            self.make_move(*m, true);
            let is_in_check = self.is_king_in_check(turn);
            self.unmake_move();
            
            !is_in_check
        }).collect()
    } 

    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        if self.is_draw() { return vec![]; }

        let moves = self.get_pseudo_legal_moves();
        self.filter_legal_moves(moves)
    }

    // guaranteed to be legal
    pub fn get_legal_captures(&mut self) -> Vec<Move> {
        if self.is_draw() { return vec![]; }

        let mut moves = vec![];

        let pieces = if self.turn == PieceColor::White {self.white_pieces} else {self.black_pieces};
        for square in pieces {
            if square == -1 {
                continue;
            }

            let mut sqrt_moves = self.get_captures_for_square(square);
            moves.append(&mut sqrt_moves);
        }
        self.filter_legal_moves(moves)
    }

    pub fn get_pseudo_legal_moves(&mut self) -> Vec<Move> {
        let mut moves = vec![];

        let pieces = if self.turn == PieceColor::White {self.white_pieces} else {self.black_pieces};
        for square in pieces {
            if square == -1 {
                continue;
            }

            let mut sqrt_moves = self.get_pseudo_legal_moves_for_square(square);
            moves.append(&mut sqrt_moves);
        }
        
        moves
    }

    pub fn get_legal_moves_for_square(&mut self, square: i32) -> Vec<Move> {
        if self.is_draw() { return vec![]; }

        let moves = self.get_pseudo_legal_moves_for_square(square);
        self.filter_legal_moves(moves)
    }

    // not guaranteed to be legal
    pub fn get_captures_for_square(&mut self, square: i32) -> Vec<Move> {
        let piece = self.get_piece(square);
        let piece_color = piece.get_color();

        let friendly_pieces = self.get_side_mask(piece_color);
        let enemy_pieces = self.get_side_mask(piece_color.flipped());

        let mut generated_moves = 0;
        match piece.get_piece_type() {
            PieceType::Pawn => {
                generated_moves |= PAWN_ATTACKS[piece_color as usize][square as usize]
            }

            PieceType::Knight => {
                generated_moves = KNIGHT_ATTACKS[square as usize];
            }

            PieceType::Rook => {
                let blockers = friendly_pieces | enemy_pieces;
                generated_moves = get_rook_magic(square, blockers);
            }

            PieceType::Bishop => {
                let blockers = friendly_pieces | enemy_pieces;
                generated_moves = get_bishop_magic(square, blockers);
            }

            PieceType::Queen => {
                let blockers = friendly_pieces | enemy_pieces;
                generated_moves = get_rook_magic(square, blockers) | get_bishop_magic(square, blockers);
            }

            PieceType::King => {
                generated_moves = KING_ATTACKS[square as usize];
            }

            _ => { 
                return vec![];
            }
        }

        //
        generated_moves &= enemy_pieces;

        let mut moves = vec![];
        self.bitboard_to_moves(generated_moves, square, piece, &mut moves);
        moves
    }

    fn get_pseudo_legal_moves_for_square(&mut self, square: i32) -> Vec<Move> {
        let piece = self.get_piece(square);
        let piece_color = piece.get_color();
        let mut moves = vec![];
        if self.turn != piece_color {
            return moves;
        }

        let mut generated_moves = 0;
        let friendly_pieces = self.get_side_mask(piece_color);
        
        match piece.get_piece_type() {
            PieceType::Pawn => {
                let enemies = self.get_side_mask(piece_color.flipped());
                let blockers = enemies | friendly_pieces;

                let atk_dir = if piece.is_white() { 8 } else { -8 };
                let start_rank = if piece.is_white() { 1 } else { 6 };

                // Advance by 1
                if (blockers & (1 << (square + atk_dir))) == 0 {
                    generated_moves |= 1 << (square + atk_dir);

                    // Advance by 2
                    if BoardHelper::get_rank(square) == start_rank && ((blockers & (1 << (square + atk_dir * 2))) == 0) {
                        moves.push(Move::new(square, square + atk_dir* 2, MoveFlag::PawnTwoUp));
                    }
                }

                // En Passant
                let attack_mask = PAWN_ATTACKS[piece_color as usize][square as usize];
                if self.en_passant != -1 {
                    // check if the attack pattern overlaps the en passant square
                    let en_passant_square_mask = 0b1u64 << self.en_passant;
                    if attack_mask & en_passant_square_mask != 0 {
                        moves.push(Move::new(square, self.en_passant, MoveFlag::EnPassant));
                    }
                }
                
                // Capturing
                generated_moves |= attack_mask & enemies;
            }

            PieceType::Knight => {
                generated_moves = KNIGHT_ATTACKS[square as usize];
            }

            PieceType::Rook => {
                let blockers = self.get_side_mask(piece_color.flipped()) | friendly_pieces;
                generated_moves = get_rook_magic(square, blockers);
            }

            PieceType::Bishop => {
                let blockers = self.get_side_mask(piece_color.flipped()) | friendly_pieces;
                generated_moves = get_bishop_magic(square, blockers);
            }

            PieceType::Queen => {
                let blockers = self.get_side_mask(piece_color.flipped()) | friendly_pieces;
                generated_moves = get_rook_magic(square, blockers) | get_bishop_magic(square, blockers);
            }

            PieceType::King => {
                // Castling
                // TODO: checks, make this more compact
                if !self.is_king_in_check(piece_color) {
                    const KS: [i32; 4] = [Square::H1 as i32, Square::G1 as i32, Square::F1 as i32, Square::G1 as i32];
                    const QS: [i32; 5] = [Square::A1 as i32, Square::B1 as i32, Square::C1 as i32, Square::D1 as i32, Square::C1 as i32];

                    let add_rights_black = (piece_color as usize) * 2;
                    let square_for_black = (piece_color as i32) * 56; // A1 + 56 = A8

                    // King Side
                    if self.castling_rights[add_rights_black] {
                        let rook = self.get_piece(KS[0] + square_for_black);
                        if 
                        rook.get_piece_type() == PieceType::Rook && 
                        rook.get_color() == piece_color && 
                        self.get_piece(KS[1] + square_for_black).is_none() && !self.is_square_in_check(piece_color, KS[1] + square_for_black) &&
                        self.get_piece(KS[2] + square_for_black).is_none() && !self.is_square_in_check(piece_color, KS[2] + square_for_black) {
                            moves.push(Move::new(square, KS[3] + square_for_black, MoveFlag::Castle));
                        }
                    }

                    // Queen Side
                    if self.castling_rights[add_rights_black + 1] {
                        let rook = self.get_piece(QS[0] + square_for_black);
                        if 
                        rook.get_piece_type() == PieceType::Rook && 
                        rook.get_color() == piece_color && 
                        self.get_piece(QS[1] + square_for_black).is_none() &&
                        self.get_piece(QS[2] + square_for_black).is_none() && !self.is_square_in_check(piece_color, QS[2] + square_for_black) &&
                        self.get_piece(QS[3] + square_for_black).is_none() && !self.is_square_in_check(piece_color, QS[3] + square_for_black) {
                            moves.push(Move::new(square, QS[4] + square_for_black, MoveFlag::Castle));
                        }
                    }
                }

                generated_moves = KING_ATTACKS[square as usize];
            }

            _ => {
                return vec![];
            }
        }
        generated_moves &= generated_moves ^ friendly_pieces;
        
        self.bitboard_to_moves(generated_moves, square, piece, &mut moves);

        moves
    }

    pub fn bitboard_to_moves(&mut self, mut generated_moves: u64, square: i32, piece: Piece, mut moves: &mut Vec<Move>) {
        let normal_add = |moves: &mut Vec<Move>, square:i32, square_to: i32| {
            moves.push(Move::new(square, square_to, MoveFlag::None));
        };
        // accounts promotions
        let pawn_add = |moves: &mut Vec<Move>, square:i32, square_to: i32| {
            let rank = BoardHelper::get_rank(square_to);
            if rank == 0 || rank == 7 {
                moves.push(Move::new(square, square_to, MoveFlag::PromoteQueen));
                moves.push(Move::new(square, square_to, MoveFlag::PromoteRook));
                moves.push(Move::new(square, square_to, MoveFlag::PromoteBishop));
                moves.push(Move::new(square, square_to, MoveFlag::PromoteKnight));
            } else {
                moves.push(Move::new(square, square_to, MoveFlag::None));
            }
        };

        let push_function = if piece.get_piece_type() != PieceType::Pawn { normal_add } else { pawn_add };
        while generated_moves != 0 {
            let square_to = BoardHelper::bitscan_forward(generated_moves);
            push_function(&mut moves, square, square_to);
            
            // finished this bit
            generated_moves ^= 1u64 << square_to;
        }
    }

    pub fn is_king_in_check(&self, king_color: PieceColor) -> bool {
        let king_square = if king_color == PieceColor::White { self.white_kings[0] } else { self.black_kings[0] };
        self.is_square_in_check(king_color, king_square)
    }

    // https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)
    pub fn is_square_in_check(&self, king_color: PieceColor, square: i32) -> bool {
        const ENEMY_BITBOARD: [usize; 2] = [6, 0];
        let enemy_bitboard_idx = ENEMY_BITBOARD[king_color as usize];
        let all_pieces = self.side_bitboards[0].get_bits() | self.side_bitboards[1].get_bits();
        
        let pawn_checks   = PAWN_ATTACKS[king_color as usize][square as usize] & self.bitboards[enemy_bitboard_idx].get_bits();
        let knight_checks = KNIGHT_ATTACKS[square as usize] & self.bitboards[enemy_bitboard_idx+1].get_bits();
        let king_checks = KING_ATTACKS[square as usize] & self.bitboards[enemy_bitboard_idx+5].get_bits();

        let bishop_checks = get_bishop_magic(square, all_pieces) & (self.bitboards[enemy_bitboard_idx+2].get_bits() | self.bitboards[enemy_bitboard_idx+4].get_bits());
        let rook_checks   = get_rook_magic(square, all_pieces) & (self.bitboards[enemy_bitboard_idx+3].get_bits() | self.bitboards[enemy_bitboard_idx+4].get_bits());

        (pawn_checks | knight_checks | bishop_checks | rook_checks | king_checks) != 0
    }
}