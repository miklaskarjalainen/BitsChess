
use super::ChessBoard;

use crate::chessboard::bitboard::{BitBoard, PAWN_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS};
use crate::chessboard::board_helper::{BoardHelper, Square};
use crate::chessboard::chessmove::{Move,MoveFlag};
use crate::chessboard::piece::{Piece, PieceColor, PieceType};

impl ChessBoard {
    fn filter_legal_moves(&mut self, moves: Vec<Move>) -> Vec<Move> {
        let turn = self.get_turn();
        moves.into_iter().filter(|m| {
            self.make_move(*m, false);
            let is_in_check = self.is_king_in_check(turn);
            self.unmake_move();
            
            !is_in_check
        }).collect()
    } 

    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        let moves = self.get_pseudo_legal_moves();
        self.filter_legal_moves(moves)
    }

    // guaranteed to be legal
    pub fn get_legal_captures(&mut self) -> Vec<Move> {
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
                generated_moves = BitBoard::get_rook_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
            }

            PieceType::Bishop => {
                let blockers = friendly_pieces | enemy_pieces;
                generated_moves = BitBoard::get_bishop_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
            }

            PieceType::Queen => {
                let blockers = friendly_pieces | enemy_pieces;
                generated_moves = BitBoard::get_queen_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
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
                generated_moves = BitBoard::get_rook_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
            }

            PieceType::Bishop => {
                let blockers = self.get_side_mask(piece_color.flipped()) | friendly_pieces;
                generated_moves = BitBoard::get_bishop_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
            }

            PieceType::Queen => {
                let blockers = self.get_side_mask(piece_color.flipped()) | friendly_pieces;
                generated_moves = BitBoard::get_queen_attack_mask(square, BitBoard::new(blockers).get_bits()).get_bits();
            }

            PieceType::King => {
                // Castling
                // TODO: checks, make this more compact
                if !self.is_king_in_check(piece_color) {
                    if piece_color == PieceColor::White {
                        // Kings Side
                        if self.castling_rights[0] {
                            let rook = self.get_piece(Square::H1 as i32);
                            if 
                            rook.get_piece_type() == PieceType::Rook && 
                            rook.get_color() == PieceColor::White && 
                            self.get_piece(Square::G1 as i32).is_none() && !self.is_square_in_check(piece_color, Square::G1 as i32) &&
                            self.get_piece(Square::F1 as i32).is_none() && !self.is_square_in_check(piece_color, Square::F1 as i32) {
                                moves.push(Move::new(square, Square::G1 as i32, MoveFlag::Castle));
                            }
                        }
    
                        // Queen Side
                        if self.castling_rights[1] {
                            let rook = self.get_piece(Square::A1 as i32);
                            if 
                            rook.get_piece_type() == PieceType::Rook && 
                            rook.get_color() == PieceColor::White && 
                            self.get_piece(Square::B1 as i32).is_none() &&
                            self.get_piece(Square::C1 as i32).is_none() && !self.is_square_in_check(piece_color, Square::C1 as i32) &&
                            self.get_piece(Square::D1 as i32).is_none() && !self.is_square_in_check(piece_color, Square::D1 as i32) {
                                moves.push(Move::new(square, Square::C1 as i32, MoveFlag::Castle));
                            }
                        }
                    }
                    else {
                        // Kings Side
                        if self.castling_rights[2] {
                            let rook = self.get_piece(Square::H8 as i32);
                            if 
                            rook.get_piece_type() == PieceType::Rook && 
                            rook.get_color() == PieceColor::Black && 
                            self.get_piece(Square::G8 as i32).is_none() && !self.is_square_in_check(piece_color, Square::G8 as i32) &&
                            self.get_piece(Square::F8 as i32).is_none() && !self.is_square_in_check(piece_color, Square::F8 as i32) {
                                moves.push(Move::new(square, Square::G8 as i32, MoveFlag::Castle));
                            }
                        }
    
                        // Queen Side
                        if self.castling_rights[3] {
                            let rook = self.get_piece(Square::A8 as i32);
                            if 
                            rook.get_piece_type() == PieceType::Rook && 
                            rook.get_color() == PieceColor::Black && 
                            self.get_piece(Square::B8 as i32).is_none() &&
                            self.get_piece(Square::C8 as i32).is_none() && !self.is_square_in_check(piece_color, Square::C8 as i32) &&
                            self.get_piece(Square::D8 as i32).is_none() && !self.is_square_in_check(piece_color, Square::D8 as i32) {
                                moves.push(Move::new(square, Square::C8 as i32, MoveFlag::Castle));
                            }
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

        let bishop_checks = BitBoard::get_bishop_attack_mask(square, all_pieces).get_bits() & (self.bitboards[enemy_bitboard_idx+2].get_bits() | self.bitboards[enemy_bitboard_idx+4].get_bits());
        let rook_checks = BitBoard::get_rook_attack_mask(square, all_pieces).get_bits() & (self.bitboards[enemy_bitboard_idx+3].get_bits() | self.bitboards[enemy_bitboard_idx+4].get_bits());

        (pawn_checks | knight_checks | bishop_checks | rook_checks | king_checks) != 0
    }
}