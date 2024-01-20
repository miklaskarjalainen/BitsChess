use super::ChessBoard;

impl ChessBoard {

    /// https://www.chessprogramming.org/Perft
    pub fn perft(&mut self, depth: u32, print: bool) -> u64 {
        if depth == 0 { 
            return 1u64; 
        }

        let moves = self.get_legal_moves();
        let mut positions = 0u64;
        for m in moves {
            self.make_move(m, true);
            let move_perft = self.perft(depth - 1, false);
            if print {
                println!("{}: {}", m.to_uci(), move_perft);
            }
            positions += move_perft;
            let _ = self.unmake_move();
        }

        if print {
            println!("Positions Searched: {}", positions);
        }

        positions
    }

}


#[cfg(test)]
mod tests {
    /// https://www.chessprogramming.org/Perft_Results

    use super::*;
    use super::super::fen::STARTPOS_FEN;
    
    fn _test_do_perft(fen: &str, depth: u32) -> u64 {
        let mut board = ChessBoard::new();
        board.parse_fen(fen).expect("valid fen");
        board.perft(depth, true)
    }

    #[test]
    fn test_chess_board_perft_startpos_1() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 1), 20);
    }

    #[test]
    fn test_chess_board_perft_startpos_2() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 2), 400);
    }

    #[test]
    fn test_chess_board_perft_startpos_3() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 3), 8902);
    }

    #[test]
    fn test_chess_board_perft_startpos_4() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 4), 197281);
    }

    #[test]
    #[ignore = "SLOW"]
    fn test_chess_board_perft_startpos_5() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 5), 4865609);
    }

    #[test]
    #[ignore = "SLOW"]
    fn test_chess_board_perft_startpos_6() {
        assert_eq!(_test_do_perft(STARTPOS_FEN, 6), 119060324);
    }

    const POSITION_2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    #[test]
    fn test_chess_board_perft_position2_1() {
        assert_eq!(_test_do_perft(POSITION_2, 1), 48);
    }

    #[test]
    fn test_chess_board_perft_position2_2() {
        assert_eq!(_test_do_perft(POSITION_2, 2), 2039);
    }

    #[test]
    fn test_chess_board_perft_position2_3() {
        assert_eq!(_test_do_perft(POSITION_2, 3), 97862);
    }

    #[test]
    fn test_chess_board_perft_position2_4() {
        assert_eq!(_test_do_perft(POSITION_2, 4), 4085603);
    }

    const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    #[test]
    fn test_chess_board_perft_position3_1() {
        assert_eq!(_test_do_perft(POSITION_3, 1), 14);
    }

    #[test]
    fn test_chess_board_perft_position3_2() {
        assert_eq!(_test_do_perft(POSITION_3, 2), 191);
    }

    #[test]
    fn test_chess_board_perft_position3_3() {
        assert_eq!(_test_do_perft(POSITION_3, 3), 2812);
    }

    #[test]
    fn test_chess_board_perft_position3_4() {
        assert_eq!(_test_do_perft(POSITION_3, 4), 43238);
    }

    #[test]
    fn test_chess_board_perft_position3_5() {
        assert_eq!(_test_do_perft(POSITION_3, 5), 674624);
    }

    const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    #[test]
    fn test_chess_board_perft_position4_1() {
        assert_eq!(_test_do_perft(POSITION_4, 1), 6);
    }

    #[test]
    fn test_chess_board_perft_position4_2() {
        assert_eq!(_test_do_perft(POSITION_4, 2), 264);
    }

    #[test]
    fn test_chess_board_perft_position4_3() {
        assert_eq!(_test_do_perft(POSITION_4, 3), 9467);
    }

    #[test]
    fn test_chess_board_perft_position4_4() {
        assert_eq!(_test_do_perft(POSITION_4, 4), 422333);
    }

    const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    #[test]
    fn test_chess_board_perft_position5_1() {
        assert_eq!(_test_do_perft(POSITION_5, 1), 44);
    }

    #[test]
    fn test_chess_board_perft_position5_2() {
        assert_eq!(_test_do_perft(POSITION_5, 2), 1486);
    }

    #[test]
    fn test_chess_board_perft_position5_3() {
        assert_eq!(_test_do_perft(POSITION_5, 3), 62379);
    }

    #[test]
    fn test_chess_board_perft_position5_4() {
        assert_eq!(_test_do_perft(POSITION_5, 4), 2103487);
    }

    const POSITION_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    #[test]
    fn test_chess_board_perft_position6_1() {
        assert_eq!(_test_do_perft(POSITION_6, 1), 46);
    }

    #[test]
    fn test_chess_board_perft_position6_2() {
        assert_eq!(_test_do_perft(POSITION_6, 2), 2079);
    }

    #[test]
    fn test_chess_board_perft_position6_3() {
        assert_eq!(_test_do_perft(POSITION_6, 3), 89890);
    }

    #[test]
    fn test_chess_board_perft_position6_4() {
        assert_eq!(_test_do_perft(POSITION_6, 4), 3894594);
    }
    
    #[test]
    #[ignore = "SLOW"]
    fn test_chess_board_perft_position6_5() {
        assert_eq!(_test_do_perft(POSITION_6, 5), 164075551);
    }


}

