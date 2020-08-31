mod game;

use std::io::{self, Write};
use std::process;   // exit function
use game::{Game, Piece, Winner, Tiles, MoveError};

#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);

fn main() {
    let mut game = Game::new();

    while !game.is_finished(){
        print_tiles(game.tiles());

        println!("Current piece: {}",match game.current_piece(){
            Piece::X => "x",
            Piece::O => "o",
        });

        let(row,col) = prompt_move();

        match game.make_move(row,col){
            Ok(()) => {},
            Err(MoveError::GameAlreadyOver) => unreachable!("Game was already endded."),
            Err(MoveError::InvalidPosition{row,col}) => {
                unreachable!("Should not be able to enter an invalid move, but still got ({}, {})", row, col)
            },
            Err(MoveError::TileNotEmpty {other_piece, row, col}) => eprintln!(
                "The title at position {}{} already has pieced {} in it",
                row+1, 
                (b'A' + col as u8) as char,  
                match other_piece {
                    Piece::X => "x",
                    Piece::O => "o",
                },
            ),
        }
    }
    print_tiles(game.tiles());

    match game.winner().expect("finished game should have winner"){
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }

    fn prompt_move() -> (usize,usize){
        loop{
            print!("Enter move(e.g. 1A): ");
            io::stdout().flush().expect("Failed to flush stdout");

            let line = read_line();

            match parse_move(&line){
                Ok((row,col)) => break(row,col),
                Err(InvalidMove(invalid_str)) => eprintln!{
                    "invalid move: '{}'.Please try again.",
                    invalid_str,
                },
            }
        }
    }
    fn parse_move(input:&str)->Result<(usize,usize),InvalidMove>{
        if input.len()!= 2 {
            return Err(InvalidMove(input.to_string()));
        } 
        let row = match &input[0..1]{
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _=> return Err(InvalidMove(input.to_string())),
        };
        let col = match &input[1..2]{
            "A" | "a" => 0,
            "B" | "b" => 1,
            "C" | "c" => 2,
            invalid => return Err(InvalidMove(invalid.to_string())),
        };
        Ok((row,col))   // The last line is return value
    }
    fn read_line() -> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.is_empty(){
            println!();
            process::exit(0);
        }
        let len_without_newline = input.trim_end().len();
        input.truncate(len_without_newline);

        input
    }
    fn print_tiles(tiles: &Tiles){
        // The result of this function will be something like the following:
        //   A B C
        // 1 x ▢ ▢
        // 2 ▢ ▢ o
        // 3 ▢ ▢ ▢
        //
        print!{"  "};
        for j in 0..tiles[0].len() as u8 {
            print!(" {}",(b'A' + j) as char);
        }
        println!();
    
        // Now we print each row preceeded by its row number
    // .iter().enumerate() goes through each row and provides a row number with each element using
    // a tuple.
        for (i, row) in tiles.iter().enumerate() {
            print!(" {}", i + 1);
            for tile in row {
                print!(" {}", match *tile {
                    Some(Piece::X) => "x",
                    Some(Piece::O) => "o",
                    None => "\u{25A2}",
                });
            }
         println!();
        }
    }
    // Add an extra line at the end of the board to space it out from the prompts that follow
    println!();
}
