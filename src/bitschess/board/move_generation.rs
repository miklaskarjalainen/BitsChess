use super::ChessBoard;

use crate::bitschess::bitboard::{PAWN_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS};
use crate::bitschess::board::magics::{get_bishop_magic, get_rook_magic};

use crate::board_helper::{BoardHelper, Square};
use crate::chess_move::{Move,MoveFlag};
use crate::piece::{PieceColor, PieceType};

impl ChessBoard {
    #[inline(always)]
    pub const fn is_king_in_check(&self, king_color: PieceColor) -> bool {
        let king_square = self.get_king_square(king_color);
        self.is_square_in_check(king_color, king_square)
    }

    // https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)
    pub const fn is_square_in_check(&self, king_color: PieceColor, square: i32) -> bool {
        const ENEMY_BITBOARD: [usize; 2] = [6, 0];
        let enemy_bitboard_idx = ENEMY_BITBOARD[king_color as usize];
        let all_pieces = self.side_bitboards[0] | self.side_bitboards[1];
        
        let pawn_checks   = PAWN_ATTACKS[king_color as usize][square as usize] & self.bitboards[enemy_bitboard_idx];
        let knight_checks = KNIGHT_ATTACKS[square as usize] & self.bitboards[enemy_bitboard_idx+1];
        let king_checks = KING_ATTACKS[square as usize] & self.bitboards[enemy_bitboard_idx+5];

        let bishop_checks = get_bishop_magic(square, all_pieces) & (self.bitboards[enemy_bitboard_idx+2] | self.bitboards[enemy_bitboard_idx+4]);
        let rook_checks   = get_rook_magic(square, all_pieces) & (self.bitboards[enemy_bitboard_idx+3] | self.bitboards[enemy_bitboard_idx+4]);

        (pawn_checks | knight_checks | bishop_checks | rook_checks | king_checks) != 0
    }
}

pub struct MoveGenerator;

impl MoveGenerator {
    #[inline(always)]
    fn generate_moves(from: i32, mut move_mask: u64, out_moves: &mut Vec<Move>) {
        while move_mask != 0 {
            let square_to = BoardHelper::bitscan_forward(move_mask);
            out_moves.push(Move::new(from, square_to, MoveFlag::None));
            move_mask ^= 1u64 << square_to;
        }
    }

    #[inline(always)]
    fn generate_moves_promotion(from: i32, mut move_mask: u64, out_moves: &mut Vec<Move>, is_quiet: bool) {
        while move_mask != 0 {
            let square_to = BoardHelper::pop_lsb(&mut move_mask);
            if is_quiet {
                out_moves.push(Move::new(from, square_to, MoveFlag::PromoteKnight));
                out_moves.push(Move::new(from, square_to, MoveFlag::PromoteBishop));
                out_moves.push(Move::new(from, square_to, MoveFlag::PromoteRook));
            }
            out_moves.push(Move::new(from, square_to, MoveFlag::PromoteQueen));
        }
    }

