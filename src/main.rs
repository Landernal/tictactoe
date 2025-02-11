use std::io;
use std::fmt;
use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq)]
enum TileState {
    TileStateEmpty,
    TileStateCross,
    TileStateRound,
}

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
#[derive(Copy, Clone)]
struct Board
{
    board : [TileState; 9],
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "    1 - 2 - 3\n{}:  {} | {} | {}\n   -----------\n{}:  {} | {} | {}\n   -----------\n{}:  {} | {} | {}",
        "A", self.board[0], self.board[1], self.board[2],
        "B", self.board[3], self.board[4], self.board[5],
        "C", self.board[6], self.board[7], self.board[8])
    }
}


impl fmt::Display for TileState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        
        match self {
            TileState::TileStateEmpty => write!(f, "{}", ' '),
            TileState::TileStateCross => write!(f, "{}", 'X'),
            TileState::TileStateRound => write!(f, "{}", 'O'),
            }
    }
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


fn diplay_board(board: Board) {
    
    println!("{board}");
    println!("\n");
}

fn is_board_full(board: Board) -> bool {

    return board.board
                .iter()
                .any(|&f| f == TileState::TileStateEmpty) == false;
}

fn is_line_achieved(board: Board) -> bool {
    
    // 1-Perform horizontal checks
    
    if board.board.iter().step_by(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().step_by(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    if board.board.iter().skip(1).step_by(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().skip(1).step_by(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    if board.board.iter().skip(2).step_by(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().skip(2).step_by(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    // 2-Perform vertical checks
    
    if board.board.iter().take(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().take(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    if board.board.iter().skip(3).take(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().skip(3).take(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    if board.board.iter().skip(6).take(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().skip(6).take(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    // 3-Perform diagonal checks
    
    if board.board.iter().step_by(4).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().step_by(4).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    if board.board.iter().skip(2).step_by(2).take(3).all(|&x| x == TileState::TileStateCross) || 
       board.board.iter().skip(2).step_by(2).take(3).all(|&x| x == TileState::TileStateRound) {
        return true;
    }
    
    return false;
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
                &_ => return Err(String::from("Unexpected !!"))
            }
            
            match caps.get(2).unwrap().as_str() {
                "1" => column = 0,
                "2" => column = 1,
                "3" => column = 2,
                &_ => return Err(String::from("Unexpected !!"))
            }
        }
        None => return Err(String::from("Invalid line or column"))
    }
    
    return Ok(3 * line + column)
}

fn main() {
    
    let mut _board = Board {
        board: [TileState::TileStateEmpty, TileState::TileStateEmpty, TileState::TileStateEmpty, 
                TileState::TileStateEmpty, TileState::TileStateEmpty, TileState::TileStateEmpty,
                TileState::TileStateEmpty, TileState::TileStateEmpty, TileState::TileStateEmpty]
    };
    
    let mut player : TileState = TileState::TileStateRound;    

    display_welcome();
    
    while is_board_full(_board) == false && 
          is_line_achieved(_board) == false {
    
        diplay_board(_board);
        
        let mut buffer : String = String::new();

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {},
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
        
        match parse_user_entry(buffer) {
            Ok(parsed_index) => {
                // TODO-improv: Find how Rust manages out-of-bounds assignment idiomatically
                if _board.board[parsed_index] != TileState::TileStateEmpty
                {
                    println!("This box is already taken, please play again");
                    continue;
                }
                        
                if player == TileState::TileStateCross {
                    player = TileState::TileStateRound;
                } else {
                    player = TileState::TileStateCross;
                } 
        
                _board.board[parsed_index] = player;
            },
            Err(error) => {
                println!("error: {error}");
                return;
            }
        }
    }
    
    diplay_board(_board);

    if is_line_achieved(_board) == false {
        println!("It's a draw");
    }
    else {
        println!("Victory for {player} !");        
    }
}
