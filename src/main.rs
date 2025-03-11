mod ia;
mod board;

use std::env;
use std::io;
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
fn parse_user_entry(buffer: String) -> Result<usize, String> {
    
    let line;
    let column;
    
    // We compile the regex on each new turn which is not desired as it is resource consuming and it won't change
    // -> instanciate it once and for all at the program start
    let parse_player_input_re = Regex::new("([A-C])([1-3])").unwrap();
    match parse_player_input_re.captures(&buffer) {
        Some(caps) => {
            match caps.get(1).unwrap().as_str() {
                "A" => line = 0,
                "B" => line = 1,
                "C" => line = 2,
                _ => return Err(String::from("Unexpected !!"))
            }
            
            match caps.get(2).unwrap().as_str() {
                "1" => column = 0,
                "2" => column = 1,
                "3" => column = 2,
                _ => return Err(String::from("Unexpected !!"))
            }
        }
        None => return Err(String::from("Invalid line or column"))
    }
    
    return Ok(3 * line + column)
}

fn get_move_from_human() -> Result<usize, String> {
    let mut buffer : String = String::new();

    let line = io::stdin().read_line(&mut buffer);
    match line {
        Ok(_) => {},
        Err(error) => panic!("Invalid entry from player, error: {error}"),
    }

    parse_user_entry(buffer)
}

fn get_move_from_ia(board: board::Board, ia_tile_state: board::TileState) -> Result<usize, String> {
    
    let index = ia::get_move(board, ia_tile_state);

    match index {
        // index 8 should be derived from board.board but Rust won't allow a runtime defined value
        0..=8 => Ok(index),
        _ => Err("Invalid index returned by IA engine".into()),
    }
}

fn parse_arguments(args: Vec<String>) -> GameSettings
{
    let mut ia_opponent_setting = false;

    for arg in args.iter() {
        if arg.eq("solo") {
            ia_opponent_setting = true;
        }
    }

    GameSettings { 
        ia_opponent: ia_opponent_setting
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let game_settings = parse_arguments(args);
    
    let mut board= Default::default();
    
    let mut current_player : board::TileState = board::TileState::TileStateCross;
    let mut last_player : board::TileState = board::TileState::TileStateEmpty;

    let computer_player;

    if game_settings.ia_opponent {
        computer_player = board::TileState::TileStateRound;
    }
    else {
        computer_player = board::TileState::TileStateEmpty;
    }

    display_welcome();
    
    while board::is_board_full(board) == false && 
          board::is_line_achieved(board) == false {
    
        display_board(board);

        let player_move_index;

        if game_settings.ia_opponent &&
           current_player == computer_player {
            player_move_index = get_move_from_ia(board, computer_player);
        }
        else {
            player_move_index = get_move_from_human();
        }
        
        match player_move_index {
            Ok(index) => {
                // TODO-improv: Find how Rust manages out-of-bounds assignment idiomatically
                if board.board[index].tile_state != board::TileState::TileStateEmpty
                {
                    println!("This box is already taken, please play again");
                    continue;
                }
                        
                board.board[index].tile_state = current_player;
                
                last_player = current_player;
                if current_player == board::TileState::TileStateCross {
                    current_player = board::TileState::TileStateRound;
                } else {
                    current_player = board::TileState::TileStateCross;
                } 
            },
            Err(error) => {
                println!("error: {error}");
                return;
            }
        }
    }
    
    display_board(board);

    if board::is_line_achieved(board) == false {
        println!("It's a draw");
    }
    else {
        println!("Victory for {last_player} !");
    }
}