    /// if generate_quiet == false then moves which doesn't either capture or promote to a queen won't be generated.
    pub fn get_legal_moves(board: &ChessBoard, generate_quiet: bool) -> Vec<Move> {
        use crate::bitschess::bitboard;
        let color_idx = board.turn as usize;
        let enemy_bitboard_idx = board.turn.flipped() as usize;

        // 
        let attack_mask = Self::get_attack_mask(board);

        let friendly_pieces = board.side_bitboards[color_idx];
        let enemy_pieces = board.side_bitboards[enemy_bitboard_idx];
        let all_pieces = friendly_pieces | enemy_pieces;
        let enemy_or_empty = (!0u64) ^ friendly_pieces;
        let move_filter_mask = if generate_quiet { !0u64 } else { enemy_pieces };

        let (pin_hv, pin_d12) = Self::get_pinned_mask(board);
        let pin_mask = pin_hv | pin_d12;
        let mut moves = Vec::<Move>::with_capacity(32);
        let mut check_mask = !0u64;

        // King 
        let king_square = board.get_king_square(board.turn);
        let king_moves = KING_ATTACKS[king_square as usize] & !attack_mask & !friendly_pieces & move_filter_mask;
        Self::generate_moves(king_square, king_moves, &mut moves);

        let king_attacked_mask = attack_mask & (1u64 << king_square);
        if king_attacked_mask != 0 {            
            let double_check;
            (double_check, check_mask) = Self::get_check_mask(board);

            // In double check, only king is allowed to move.
            if double_check {
                return moves;
            }
        }
        else if generate_quiet {
            // Castling
            let rights_idx = (color_idx) * 2;
            let rooks = board.bitboards[PieceType::Rook.get_side_index(board.turn)];
            let square_for_black = (color_idx as i32) * 56;

            // King Side
            if board.castling_rights[rights_idx] {
                const ROOK_LOCATION_MASK: [u64; 2] = [1u64 << (Square::H1 as u64), 1u64 << (Square::H8 as u64)];
                const EMPTY_SQUARES: [u64; 2] = [0b1100000, 0b1100000 << (7*8)];

                let are_empty = all_pieces & EMPTY_SQUARES[color_idx] == 0;
                let are_attacked = attack_mask & EMPTY_SQUARES[color_idx] != 0;
                let rook_in_place = rooks & ROOK_LOCATION_MASK[color_idx] != 0;
                if are_empty && !are_attacked && rook_in_place {
                    moves.push(Move::new((Square::E1 as i32) + square_for_black, (Square::G1 as i32) + square_for_black, MoveFlag::Castle));
                }
            }

            // Queen Side
            if board.castling_rights[rights_idx+1] {
                const ROOK_LOCATION_MASK: [u64; 2] = [1u64 << (Square::A1 as u64), 1u64 << (Square::A8 as u64)];
                const EMPTY_SQUARES: [u64; 2] = [0b1110, 0b1110 << (7*8)];
                const NON_ATTACKED_MASK: [u64; 2] = [0b1100, 0b1100 << (7*8)];

                let are_empty = all_pieces & EMPTY_SQUARES[color_idx] == 0;
                let are_attacked = attack_mask & NON_ATTACKED_MASK[color_idx] != 0;
                let rook_in_place = rooks & ROOK_LOCATION_MASK[color_idx] != 0;
                if are_empty && !are_attacked && rook_in_place {
                    moves.push(Move::new((Square::E1 as i32) + square_for_black, (Square::C1 as i32) + square_for_black, MoveFlag::Castle));
                }
            }
        }

        // Knights
        let mut knights = board.bitboards[PieceType::Knight.get_side_index(board.turn)];
        while knights != 0 {
            let knight_square = BoardHelper::pop_lsb(&mut knights);
            // Pinned knight cannot move
            if pin_mask & (1 << knight_square) != 0 { continue; } 

            let knight_attacks = bitboard::KNIGHT_ATTACKS[knight_square as usize] & enemy_or_empty & check_mask & move_filter_mask;
            Self::generate_moves(knight_square, knight_attacks, &mut moves);
        } 
        
        // Bishop
        let mut bishops = board.bitboards[PieceType::Bishop.get_side_index(board.turn)] | board.bitboards[PieceType::Queen.get_side_index(board.turn)];
        while bishops != 0 {
            let bishop_square = BoardHelper::pop_lsb(&mut bishops);
            let bishop_attacks = get_bishop_magic(bishop_square, all_pieces) & enemy_or_empty & check_mask & move_filter_mask;
            if pin_mask & (1 << bishop_square) != 0 {
                // For Bishops the pin cannot be by horizontal/vertical moving piece for it be able to move  
                if pin_hv & (1 << bishop_square) == 0 {
                    Self::generate_moves(bishop_square, bishop_attacks & pin_d12, &mut moves);
                }
                continue;
            }
            Self::generate_moves(bishop_square, bishop_attacks, &mut moves);
        } 

        // Rook
        let mut rooks = board.bitboards[PieceType::Rook.get_side_index(board.turn)] | board.bitboards[PieceType::Queen.get_side_index(board.turn)];
        while rooks != 0 {
            let rook_square = BoardHelper::pop_lsb(&mut rooks);
            let rook_attacks = get_rook_magic(rook_square, all_pieces) & enemy_or_empty & check_mask & move_filter_mask;
            if pin_mask & (1 << rook_square) != 0 {
                // For rooks the pin cannot be by diagonal moving piece for it be able to move  
                if pin_d12 & (1 << rook_square) == 0 {
                    Self::generate_moves(rook_square, rook_attacks & pin_hv, &mut moves);
                }
                continue;
            }
            Self::generate_moves(rook_square, rook_attacks, &mut moves);
        }

        // Pawns
        let mut pawns = board.bitboards[PieceType::Pawn.get_side_index(board.turn)];
        while pawns != 0 {
            let pawn_square = BoardHelper::pop_lsb(&mut pawns);

            let mut promotable_moves = 0u64;
            let current_rank = BoardHelper::get_rank(pawn_square);
            
            // Attack
            if pin_mask & (1 << pawn_square) == 0 {
                promotable_moves |= PAWN_ATTACKS[color_idx][pawn_square as usize] & enemy_pieces & check_mask;
            }
            else if pin_d12 & (1 << pawn_square) != 0 && pin_hv & (1 << pawn_square) == 0 {
                promotable_moves |= PAWN_ATTACKS[color_idx][pawn_square as usize] & enemy_pieces & check_mask & pin_d12;
            }

            // Advance by 1
            let move_dir = if board.turn == PieceColor::White{ 8 } else { -8 };
            let move_mask = 1u64 << (pawn_square + move_dir);
            let pin_allowed_to_move = ((pin_hv & (1 << pawn_square) == 0) || (move_mask & pin_hv) != 0) && ((pin_d12 & (1 << pawn_square) == 0) || (move_mask & pin_d12) != 0); // don't allow pawn jumping pin masks
            if generate_quiet && (all_pieces & move_mask) == 0 && pin_allowed_to_move {
                promotable_moves |= (1u64 << (pawn_square + move_dir)) & check_mask;

                // Advance by 2
                // FIXME: only on a different if, because '1u64 << (pawn_square + move_dir*2)' would overflow
                let on_start_rank = if board.turn == PieceColor::White { 1 } else { 6 } == current_rank;
                if on_start_rank {
                    let advance_mask = 1u64 << (pawn_square + move_dir*2);
                    let not_blocked = all_pieces & advance_mask == 0;
                    if on_start_rank && not_blocked && (advance_mask & check_mask) != 0 {
                        moves.push(Move::new(pawn_square, pawn_square + move_dir * 2, MoveFlag::PawnTwoUp));
                    }
                }
            }
            
            // Push promotable_moves
            let promotion_rank = if board.turn == PieceColor::White{ 6 } else { 1 };
            if promotion_rank == current_rank {
                Self::generate_moves_promotion(pawn_square, promotable_moves, &mut moves, generate_quiet);
            }
            else {
                Self::generate_moves(pawn_square, promotable_moves, &mut moves);
            }

            // En Passant
            if board.en_passant != -1 && (pin_mask & (1 << pawn_square) == 0) {
                // check if the attack pattern overlaps the en passant square
                let en_passant_square_mask = 0b1u64 << board.en_passant;

                // If the pawn which moved 2 up is part of the pinned mask
                let pawn_moved_mask = if color_idx == 0 {en_passant_square_mask >> 8} else {en_passant_square_mask << 8};
                let pawn_moved_diag_pinned = pawn_moved_mask & pin_d12 != 0; // only checking diagonal pins allows capturing vertically pinned pieces.
                let en_passant_on_attack = PAWN_ATTACKS[color_idx][pawn_square as usize] & en_passant_square_mask != 0;

                if en_passant_on_attack && !pawn_moved_diag_pinned {
                    
                    // handles this 8/2p5/3p4/KP5r/1R2Pp1k/8/6P1/8 b - e3 0 1
                    if BoardHelper::get_rank(pawn_square) == BoardHelper::get_rank(king_square) {
                        let opp_rq = board.bitboards[PieceType::Rook.get_side_index(board.turn.flipped())] | board.bitboards[PieceType::Queen.get_side_index(board.turn.flipped())];
                        
                        let two_pawn_mask = pawn_moved_mask | (1 << pawn_square);
                        let blockers = all_pieces ^ two_pawn_mask;
                        let rook_attacks = get_rook_magic(king_square, blockers);

                        if rook_attacks & opp_rq == 0 {
                            moves.push(Move::new(pawn_square, board.en_passant, MoveFlag::EnPassant));
                        }
                    }

                    // Allows to en passant a checking pawn
                    else if check_mask & pawn_moved_mask == pawn_moved_mask {
                        moves.push(Move::new(pawn_square, board.en_passant, MoveFlag::EnPassant));
                    }
                }
            }
        }   

        moves
    }

