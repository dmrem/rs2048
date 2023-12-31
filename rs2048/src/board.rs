use data_grid::{DataGrid, MatrixError};
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};

pub type TileType = u8;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    board: DataGrid<TileType>, // items are stored as their power of 2 - if 3 is in the grid, that means 8 is shown in game because 2^3=8
}

#[derive(Debug)]
pub enum BoardError {
    AddRandomTileError,
}

impl Board {
    /// Creates a new `Board` with the specified size and initializes all cells with zero values.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the square board (number of rows and columns).
    ///
    /// # Returns
    ///
    /// Returns a new `Board` instance.
    pub fn new(size: usize) -> Board {
        Board {
            board: DataGrid::new(size, size, 0 as TileType),
        }
    }

    /// Places an item with the specified value at the given column and row on the board.
    ///
    /// # Arguments
    ///
    /// * `column` - The column index where the item will be placed.
    /// * `row` - The row index where the item will be placed.
    /// * `value` - The value of the item to be placed on the board.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the placement was successful, or an `Err(MatrixError)` with a description of the error otherwise.
    fn place_item_in_board(
        &mut self,
        column: usize,
        row: usize,
        value: TileType,
    ) -> Result<(), MatrixError> {
        self.board.update_single_position(column, row, value)
    }

    /// Merges the cells in the board by moving tiles upwards as if the user had swiped up.
    pub fn merge_up(&mut self) {
        for i in 0..self.board.get_width() {
            let column = self.board.get_column(i).unwrap();
            self.board
                .update_column(i, Board::merge_tiles(&column))
                .unwrap();
        }
    }

    /// Merges the cells in the board by moving tiles downwards as if the user had swiped down.
    pub fn merge_down(&mut self) {
        for i in 0..self.board.get_width() {
            let mut column = self.board.get_column(i).unwrap();
            column.reverse();
            let mut merged = Board::merge_tiles(&column);
            merged.reverse();
            self.board.update_column(i, merged).unwrap();
        }
    }

    /// Merges the cells in the board by moving tiles to the left as if the user had swiped left.
    pub fn merge_left(&mut self) {
        for i in 0..self.board.get_height() {
            let row = self.board.get_row(i).unwrap();
            self.board.update_row(i, Board::merge_tiles(&row)).unwrap();
        }
    }

    /// Merges the cells in the board by moving tiles to the right as if the user had swiped right.
    pub fn merge_right(&mut self) {
        for i in 0..self.board.get_height() {
            let mut row = self.board.get_row(i).unwrap();
            row.reverse();
            let mut merged = Board::merge_tiles(&row);
            merged.reverse();
            self.board.update_row(i, merged).unwrap();
        }
    }

    /// Merges the tiles in a single row or column as if motion is from the back of the vector to the front.
    ///
    /// This function takes a vector representing a row or column of the game board and merges it according to
    /// the rules of the 2048 game.
    ///
    /// # Arguments
    ///
    /// * `tiles` - A reference to a vector containing the tiles to be merged.
    ///
    /// # Returns
    ///
    /// Returns a new vector with the merged tiles.
    fn merge_tiles(tiles: &[TileType]) -> Vec<TileType> {
        if tiles.is_empty() {
            return vec![];
        }

        let mut last_seen_tile: TileType = tiles[0];
        let mut result: Vec<TileType> = Vec::with_capacity(tiles.len());

        for &tile in tiles.iter().skip(1) {
            if tile == 0 {
                continue;
            }

            if tile == last_seen_tile {
                result.push(tile + 1);
                last_seen_tile = 0;
            } else {
                if last_seen_tile != 0 {
                    result.push(last_seen_tile);
                }
                last_seen_tile = tile;
            }
        }
        result.push(last_seen_tile);
        result.extend([0].repeat(tiles.len() - result.len()));
        result
    }

