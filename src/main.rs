mod chessboard;

use chessboard::board::fen::STARTPOS_FEN;
use chessboard::board::{ChessBoard, CHESSBOARD_WIDTH};

use crate::chessboard::board_helper::BoardHelper;

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
                    board.perft(depth, true);
                }
                Err(_) => {
                    println!("error while parsing numerical value")
                }
            }
        }
        else if args.len() == 2 && args[0] == "moves" {
            let square = BoardHelper::text_to_square(&args.last().unwrap()[0..2]);
            let moves = board.get_legal_moves_for_square(square);

            let mut str = String::from("");
            for y in (0..=7).rev() {
                str.push('|');
                for x in 0..=7 {
                    str.push(board.get_piece(y * CHESSBOARD_WIDTH + x).to_char());
                    for m in &moves {
                        if m.get_to_idx() == (y*CHESSBOARD_WIDTH+x) {
                            str.pop().unwrap();
                            str.push('*');
                            break;
                        }
                    }
                    str.push('|');
                }
                str.push('\n');
            }

            println!("{}", str);
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
