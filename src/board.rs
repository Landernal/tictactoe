use std::{fmt, self};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileState {
    TileStateEmpty,
    TileStateCross,
    TileStateRound,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_state: TileState,
    pub index_in_board: usize,
}

#[derive(Copy, Clone)]
pub struct Board
{
    pub board : [Tile; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [ 
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 0 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 1 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 2 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 3 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 4 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 5 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 6 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 7 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 8 },
            ], 
        }
    }
}

impl Board {
    pub fn get_line<'a>(&'a self, line_index: usize) -> Option<[&'a Tile;3]> {
        match line_index {
            0..=2 => Option::Some([&self.board[3*line_index], &self.board[3*line_index+1], &self.board[3*line_index+2]]),
            _ => Option::None
        }
    }
    
    pub fn get_column<'a>(&'a self, column_index: usize) -> Option<[&'a Tile;3]> {
        match column_index {
            0..=2 => Option::Some([&self.board[column_index], &self.board[column_index+3], &self.board[column_index+6]]),
            _ => Option::None
        }
    }
    
    pub fn get_diagonal<'a>(&'a self, diagonal_index: usize) -> Option<[&'a Tile;3]> {
        match diagonal_index {
            0 => Option::Some([&self.board[0], &self.board[4], &self.board[8]]),
            1 => Option::Some([&self.board[2], &self.board[4], &self.board[6]]),
            _ => Option::None
        }
    }
    

    pub fn is_board_full(&self) -> bool {

        return self.board
                    .iter()
                    .any(|&f| f.tile_state == TileState::TileStateEmpty) == false;
    }

    pub fn is_line_achieved(&self) -> bool {
        
        // 1-Perform horizontal checks

        for line_index in 0..2 {
            let line = self.get_line(line_index).unwrap();
            
            if line.iter().all(|&x| x.tile_state == TileState::TileStateCross) || 
            line.iter().all(|&x| x.tile_state == TileState::TileStateRound) {
                return true;
            }
        }
        
        // 2-Perform vertical checks

        for column_index in 0..2 {
            let column = self.get_column(column_index).unwrap();
            
            if column.iter().all(|&x| x.tile_state == TileState::TileStateCross) || 
            column.iter().all(|&x| x.tile_state == TileState::TileStateRound) {
                return true;
            }
        }
        
        // 3-Perform diagonal checks

        for diagonal_index in 0..2 {
            let diagonal = self.get_diagonal(diagonal_index).unwrap();
            
            if diagonal.iter().all(|&x| x.tile_state == TileState::TileStateCross) || 
            diagonal.iter().all(|&x| x.tile_state == TileState::TileStateRound) {
                return true;
            }
        }
        
        return false;
    }
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tile_state)
    }
}