    /// Adds a new tile with a random value to a random empty position on the board.
    ///
    /// The function searches for empty positions on the board and randomly selects one
    /// to place a new tile. The new tile is assigned a value of either 2 or 4 based on
    /// a weighted choice (3:1 ratio for 2's and 4's).
    ///
    /// # Errors
    ///
    /// If there are no empty positions on the board, an `Err(BoardError::AddRandomTileError)`
    /// is returned, indicating that there is no available space to insert a new tile.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if a new tile is successfully added.
    /// - An error variant of `BoardError` if the operation fails.
    ///
    /// # Example
    ///
    /// ```
    /// let mut board = Board::new(4);
    /// board.add_random_tile().unwrap();
    /// ```
    pub fn add_random_tile(&mut self) -> Result<(), BoardError> {
        let empty_positions: Vec<(usize, usize)> = self
            .board
            .iter_rows()
            .enumerate()
            .flat_map(|(y_index, vec)| {
                vec.iter()
                    .enumerate()
                    .filter(|&(_x_index, &item)| item == 0)
                    .map(|(x_index, _item)| (x_index, y_index))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        if let Some(pos) = empty_positions.choose(&mut rand::thread_rng()) {
            let value_to_add = [1 as TileType, 2]
                .choose_weighted(
                    &mut rand::thread_rng(),
                    |item| if *item == 1 { 3 } else { 1 },
                )
                .unwrap();
            self.place_item_in_board(pos.1, pos.0, *value_to_add)
                .unwrap();
        } else {
            return Err(BoardError::AddRandomTileError); // nowhere to insert tile
        }

        Ok(())
    }

    pub fn get_data_for_display(&self) -> &Vec<Vec<TileType>> {
        self.board.get_values()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board.to_string().replace(" 0 ", "   "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // single row merge tests
    #[test]
    fn merge_simple() {
        let input = vec![2 as TileType, 2, 0, 0];
        let expected = vec![3 as TileType, 0, 0, 0];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }
    #[test]
    fn merge_with_spaces() {
        let input = vec![2 as TileType, 0, 2, 0];
        let expected = vec![3 as TileType, 0, 0, 0];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }
    #[test]
    fn merge_but_cant() {
        let input = vec![2 as TileType, 3, 2, 3];
        let expected = vec![2 as TileType, 3, 2, 3];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_all_same() {
        let input = vec![2 as TileType, 2, 2, 2];
        let expected = vec![3 as TileType, 3, 0, 0];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn dont_merge_twice_at_once() {
        let input = vec![1 as TileType, 1, 2, 0];
        let expected = vec![2 as TileType, 2, 0, 0];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn dont_merge_twice_at_once_reverse() {
        let input = vec![2 as TileType, 1, 1, 0];
        let expected = vec![2 as TileType, 2, 0, 0];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_empty_input() {
        let input = vec![];
        let expected: Vec<TileType> = vec![];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_single_element() {
        let input = vec![2 as TileType];
        let expected = vec![2 as TileType];
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_large_input() {
        let input = vec![2 as TileType; 1000];
        let mut expected = vec![3 as TileType; 500];
        expected.extend(vec![0 as TileType; 500]);
        let actual = Board::merge_tiles(&input);
        assert_eq!(expected, actual);
    }

    // Board merge tests

    #[test]
    fn merge_up_simple() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 0, 0, 0 as TileType],
                vec![2, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![3, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_up();

        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_up_cant_merge() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![3, 3, 3, 3 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![3, 3, 3, 3 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![3, 3, 3, 3 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![3, 3, 3, 3 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_up();

        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_up_full_board() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![3, 3, 3, 3 as TileType],
                vec![3, 3, 3, 3 as TileType],
                vec![0, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_up();

        assert_eq!(expected, actual);
    }
    #[test]
    fn merge_left_full_board() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![3, 3, 0, 0 as TileType],
                vec![3, 3, 0, 0 as TileType],
                vec![3, 3, 0, 0 as TileType],
                vec![3, 3, 0, 0 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_left();

        assert_eq!(expected, actual);
    }
    #[test]
    fn merge_right_full_board() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![0, 0, 3, 3 as TileType],
                vec![0, 0, 3, 3 as TileType],
                vec![0, 0, 3, 3 as TileType],
                vec![0, 0, 3, 3 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_right();

        assert_eq!(expected, actual);
    }
    #[test]
    fn merge_down_full_board() {
        let input = Board {
            board: DataGrid::try_from(vec![
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
                vec![2, 2, 2, 2 as TileType],
            ])
            .unwrap(),
        };

        let expected = Board {
            board: DataGrid::try_from(vec![
                vec![0, 0, 0, 0 as TileType],
                vec![0, 0, 0, 0 as TileType],
                vec![3, 3, 3, 3 as TileType],
                vec![3, 3, 3, 3 as TileType],
            ])
            .unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_down();

        assert_eq!(expected, actual);
    }

    #[test]
    fn merge_up_large_board() {
        let input = Board {
            board: DataGrid::try_from(vec![vec![2 as TileType; 1000]; 1000]).unwrap(),
        };

        let mut expected_board = vec![vec![3 as TileType; 1000]; 500];
        expected_board.extend(vec![vec![0 as TileType; 1000]; 500]);
        let expected = Board {
            board: DataGrid::try_from(expected_board).unwrap(),
        };

        let mut actual = input.clone();
        actual.merge_up();

        assert_eq!(expected, actual);
    }
}