    #[inline(always)]
    pub fn get_legal_moves_for_square(board: &ChessBoard, square: i32) -> Vec<Move> {
        Self::get_legal_moves(board, true).into_iter().filter(|m| {
            m.get_from_idx() == square
        }).collect()
    }

    /// (HorizontalVertical, Diagonal)
    pub fn get_pinned_mask(board: &ChessBoard) -> (u64, u64) {
        let opponent = board.get_turn().flipped();
        let current_turn = board.get_turn();

        let opp_bq = board.bitboards[PieceType::Bishop.get_side_index(opponent)] | board.bitboards[PieceType::Queen.get_side_index(opponent)];
        let opp_rq = board.bitboards[PieceType::Rook  .get_side_index(opponent)] | board.bitboards[PieceType::Queen.get_side_index(opponent)];
        let king_square = board.get_king_square(current_turn);

        let occupied = board.side_bitboards[0] | board.side_bitboards[1];
        let own_pieces = board.side_bitboards[current_turn as usize];
        
        let (mut rook_pins, mut bishop_pins) = (0u64, 0u64);

        // Bishop
        let mut pinner_bq = Self::xray_bishop_attacks(occupied, own_pieces, king_square) & opp_bq;
        while pinner_bq != 0 {
            use super::super::bitboard::BETWEENS;

            let square = BoardHelper::bitscan_forward(pinner_bq);
            bishop_pins |= BETWEENS[square as usize][king_square as usize] | (1 << square);
            pinner_bq &= pinner_bq - 1;
        }

        // Rook
        let mut pinner_rq = Self::xray_rook_attacks(occupied, own_pieces, king_square) & opp_rq;
        while pinner_rq != 0 {
            use super::super::bitboard::BETWEENS;

            let square = BoardHelper::bitscan_forward(pinner_rq);
            rook_pins |= BETWEENS[square as usize][king_square as usize] | (1 << square);
            pinner_rq &= pinner_rq - 1;
        }

        (rook_pins, bishop_pins)
    }

