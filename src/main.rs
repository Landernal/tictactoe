mod ia;
mod board;

use std::env;
use std::io;
use board::{Board, TileState};
use regex::Regex;

#[derive(Copy, Clone)]
struct GameSettings
{
    ia_opponent: bool,
}

fn display_welcome() {
    println!("");
    println!("--------------------------------");
    println!("Welcome to ticTACtoe !");
    println!("--------------------------------");
    println!("| Player uses code to select position (ex: A3, C2, B1)");
    println!("| A, B, C being the line, 1, 2, 3 being the column");
    println!("| X begins.");
    println!("");
}

fn display_board(board: board::Board) {    
    println!("{board}");
    println!("\n");
}

// This is written with regex which is a complete insanity given the simplicity of the task
fn parse_user_entry(buffer: &String) -> Result<usize, String> {    
    // We compile the regex on each new turn which is not desired as it is resource consuming and it won't change
    // -> instanciate it once and for all at the program start
    let parse_player_input_re = Regex::new("([A-C])([1-3])")
        .map_err(|err| format!("{err}"))?;
    
    if let Some(caps) = parse_player_input_re.captures(&buffer) {
    
        let line = match caps.get(1).unwrap().as_str() {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => return Err("Invalid line !!".into()),
        };

        let column = match caps.get(2).unwrap().as_str() {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _ => return Err("Invalid column !!".into()),
        };

        return Ok(3 * line + column)
    }
    
    Err("Failed to read user's entry".into())
}

fn get_move_from_human() -> Result<usize, String> {
    let mut buffer : String = String::new();

    io::stdin().read_line(&mut buffer)
        .map_err(|err| format!("Invalid entry from player, error: {err}"))?;

    parse_user_entry(&buffer)
}

fn get_move_from_ia(board: &board::Board, ia_tile_state: TileState) -> Result<usize, String> {
    match ia::get_move(board, ia_tile_state) {
        // index 8 should be derived from board.board but Rust won't allow a runtime defined value
        index @ 0..=8 => Ok(index),
        _ => Err("Invalid index returned by IA engine".into()),
    }
}

fn parse_arguments(args: Vec<String>) -> GameSettings {
    GameSettings { 
        ia_opponent: args.iter().any(|arg| arg.eq("solo"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let game_settings = parse_arguments(args);
    
    let mut board= Board::default();
    
    let mut current_player : TileState = TileState::TileStateCross;
    let mut last_player : TileState = TileState::TileStateEmpty;

    let computer_player = if game_settings.ia_opponent {
        TileState::TileStateRound
    }
    else {
        TileState::TileStateEmpty
    };

    display_welcome();
    
    while board.is_board_full() == false && board.is_line_achieved() == false {
    
        display_board(board);

        let player_move_index = if current_player == computer_player {
            get_move_from_ia(&board, computer_player)
        }
        else {
            get_move_from_human()
        };

        if let Err(error) = player_move_index {
            println!("error: {error}");
            return;
        }
        
        let unwrapped_index = player_move_index.unwrap();
        
        if let Some(tile) = board.board.get(unwrapped_index) {
            if tile.tile_state != TileState::TileStateEmpty {
                println!("This box is already taken, please play again");
                continue;
            }
        }
        else {
            panic!("Invalid board index {unwrapped_index}");
        }
                
        board.board[unwrapped_index].tile_state = current_player;
        
        last_player = current_player;
        if current_player == TileState::TileStateCross {
            current_player = TileState::TileStateRound;
        } else {
            current_player = TileState::TileStateCross;
        } 
    }
    
    display_board(board);

    if board.is_line_achieved() {
        println!("Victory for {last_player} !");
    }
    else {
        println!("It's a draw");
    }
}
