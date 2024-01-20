#![allow(clippy::inline_always)]

mod chessboard;

use chessboard::board::fen::STARTPOS_FEN;
use chessboard::board::ChessBoard;
use chessboard::board_helper::BoardHelper;
use chessboard::bitboard::BitBoard;

fn main() {
    let mut board = ChessBoard::new();
    println!("Welcome to BitChess' interface!");
    
    board.parse_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").expect("valid fen");

    loop {
        let line = std::io::stdin().lines().next().expect("").expect("");
        let args: Vec<&str> = line.split(' ').collect();

        if &line == "exit" {
            break;
        }
        else if &line == "board" {
            println!("{board}");
        }
        else if &line == "pgn" {
            println!("{:?}", board.to_pgn());
            println!("{}", board.to_pgn().to_string());
        }
        else if &line == "undo" {
            let m = board.unmake_move();
            if let Some(made_move) = m {
                println!("undid {}", made_move.to_uci());
            } else {
                println!("nothing to undo");
            }
        }
        else if args.len() == 2 && args[0] == "checked" {
            if let Some(square_str) = args.last() {
                let square = BoardHelper::text_to_square(&square_str[0..2]);
                println!("is {square} checked?: {}", board.is_square_in_check(board.get_turn(), square));
            }
        }
        else if args.len() == 3 && args[0] == "go" && args[1] == "perft" {
            match args.last().expect(":^(").parse::<u32>() {
                Ok(depth) => {
                    let begin = std::time::Instant::now();
                    board.perft(depth, true);
                    let duration = begin.elapsed();

                    println!("perft took: {duration:?}");
                }
                Err(_) => {
                    println!("error while parsing numerical value");
                }
            }
        }
        else if args[0] == "attackmask" {
            use crate::chessboard::board::move_generation::MoveGenerator;
            let atk = MoveGenerator::get_attack_mask(&board);
            println!("{}", BitBoard::new(atk));
        }
        else if args[0] == "checkmask" {
            use crate::chessboard::board::move_generation::MoveGenerator;
            let (double_check, all_pieces) = MoveGenerator::get_check_mask(&board);
            println!("double_check: {double_check}\n{}", BitBoard::new(all_pieces));
        }
        else if args[0] == "pinmask" {
            use crate::chessboard::board::move_generation::MoveGenerator;
            let (hv, d12) = MoveGenerator::get_pinned_mask(&board);

            println!("HorizontalVertical: \n{}", BitBoard::new(hv));
            println!("Diagonal: \n{}", BitBoard::new(d12));
        }
        else if args[0] == "fen" {
            println!("FEN: {}", board.to_fen());
        }
        else if args.len() == 2 && args[0] == "moves" {
            if let Some(square_str) = args.last() {
                let square = BoardHelper::text_to_square(&square_str[0..2]);
                board.print_legal_moves_for_square(square);
            }
        }
        else if &line == "quit" {
            return;
        }
        else if &line == "cpu-ins" {
            println!("Allowed cpu instruction sets:");
            println!("\tAVX={}", cfg!(target_feature = "avx"));
            println!("\tAVX2={}", cfg!(target_feature = "avx2"));
            println!("\tSSE={}", cfg!(target_feature = "sse"));
            println!("\tSSE2={}", cfg!(target_feature = "sse2"));
            println!("\tSSE3={}", cfg!(target_feature = "sse3"));
            println!("\tSSE4.1={}", cfg!(target_feature = "sse4.1"));
            println!("\tSSE4.2={}", cfg!(target_feature = "sse4.2"));
        }
        else if BoardHelper::is_valid_uci_move(&line) && board.make_move_uci(&line).is_some() {}   
        else if board.make_move_pgn(&line).is_some() {
            println!("PGN: made move '{line}'");
        }
        else {
            println!("invalid command :^(");
        }
    }
}
