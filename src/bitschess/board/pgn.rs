
use super::{ BoardHelper, ChessBoard, PieceType, Move, MoveFlag, ReversibleMove, Square };
use super::fen::STARTPOS_FEN;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum PGNParserError {
    SyntaxError,
}

// https://en.wikipedia.org/wiki/Portable_Game_Notation
#[derive(Debug)]
pub struct Pgn {
    tags: HashMap<String, String>, 
    moves: Vec<String>
}

impl ToString for Pgn {
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

impl Default for Pgn {
    fn default() -> Self {
        Self::new()
    }
}

impl Pgn {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            moves: vec![]
        }
    }

    /// Replaces the tag if already set
    #[allow(dead_code)]
    #[inline(always)]
    pub fn set_tag(&mut self, tag: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(tag.into(), value.into());
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_tag(&mut self, tag: impl AsRef<String>) -> Option<&String> {
        self.tags.get(tag.as_ref())
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn del_tag(&mut self, tag: impl AsRef<String>) -> bool {
        self.tags.remove_entry(tag.as_ref()).is_some()
    }

    #[allow(dead_code)]
    pub fn parse_string(&mut self, contents: &str) {
        self.tags = Self::parse_tags(contents).expect("parse error");
        self.moves = Self::parse_moves(contents).expect("parse error");
    }

    #[allow(dead_code, clippy::unnecessary_wraps)] // TODO: proper error handling
    pub fn parse_tags(contents: &str) -> Result<HashMap<String, String>, PGNParserError> {
        /*
        What we're trying to parse:
        [Event "F/S Return Match"]
        [Site "Belgrade, Serbia JUG"]
        [Date "1992.11.04"]
        [Round "29"]
        [White "Fischer, Robert J."]
        [Black "Spassky, Boris V."]
        [Result "1/2-1/2"]

        (stop here ->) 1. e4 e5 ...
        */
        let mut tags = HashMap::new();

        let mut is_literal = false;
        let mut is_in_tag = false;
        
        let mut key = String::from("");
        let mut working_word = String::from("");

        for c in contents.chars() {
            if c == '[' {
                is_in_tag = true;
                continue;
            }
            else if c == ']' {
                is_in_tag = false;
                continue;
            }

            if !is_in_tag && !c.is_whitespace() {
                break;
            }

            // parsing key
            if key.is_empty() {
                if !c.is_whitespace() {
                    working_word.push(c);
                    continue;
                }
                
                if !working_word.is_empty() {
                    key = working_word.clone();
                    working_word.clear();
                }
                continue;
            }
            
            // parsing value
            match c {
                '"' => {
                    if is_literal {
                        tags.insert(key.clone(), working_word.clone());
                        working_word.clear();
                        key.clear();
                    }
                    
                    is_literal = !is_literal;
                    continue;
                }

                _ => {
                    if c.is_whitespace() && !is_literal {
                        continue;
                    }
                    working_word.push(c);
                }
            }

        }

        Ok(tags)
    }

    // TODO: proper error handling
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_moves(contents: &str) -> Result<Vec<String>, PGNParserError> {
        /*
        What we're trying to parse:
        1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
        11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
        Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
        23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
        hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
        35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
        Nf2 42. g4 Bd3 43. Re6 1/2-1/2
        */

        let pos = contents.rfind(']').unwrap_or(0usize);
        let unescape = &contents[(pos+1)..contents.len()]
            .replace('\"', "")
            .replace('\n', " ");

        Ok(
            unescape.split(' ')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| { !x.is_empty() && !x.contains('.') })
            .map(|x| { String::from(x)} )
            .collect::<Vec<String>>()
        )
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
                attackers = KNIGHT_ATTACKS[to as usize] & self.bitboards[PieceType::Knight.get_side_index(turn)];
            }

            PieceType::Bishop => {
                attackers = get_bishop_magic(to, blockers) & self.bitboards[PieceType::Bishop.get_side_index(turn)];
            }

            PieceType::Rook => {
                attackers = get_rook_magic(to, blockers) & self.bitboards[PieceType::Rook.get_side_index(turn)];
            }

            PieceType::Queen => {
                attackers = (get_rook_magic(to, blockers) | get_bishop_magic(to, blockers)) & self.bitboards[PieceType::Queen.get_side_index(turn)];
            }

            // If a pawn made the capture then file is always needed
            PieceType::Pawn => {
                return (!m.captured.is_none(), false);
            }

            _ => { return (false, false); }
        }

        // Pieces which can also move to the same position
        let mut overlapping_pieces = vec![];
        while attackers != 0 {
            let square = BoardHelper::pop_lsb(&mut attackers);
            overlapping_pieces.push(square);
        }
        if overlapping_pieces.len() < 2 { return (false, false); }


        let mut add_file = false;
        let mut add_rank = false;
        for x in &overlapping_pieces {
            for y in &overlapping_pieces {
                if x == y { continue; }
                if BoardHelper::get_file(*x) == BoardHelper::get_file(*y) {
                    add_rank = true;
                }
                else if BoardHelper::get_rank(*x) == BoardHelper::get_rank(*y) {
                    add_file = true;
                }
            }
        }

        // by default add file if not in the same file nor rank
        // but can move into same location (happens with knights)
        add_file = add_file || !add_rank;

        (add_file, add_rank)
    }

    fn get_move_san(&self, m: ReversibleMove) -> String {
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
        if dis_file {
            dis_amb.push(file);
        }
        if dis_rank {
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

    pub fn to_pgn(&self) -> Pgn {
        use chrono::prelude::*;

        let now = Local::now();
        now.day();
        
        let mut pgn = Pgn::new();
        
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
        while let Some(reversible_move) = board.move_history.last().copied() {
            let check_or_mate = if board.is_check_mate() { "#" } else if board.is_king_in_check(board.turn) { "+" } else { "" };
            board.unmake_move().unwrap();

            let move_pgn = format!("{}{}", board.get_move_san(reversible_move), check_or_mate);
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

    /// Tags are not saved!
    #[allow(dead_code)]
    pub fn parse_pgn(&mut self, pgn_str: &str) {
        let mut pgn = Pgn::new();
        pgn.parse_string(pgn_str);

        println!("parsed moves {:?}", pgn.moves);

        for pgn_m in &pgn.moves {
            if let Some(m) = self.make_move_pgn(pgn_m) {
                println!("executed '{}'->'{}'", pgn_m, m.to_uci());
            }
            else {
                println!("couldn't execute '{}'", pgn_m);
                break;
            }
        }
    }

    /// Gets a LEGAL move from a PGN string
    pub fn get_move_pgn(&mut self, pgn: &str) -> Option<Move> {
        // PGN move examples: 
        // e4      (A pawn moved to 'e4')
        // Ng1     (A Knight moved to 'g1')
        // Qe2xe4+ (Queen moved from e2, captured a piece on e4 whilst putting the opponent's king in check)
        // exd8=Q# (A pawn moved from e file to d8, captured a piece, promoted to a Queen and check mated the opponent).

        if pgn.len() < 2 {
            return None;
        }
        
        // The objective here is to get the destination square, which at the end of the pgn (before the flags).
        // So removing the flags such "#", "=Q", "=N", "+" we guarantee that the destination square is at the end.
        // "Qe2xe4+" -> "Qe2e4"
        let flagless = pgn
        .replace(['#', '+', 'x'], "") // [check make, check, capture]
        .replace("=Q", "") // promote queen
        .replace("=R", "") // promote rook
        .replace("=B", "") // promote bishop
        .replace("=N", ""); // promote knight

        let mut moves = self.get_legal_moves().into_iter();

        // first if check if it's castle
        if flagless == "O-O" || flagless == "0-0" {
            return moves.find(|m| {
                m.get_to_idx() == Square::G1 as i32 || m.get_to_idx() == Square::G8 as i32
            });
        }
        else if flagless == "O-O-O" || flagless == "0-0-0" {
            return moves.find(|m| { 
                m.get_to_idx() == Square::C1 as i32 || m.get_to_idx() == Square::C8 as i32
            });
        }

        // extracting the destination square "Qe2e4" -> "4e2eQ" -> ('4', 'e') -> 28
        let mut flagless_iter = flagless.chars().rev();
        let to_square: i32 = {
            let rank_to_char = flagless_iter.next().unwrap();
            let file_to_char = flagless_iter.next().unwrap();
            BoardHelper::chars_to_square(file_to_char, rank_to_char)
        };
        
        // if the first char is upper like in "Qe2" that means that a queen moved to e2. If there's no uppercase letter it means that a pawn moved.
        let moving_piece = {
            let piece_char = pgn.chars().next().unwrap();
            if piece_char.is_uppercase() { PieceType::from_char(piece_char) } else { PieceType::Pawn }
        };

        // We want get additional information about the file and rank which the piece is moving from if provided.
        // "Qe2xe8" -> "e2", "axb7" -> "a"
        let from_info = {
            let skip_first = if moving_piece != PieceType::Pawn { 1 } else { 0 }; // remove first if moving piece is not a pawn
            &flagless[skip_first..flagless.len()-2]// remove last 2 (destination square)
        };
        
        let mut file_from = -1;
        let mut rank_from = -1;
        for c in from_info.chars() {
            if ('a'..='h').contains(&c) {
                file_from = BoardHelper::file_to_idx(c);
            }
            else if ('1'..='8').contains(&c) {
                rank_from = BoardHelper::rank_to_idx(c);
            }
        }
        let promotion = {
            if pgn.contains("=Q") {
                MoveFlag::PromoteQueen
            }
            else if pgn.contains("=R") {
                MoveFlag::PromoteRook
            }
            else if pgn.contains("=B") {
                MoveFlag::PromoteBishop
            }
            else if pgn.contains("=N") {
                MoveFlag::PromoteKnight
            }
            else {
                MoveFlag::None
            }
        };

        let mut result: Vec<Move> = moves.filter(|m| {
            if m.get_to_idx() != to_square {
                return false;
            }
            
            // Now let's check if the move meets our conditions
            let mut filter = true;
            filter = filter && self.get_piece(m.get_from_idx()).get_piece_type() == moving_piece;
            if file_from != -1 {
                filter = filter && BoardHelper::get_file(m.get_from_idx()) == file_from;
            }
            if rank_from != -1 {
                filter = filter && BoardHelper::get_rank(m.get_from_idx()) == rank_from;
            }
            if promotion != MoveFlag::None {
                filter = filter && m.get_flag() == promotion;
            }
        
            filter
        }).collect();

        // There SHOULD only be 1 move which matches the given conditions.
        if result.len() == 1 {
            return result.pop();
        }
        None
    }

    /// Returns the made move, only does legal moves
    pub fn make_move_pgn(&mut self, pgn: &str) -> Option<Move> {
        let m = self.get_move_pgn(pgn);
        if let Some(chess_move) = m {
            self.make_move(chess_move, false);
        }
        m
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
    fn test_pgn_ambiguous_sliding() {
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

    #[test]
    fn test_pgn_ambiguous_knights() {
        let mut board = ChessBoard::new();
        board.parse_fen("rnbq1rk1/2p1bppp/p2p1n2/1p2p3/3PP3/1BP2N1P/PP3PP1/RNBQR1K1 b - d3 0 10").expect("valid fen");
        board.make_move_uci("f6d7").unwrap();

        let mut pgn = board.to_pgn();
        // 3 different queens can move to the same location, from different files and ranks so both are needed 
        assert_eq!(pgn.moves.pop(), Some(String::from("Nfd7"))); 
    }

    #[test]
    fn test_pgn_parse_moves_simple() {
        const FISCHER_V_SPASSKY: &str = "
        1. e4 e5 2. Nf3 Nc6 3. Bb5 a6
        4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
        11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
        Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
        23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
        hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
        35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
        Nf2 42. g4 Bd3 43. Re6
        ";

        let mut board = ChessBoard::new();
        board.parse_fen(STARTPOS_FEN).unwrap();
        board.parse_pgn(FISCHER_V_SPASSKY.into());
        assert_eq!(board.to_fen(), "8/8/4R1p1/2k3p1/1p4P1/1P1b1P2/3K1n2/8 b - - 2 43");
    }
}
