use super::{ BoardHelper, ChessBoard, PieceType, MoveFlag, ReversibleMove};
use super::fen::STARTPOS_FEN;
use std::collections::HashMap;

// https://en.wikipedia.org/wiki/Portable_Game_Notation
#[derive(Debug)]
pub struct PGN {
    tags: HashMap<String, String>, 
    moves: Vec<String>
}

impl ToString for PGN {
    fn to_string(&self) -> String {
        let mut pgn = String::new();

        // Tags
        for (key, value) in &self.tags {
            pgn.push_str(format!("[{} \"{}\"]\n", key, value).as_str());
        }
        pgn.push('\n');
        
        // Moves
        let mut moves_iter = self.moves.iter().peekable();
        let mut is_white = true;
        let mut full_turns = 1;
        while moves_iter.peek().is_some() {
            let pgn_move = moves_iter.next().unwrap();
            
            if is_white {
                if full_turns % 4 == 0 {
                    pgn.push('\n');
                }
                
                pgn.push_str(format!("{}. {} ", full_turns, pgn_move).as_str());
                full_turns += 1;
            } else {
                pgn.push_str(format!("{} ", pgn_move).as_str());
            }

            is_white = !is_white;
        }

        pgn
    }
}

impl PGN {
    /// Replaces the tag if already set
    #[inline(always)]
    pub fn set_tag(&mut self, tag: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(tag.into(), value.into());
    }

    #[inline(always)]
    pub fn get_tag(&mut self, tag: impl AsRef<String>) -> Option<&String> {
        self.tags.get(tag.as_ref())
    }

    #[inline(always)]
    pub fn del_tag(&mut self, tag: impl AsRef<String>) -> bool {
        self.tags.remove_entry(tag.as_ref()).is_some()
    }
}


impl PGN {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            moves: vec![]
        }
    }
}

impl ChessBoard {

    /// (add_file, add_rank)
    /// https://en.wikipedia.org/wiki/Algebraic_notation_(chess)#Disambiguating_moves
    fn pgn_needs_disambiguating(&self, m: ReversibleMove) -> (bool, bool) {
        use super::magics::{get_bishop_magic, get_rook_magic};
        use super::super::bitboard::KNIGHT_ATTACKS;

        let mut attackers;

        let turn = self.get_turn();
        let own_pieces = self.get_side_mask(turn);
        let blockers = self.get_side_mask(turn.flipped()) | own_pieces;
        let to = m.board_move.get_to_idx();

        let piece = self.get_piece(m.board_move.get_from_idx());
        match piece.get_piece_type() {

            PieceType::Knight => {
                attackers = KNIGHT_ATTACKS[to as usize] & self.bitboards[PieceType::Knight.get_side_index(turn)].get_bits();
            }

            PieceType::Bishop => {
                attackers = get_bishop_magic(to, blockers) & self.bitboards[PieceType::Bishop.get_side_index(turn)].get_bits();
            }

            PieceType::Rook => {
                attackers = get_rook_magic(to, blockers) & self.bitboards[PieceType::Rook.get_side_index(turn)].get_bits();
            }

            PieceType::Queen => {
                attackers = (get_rook_magic(to, blockers) | get_bishop_magic(to, blockers)) & self.bitboards[PieceType::Queen.get_side_index(turn)].get_bits();
            }

            // If a pawn made the capture then file is always needed
            PieceType::Pawn => {
                return (false, !m.captured.is_none());
            }

            _ => { return (false, false); }
        }

        // Pieces which can also move to the same position
        let mut overlapping_pieces = vec![];
        while attackers != 0 {
            let square = BoardHelper::pop_lsb(&mut attackers);
            overlapping_pieces.push(square);
        }

        let mut disambiguate_rank = false;
        let mut disambiguate_file = false;
        for x in &overlapping_pieces {
            for y in &overlapping_pieces {
                if x == y { continue; }

                if disambiguate_file {
                    if BoardHelper::get_rank(*x) == BoardHelper::get_rank(*y) {
                        disambiguate_rank = true;
                        break;
                    }
                }
                else if disambiguate_rank {
                    if BoardHelper::get_file(*x) == BoardHelper::get_file(*y) {
                        disambiguate_file = true;
                        break;
                    }
                }

                if BoardHelper::get_file(*x) == BoardHelper::get_file(*y) {
                    disambiguate_file = true;
                }
                else if BoardHelper::get_rank(*x) == BoardHelper::get_rank(*y) {
                    disambiguate_rank = true;
                }
            }
        }

        (disambiguate_file, disambiguate_rank)
    }

