use crate::board::{self, TileState};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// List strategies in the hierarchical order they must be processed
#[derive(EnumIter)]
enum StrategyType {
    StrategyManageLastTileOnSegment,
    StrategyFirstFreeTile,
}

impl StrategyType {
    pub fn get_index_using_strategy(&self, board: &board::Board, ia_tile_state: board::TileState) -> Option<usize> {
        match self {
            StrategyType::StrategyManageLastTileOnSegment => fill_in_segment_or_block_opponent(board, ia_tile_state),
            
            // Default strategy
            StrategyType::StrategyFirstFreeTile => 
                board.board.iter().position(|&x| x.tile_state == board::TileState::TileStateEmpty),
        }
    }
}

/// Complete a segment, either to win or to avoid defeat
fn fill_in_segment_or_block_opponent(board: &board::Board, ia_tile_state: board::TileState) -> Option<usize> {

    let mut returned_index = Option::None;

    /* TODO As this is a prototype, I just copied the same processing. Will need to factorize this. */

    for line_index in 0..=2 {
        let line = board.get_line(line_index).unwrap();
        let (empty_tiles, not_empty_tiles): (Vec<&board::Tile>, Vec<&board::Tile>) = line.iter()
            .map(|x| *x)    // Dereference once to get a vector of &Tile
            .partition(|&x| x.tile_state == TileState::TileStateEmpty);

        if empty_tiles.len() != 1 {
            continue;
        }

        if not_empty_tiles[0].tile_state != not_empty_tiles[1].tile_state {
            continue;
        }

        // and the other two tiles are of same type
        
        if not_empty_tiles[0].tile_state == ia_tile_state {
            // Victory is on the line : leave at once
            return Some(empty_tiles[0].index_in_board);
        }
        else {
            // If not, block opponent, but only after we checked that there is not an other possibility of victory
            returned_index = Some(empty_tiles[0].index_in_board);
        }
    }

    for column_index in 0..=2 {
        let column = board.get_column(column_index).unwrap();
        let (empty_tiles, not_empty_tiles): (Vec<&board::Tile>, Vec<&board::Tile>) = column.iter()
            .map(|x| *x)    // Dereference once to get a vector of &Tile
            .partition(|&x| x.tile_state == TileState::TileStateEmpty);

        if empty_tiles.len() != 1 {
            continue;
        }

        if not_empty_tiles[0].tile_state != not_empty_tiles[1].tile_state {
            continue;
        }

        // and the other two tiles are of same type
        
        if not_empty_tiles[0].tile_state == ia_tile_state {
            // Victory is on the line : leave at once
            return Some(empty_tiles[0].index_in_board);
        }
        else {
            // If not, block opponent, but only after we checked that there is not an other possibility of victory
            returned_index = Some(empty_tiles[0].index_in_board);
        }
    }

    for diagonal_index in 0..=1 {
        let diagonal = board.get_diagonal(diagonal_index).unwrap();
        let (empty_tiles, not_empty_tiles): (Vec<&board::Tile>, Vec<&board::Tile>) = diagonal.iter()
            .map(|x| *x)    // Dereference once to get a vector of &Tile
            .partition(|&x| x.tile_state == TileState::TileStateEmpty);

        if empty_tiles.len() != 1 {
            continue;
        }

        if not_empty_tiles[0].tile_state != not_empty_tiles[1].tile_state {
            continue;
        }

        // and the other two tiles are of same type
        
        if not_empty_tiles[0].tile_state == ia_tile_state {
            // Victory is on the line : leave at once
            return Some(empty_tiles[0].index_in_board);
        }
        else {
            // If not, block opponent, but only after we checked that there is not an other possibility of victory
            returned_index = Some(empty_tiles[0].index_in_board);
        }
    }
    
    return returned_index;
}

pub fn get_move(board: &board::Board, ia_tile_state: board::TileState) -> usize {
    for strategy in StrategyType::iter() {
        if let Some(index) = strategy.get_index_using_strategy(board, ia_tile_state) {
            return index;
        } 
    }
    
    panic!("No strategy could work !!!");
}

#[cfg(test)]
mod tests_ia
{
    use crate::{board::{Board, Tile, TileState}, ia::get_move};

    #[test]
    fn test_sequence_a1_a3_b2() {
        let board = Board {
            board: [ 
                Tile{ tile_state: TileState::TileStateCross, index_in_board: 0 },   // A1: player
                Tile{ tile_state: TileState::TileStateRound, index_in_board: 1 },   // A2: ia
                Tile{ tile_state: TileState::TileStateCross, index_in_board: 2 },   // A3: player
                Tile{ tile_state: TileState::TileStateRound, index_in_board: 3 },   // B1: ia
                Tile{ tile_state: TileState::TileStateCross, index_in_board: 4 },   // B2: player
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 5 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 6 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 7 },
                Tile{ tile_state: TileState::TileStateEmpty, index_in_board: 8 },
            ], 
        };

        let next_move = get_move(&board, TileState::TileStateRound);

        assert_eq!(next_move, 6);
    }
}