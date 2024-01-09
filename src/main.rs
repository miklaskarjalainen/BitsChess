mod chessboard;

use chessboard::board::fen::STARTPOS_FEN;
use chessboard::board::{ChessBoard, CHESSBOARD_WIDTH};
use chessboard::board_helper::BoardHelper;
use chessboard::bitboard::*;

fn main() {
    let mut board = ChessBoard::new();
    println!("Welcome to BitChess' interface!");
    
    board.parse_fen(STARTPOS_FEN);

    loop {
        let line = std::io::stdin().lines().next().unwrap().unwrap();
        let args: Vec<&str> = line.split(' ').collect();

        if &line == "exit" {
            break;
        }
        else if &line == "board" {
            println!("{}", board);
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
            let square = BoardHelper::text_to_square(&args.last().unwrap()[0..2]);
            println!("is {} checked?: {}", square, board.is_square_in_check(board.get_turn(), square));
        }
        else if args.len() == 3 && args[0] == "go" && args[1] == "perft" {

            match args.last().expect(":^(").parse::<u32>() {
                Ok(depth) => {
                    let begin = std::time::Instant::now();
                    board.perft(depth, true);
                    let duration = std::time::Instant::now() - begin;

                    println!("perft took: {:?}", duration);
                }
                Err(_) => {
                    println!("error while parsing numerical value")
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
            println!("double_check: {}\n{}", double_check, BitBoard::new(all_pieces));
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
        else if args.len() == 2 && args[0] == "boards" {
            use crate::chessboard::board::magics::ROOK_MASK;
            let square = BoardHelper::text_to_square(&args.last().unwrap()[0..2]);
            
            println!("{}", BitBoard::new(ROOK_MASK[square as usize]));
        }
        else if args.len() == 2 && args[0] == "moves" {
            let square = BoardHelper::text_to_square(&args.last().unwrap()[0..2]);
            board.print_legal_moves_for_square(square);
        }
        else if &line == "quit" {
            return;
        }
        else if BoardHelper::is_valid_uci_move(&line) {
            board.make_move_uci(&line);
        }   
        else {
            println!("invalid command :^(");
        }
    }
}
