# BitsChess

BitsChess is a chess library written in Rust. It was mainly written for the chess engine [GiffiBot](https://github.com/miklaskarjalainen/GiffiBot) which i'm currently working on, but BitsChess is open for everybody to use under the [MIT license](./LICENSE).  
Behind the scenes BitsChess uses [bitboards](https://www.chessprogramming.org/Bitboards) for [board representation](https://www.chessprogramming.org/Board_Representation). [Attack, check, pin -masks](https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)) and [magic bitboards](https://www.chessprogramming.org/Magic_Bitboards) for [move generation](https://www.chessprogramming.org/Move_Generation).

## Using in a project

### Cargo.toml
Add BitsChess as a dependency into a Cargo.toml
```toml
[dependencies]
bitschess = { git = "https://github.com/miklaskarjalainen/BitsChess.git" }
```

### Using the library
A simple example of using the BitsChess library

```rust
use bitschess::prelude::*;

fn main()  {
    // Creating a board
    let mut board = ChessBoard::new();
    board.parse_fen(STARTPOS_FEN).expect("valid fen");

    // Making moves from uci commands
    board.make_move_uci("e2e4").unwrap();
    board.make_move_uci("a7a5").unwrap();
    board.make_move_uci("e4e5").unwrap();
    board.make_move_uci("d7d5").unwrap();
    board.make_move_uci("e5d6").unwrap(); // en passant
    board.make_move_uci("a5a4").unwrap();
    board.make_move_uci("d6e7").unwrap();
    board.make_move_uci("a4a3").unwrap();
    board.make_move_uci("e7f8r").unwrap(); // promote pawn to a rook

    let legal_moves_uci: Vec<String> = board.get_legal_moves().into_iter().map(|m: Move| {
        m.to_uci()
    }).collect();
    

    println!("{}", board);
    println!("------------------------");
    println!("Legal Moves: {:?}", legal_moves_uci);
}
```

## Compiling as binary
BitsChess can be compiled as a binary, and it has a very primitive CLI which is mainly there for development purposes.  

```bash
git clone https://github.com/miklaskarjalainen/BitsChess
cd BitsChess
cargo build --bin bitschess-bin # add '--release' for optimized builds
```

For the cli commands have look at main.rs they are all in there. They all are subject for change and is the reason they are not listed here (yet). 

## Testing
The repository contains unit tests which can be ran with the command:  
```bash
cargo test # add '--release' for optimized builds
```

## Sources
Sources for the content which helped me to develope this chess library!

https://www.chessprogramming.org/  
https://www.youtube.com/watch?v=U4ogK0MIzqk  
https://www.youtube.com/@chessprogramming591
