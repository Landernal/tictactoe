use crate::board::{self, TileState};

fn fill_in_segment_or_block_opponent(board: board::Board, ia_tile_state: board::TileState) -> Option<usize> {

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

pub fn get_move(board: board::Board, ia_tile_state: board::TileState) -> usize {

    // Complete a segment, either to win or to avoid defeat
    if let Some(index) = fill_in_segment_or_block_opponent(board, ia_tile_state) {
        return index;
    }
    
    // Default : return first free tile
    board.board.iter()
        .position(|&x| x.tile_state == board::TileState::TileStateEmpty)
        .unwrap()   // If this case is to happen, let the program panic
}