    // (double_check, check_mask)
    pub fn get_check_mask(board: &ChessBoard) -> (bool, u64) {
        let opponent = board.get_turn().flipped();
        let side_king = board.get_turn() as usize;
        
        let king_mask = board.bitboards[side_king * 6 + 5];
        let king_square = board.get_king_square(board.get_turn());
        let blockers = board.side_bitboards[0] | board.side_bitboards[1];

        let mut check_mask = 0u64;

        let mut is_checked = false;
        let mut is_double_check = false;

        // Pawns
        {
            let mut pawns = board.bitboards[PieceType::Pawn.get_side_index(opponent)];
            while pawns != 0 {
                let pawn_square = BoardHelper::pop_lsb(&mut pawns);
                
                let attack = PAWN_ATTACKS[opponent as usize][pawn_square as usize];
                if (attack & king_mask) != 0 {
                    check_mask |= 1 << pawn_square;
                    is_double_check = is_checked;
                    is_checked = true;
                }
            }
        }
 
        // Knights
        {
            let mut knights = board.bitboards[PieceType::Knight.get_side_index(opponent)];
            while knights != 0 {
                let knight_square = BoardHelper::pop_lsb(&mut knights);
                
                let attack = KNIGHT_ATTACKS[knight_square as usize];
                if (attack & king_mask) != 0 {
                    check_mask |= 1 << knight_square;
                    is_double_check = is_checked;
                    is_checked = true;
                }
            }
        }

        // Bishop
        {
            let mut bishops = board.bitboards[PieceType::Bishop.get_side_index(opponent)] | board.bitboards[PieceType::Queen.get_side_index(opponent)];
            while bishops != 0 {
                let bishop_square = BoardHelper::pop_lsb(&mut bishops);
                
                let attack = get_bishop_magic(bishop_square, blockers);
                if (attack & king_mask) != 0 {
                    check_mask |= attack & get_bishop_magic(king_square, blockers);
                    check_mask |= 1 << bishop_square;
                    is_double_check = is_checked;
                    is_checked = true;
                }
            }
        }
            
        // Rooks
        {
            let mut rooks = board.bitboards[PieceType::Rook.get_side_index(opponent)] | board.bitboards[PieceType::Queen.get_side_index(opponent)];
            while rooks != 0 {
                let rook_square = BoardHelper::pop_lsb(&mut rooks);
                
                let attack = get_rook_magic(rook_square, blockers);
                if (attack & king_mask) != 0 {
                    check_mask |= attack & get_rook_magic(king_square, blockers);
                    check_mask |= 1 << rook_square;
                    is_double_check = is_checked;
                    is_checked = true;
                }
            }
        }
        
        (is_double_check, check_mask)
    }
    