    fn get_move_pgn(&self, m: ReversibleMove) -> String {
        // Castling
        if m.board_move.get_flag() == MoveFlag::Castle {
            let to = m.board_move.get_to_idx();
            match to {
                // King side
                6 | 62 => {
                    return "O-O".to_string();
                }
                // Queen side
                2 | 58 => {
                    return "O-O-O".to_string();
                }
                _ => { panic!("invalid castle to"); }
            }
        }
        
        // if more than 1 pieces of the same type can move to the same location then a starting location is added as prefix for disambiguation.
        // https://en.wikipedia.org/wiki/Algebraic_notation_(chess)#Disambiguating_moves
        let mut dis_amb = "".to_string();
        let (dis_file, dis_rank) = self.pgn_needs_disambiguating(m);
        let (file, rank) = BoardHelper::square_to_chars(m.board_move.get_from_idx());
        if dis_rank {
            dis_amb.push(file);
        }
        if dis_file {
            dis_amb.push(rank);
        }

        let piece = self.get_piece(m.board_move.get_from_idx());
        let captured = if m.captured.is_none() { "" } else { "x" };
        let moving = if piece.get_piece_type() == PieceType::Pawn { "".to_string() } else { piece.get_piece_type().to_char().to_uppercase().to_string() };
        let promotion = match m.board_move.get_flag() {
            MoveFlag::PromoteKnight => { "=N" }
            MoveFlag::PromoteBishop => { "=B" }
            MoveFlag::PromoteRook => { "=R" }
            MoveFlag::PromoteQueen => { "=Q" }
            _ => { "" }
        };
        let to_square = BoardHelper::square_to_string(m.board_move.get_to_idx());

        format!("{}{}{}{}{}", moving, dis_amb, captured, to_square, promotion)
    }

    pub fn to_pgn(&self) -> PGN {
        use chrono::prelude::*;

        let now = Local::now();
        now.day();
        
        let mut pgn = PGN::new();
        
        // Seven tag roster
        pgn.set_tag("Event", "?");
        pgn.set_tag("Site", "?");
        pgn.set_tag("Date", format!("{}.{:0>2}.{:0>2}", now.year(), now.month(), now.day()));
        pgn.set_tag("Round", "?");
        pgn.set_tag("White", "?");
        pgn.set_tag("Black", "?");
        pgn.set_tag("Result", "?");

        // get moves as pgn
        let mut board = self.clone();
        while board.move_history.len() > 0 {
            let reversible_move = board.move_history.last().unwrap().clone();
            let check_or_mate = if board.is_check_mate() { "#" } else if board.is_king_in_check(board.turn) { "+" } else { "" };
            board.unmake_move().unwrap();

            let move_pgn = format!("{}{}", board.get_move_pgn(reversible_move), check_or_mate);
            pgn.moves.push(move_pgn);
        }
        pgn.moves.reverse();

        // Add fen if the position differs from starting position
        let board_fen = board.to_fen();
        if board_fen != STARTPOS_FEN {
            pgn.set_tag("FEN", board_fen);
        }

        pgn
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::fen::STARTPOS_FEN;

    #[test]
    fn test_pgn_pawns_capture() {
        let mut board = ChessBoard::new();
        board.parse_fen("r1bqk2r/ppp1nppp/6n1/2bPp3/5P2/P1N2QP1/1PPB3P/2KR1B1R w kq - 0 14").unwrap();
        board.make_move_uci("f4e5").unwrap();
        
        // pawn captures always include the file which the pawn moved from.
        let mut pgn = board.to_pgn();
        assert_eq!(pgn.moves.pop(), Some(String::from("fxe5")));
    }
    
    #[test]
    fn test_pgn_pawns_promotion_check() {
        let mut board = ChessBoard::new();
        board.parse_fen("8/6Pk/5K2/8/8/8/8/8 w - - 0 1").unwrap();
        board.make_move_uci("g7g8q").unwrap();
        
        let mut pgn = board.to_pgn();
        assert_eq!(pgn.moves.pop(), Some(String::from("g8=Q+")));
    }
    
    #[test]
    fn test_pgn_pawns_promotion_checkmate() {
        let mut board = ChessBoard::new();
        board.parse_fen("2k5/Q4PK1/8/8/8/8/8/8 w - - 0 1").unwrap();
        board.make_move_uci("f7f8q").unwrap();
        
        let mut pgn = board.to_pgn();
        assert_eq!(pgn.moves.pop(), Some(String::from("f8=Q#")));
    }
    
    #[test]
    fn test_pgn_ambiguous() {
        let mut board = ChessBoard::new();
        board.parse_fen("3r3r/2k5/8/R7/4Q2Q/8/1K6/R6Q w - - 0 1").unwrap();

        board.make_move_uci("a1a3").unwrap();
        board.make_move_uci("d8f8").unwrap();
        board.make_move_uci("h4e1").unwrap();

        let mut pgn = board.to_pgn();
        // 3 different queens can move to the same location, from different files and ranks so both are needed 
        assert_eq!(pgn.moves.pop(), Some(String::from("Qh4e1"))); 
        assert_eq!(pgn.moves.pop(), Some(String::from("Rdf8")));
        assert_eq!(pgn.moves.pop(), Some(String::from("R1a3")));
    }
}