    pub fn get_attack_mask(board: &ChessBoard) -> u64 {
        use crate::bitschess::bitboard;
        let king_mask = board.bitboards[board.get_turn() as usize * 6 + 5];
        let enemy_color = board.get_turn().flipped();
        
        // erase king from blockers as well
        let all_pieces = (board.side_bitboards[0] | board.side_bitboards[1]) ^ king_mask;
    
        let mut attacks = 0u64;
            
        {
            let mut pawns = board.bitboards[PieceType::Pawn.get_side_index(enemy_color)];
            while pawns != 0 {
                let pawn_square = BoardHelper::pop_lsb(&mut pawns);
                attacks |= bitboard::PAWN_ATTACKS[enemy_color as usize][pawn_square as usize];
            }
        }

        {
            let mut knights = board.bitboards[PieceType::Knight.get_side_index(enemy_color)];
            while knights != 0 {
                let knight_square = BoardHelper::pop_lsb(&mut knights);
                attacks |= bitboard::KNIGHT_ATTACKS[knight_square as usize];
            }
        }

        {
            let mut bishops = board.bitboards[PieceType::Bishop.get_side_index(enemy_color)] | board.bitboards[PieceType::Queen.get_side_index(enemy_color)];
            while bishops != 0 {
                let bishop_square = BoardHelper::pop_lsb(&mut bishops);
                attacks |= get_bishop_magic(bishop_square, all_pieces);
            }
        }

        {
            let mut rooks = board.bitboards[PieceType::Rook.get_side_index(enemy_color)] | board.bitboards[PieceType::Queen.get_side_index(enemy_color)];
            while rooks != 0 {
                let rook_square = BoardHelper::pop_lsb(&mut rooks);
                attacks |= get_rook_magic(rook_square, all_pieces);
            }
        }
        
        attacks |= KING_ATTACKS[board.get_king_square(enemy_color) as usize];
        attacks
    }

    // https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)#ModifyingOccupancy
    #[inline(always)]
    const fn xray_rook_attacks(occupied: u64, mut blockers: u64, rook_square: i32) -> u64 {
        let attacks = get_rook_magic(rook_square, blockers);
        blockers &= attacks;
        attacks ^ get_rook_magic(rook_square, occupied ^ blockers)
    }

    #[inline(always)]
    const fn xray_bishop_attacks(occupied: u64, mut blockers: u64, bishop_square: i32) -> u64 {
        let attacks = get_bishop_magic(bishop_square, blockers);
        blockers &= attacks;
        attacks ^ get_bishop_magic(bishop_square, occupied ^ blockers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_en_passant_pin() {
        let mut board = ChessBoard::new();
        board.parse_fen("qk6/8/8/3pP3/8/5K2/8/8 w - d6 0 1").expect("valid fen");
        board.make_move_uci("e5d6").unwrap(); 
    }

    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_en_passant_capture_reveal() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/8/8/1kqpP1K1/8/8/8/8 w - d6 0 1").expect("valid fen");
        board.make_move_uci("e5d4").unwrap(); 
    }

    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_en_passant_capture_reveal_black() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/8/8/8/1k1pPQK1/8/8/8 b - e3 0 1").expect("valid fen");
        board.make_move_uci("d4e3").unwrap(); 
    }

    #[test]
    fn test_chess_board_move_generation_en_passant_in_check() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/8/3p4/1Pp4r/1K3p2/6k1/4P1P1/1R6 w - c6 0 3").expect("valid fen");
        board.make_move_uci("b5c6").expect("en passant resolves the check and as such, should be allowed")
    }

    #[test]
    fn test_chess_board_move_generation_en_passant_vertical_pin() {
        let mut board = ChessBoard::new();
        board.parse_fen("r1bqkbnr/ppp1pppp/8/2Pp4/8/8/PPPKPPPP/RNBQ1BNR w kq d6 0 4").expect("valid fen");
        board.make_move_uci("c5d6").expect("en passant captures pinned piece and also resolves the pin so should be allowed")
    }

    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_double_pin_on_rook() {
        let mut board = ChessBoard::new();
        board.parse_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/P2P1RPP/q2Q2K1 w kq - 0 2").expect("valid fen");
        board.make_move_uci("f2f1").unwrap(); 
    }

    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_pawn_jumping_pin_masks() {
        let mut board = ChessBoard::new();
        board.parse_fen("6k1/6p1/8/1r2p2K/4b1P1/P7/8/3q4 w - - 3 49").expect("valid fen");
        board.make_move_uci("g4g5").unwrap(); 
    }

    #[test]
    #[should_panic]
    fn test_chess_board_move_generation_pawn_jumping_pin_masks_capture() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/R4p1k/5rP1/8/1P2Q3/P7/5P2/5K2 b - - 0 52").expect("valid fen");
        board.make_move_uci("f7g6").unwrap(); 
    }
}

