use std::cmp;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataGrid<T>
where
    T: Clone,
{
    values: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MatrixError {
    InvalidDataLength(String),
    IndexNotFound,
}

impl<T: Clone> DataGrid<T> {
    /// Creates a new matrix with the specified dimensions and initializes all elements with the given initial value.
    ///
    /// # Arguments
    ///
    /// * `width` - The width (number of columns) of the matrix.
    /// * `height` - The height (number of rows) of the matrix.
    /// * `initial_value` - The initial value to fill the matrix with.
    pub fn new(width: usize, height: usize, initial_value: T) -> DataGrid<T> {
        DataGrid {
            values: vec![vec![initial_value; width]; height],
        }
    }

    /// Gets a row from the matrix by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the row to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Option<Vec<T>>` containing the row's elements, or `None` if the index is out of bounds.
    pub fn get_row(&self, index: usize) -> Option<Vec<T>> {
        self.values.get(index).cloned()
    }

    /// Gets a column from the matrix by its index. The item in the top row of the matrix is in the
    /// front of the output.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the column to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Vec<T>>` containing the column's elements, or `None` if the index is out of bounds.
    pub fn get_column(&self, index: usize) -> Option<Vec<T>> {
        self.values
            .iter()
            .map(|vec| vec.get(index).cloned())
            .collect()
    }

    /// Updates a row in the matrix with the provided data.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the row to update.
    /// * `data` - The new data to replace the row with. It must have the same length as the matrix's width.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update was successful, or an `Err(MatrixError)` with a description of the error otherwise.
    pub fn update_row(&mut self, index: usize, data: Vec<T>) -> Result<(), MatrixError> {
        if data.len() != self.values[0].len() {
            return Err(MatrixError::InvalidDataLength(
                "Input data length is not equal to matrix width!".to_string(),
            ));
        }

        if let Some(row) = self.values.get_mut(index) {
            *row = data;
            Ok(())
        } else {
            Err(MatrixError::IndexNotFound)
        }
    }

    /// Updates a column in the matrix with the provided data.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the column to update.
    /// * `data` - The new data to replace the column with. It must have the same length as the matrix's height.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update was successful, or an `Err(MatrixError)` with a description of the error otherwise.
    pub fn update_column(&mut self, index: usize, data: Vec<T>) -> Result<(), MatrixError> {
        if data.len() != self.values.len() {
            return Err(MatrixError::InvalidDataLength(
                "Input data length is not equal to matrix height!".to_string(),
            ));
        }

        for (row, value) in self.values.iter_mut().zip(data) {
            if let Some(column) = row.get_mut(index) {
                *column = value;
            } else {
                return Err(MatrixError::IndexNotFound);
            }
        }

        Ok(())
    }

    /// Updates a single position in the matrix with the provided value.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the position to update.
    /// * `column` - The column index of the position to update.
    /// * `value` - The new value to set at the specified position.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the update was successful, or an `Err(MatrixError)` with a description of the error otherwise.
    pub fn update_single_position(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<(), MatrixError> {
        *(self
            .values
            .get_mut(row)
            .ok_or(MatrixError::IndexNotFound)?
            .get_mut(column)
            .ok_or(MatrixError::IndexNotFound)?) = value;
        Ok(())
    }

    /// Transpose the DataGrid, converting columns into rows.
    ///
    /// # Returns
    ///
    /// A new DataGrid representing the transposed data.
    ///
    /// # Example
    ///
    /// ```
    /// use data_grid::DataGrid;
    /// let grid: DataGrid<i32> = DataGrid::try_from(vec![vec![1, 2, 3], vec![1, 2, 3]]).unwrap();
    /// let transposed_grid: DataGrid<i32> = grid.transpose();
    ///
    /// assert!(transposed_grid == DataGrid::try_from(vec![vec![1, 1], vec![2, 2], vec![3, 3]]).unwrap());
    /// ```
    pub fn transpose(&self) -> DataGrid<T> {
        if self.values.is_empty() {
            // this will never happen because the constructor prevents it
            return self.clone();
        }

        // The internal values object is a Vec<Vec<T>. This data is stored such that each inner vec is a row.
        // By getting each column, we can store those as the rows in the new data grid, getting transposition for free.
        // See the implementation of get_column for context.
        let rows: Vec<Vec<T>> = (0..self.get_width())
            .map(|col_index| match self.get_column(col_index) {
                Some(item) => item,
                None => Vec::new(),
            })
            .collect();

        DataGrid { values: rows }
    }

    /// Returns an immutable iterator over the rows in the DataGrid.
    ///
    /// To iterate over columns, call `grid.transpose().iter_rows()`.
    ///
    /// # Returns
    ///
    /// An iterator that yields references to rows as `&Vec<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use data_grid::DataGrid;
    /// let grid: DataGrid<i32> = DataGrid::try_from(vec![vec![1, 2, 3], vec![1, 2, 3]]).unwrap();
    /// for row in grid.iter_rows() {
    ///     // Process each row.
    /// }
    /// ```
    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.values.iter()
    }

    /// Gets the height (number of rows) of the matrix.
    ///
    /// # Returns
    ///
    /// Returns the height of the matrix as a `usize` value.
    pub fn get_height(&self) -> usize {
        self.values.len()
    }

    /// Gets the width (number of columns) of the matrix.
    ///
    /// # Returns
    ///
    /// Returns the width of the matrix as a `usize` value.
    pub fn get_width(&self) -> usize {
        self.values[0].len()
    }

    // get data in grid immutably - this exists to read all the data without needing to clone each row
    pub fn get_values(&self) -> &Vec<Vec<T>> {
        &self.values
    }
}

impl<T: Clone> TryFrom<Vec<Vec<T>>> for DataGrid<T> {
    type Error = MatrixError;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(MatrixError::InvalidDataLength(
                "Matrix must be at least 1 wide".to_string(),
            ));
        }
        if value[0].is_empty() {
            return Err(MatrixError::InvalidDataLength(
                "Matrix must be at least 1 tall".to_string(),
            ));
        }

        if !value
            .iter()
            .map(|inner| inner.len())
            .all(|len| len == value[0].len())
        {
            return Err(MatrixError::InvalidDataLength(
                "Matrix rows must have consistent lengths".to_string(),
            ));
        }

        Ok(DataGrid { values: value })
    }
}

impl<T> Display for DataGrid<T>
where
    T: Clone,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_item_length = self.values.iter().fold(0usize, |max_row_len, vec| {
            cmp::max(
                max_row_len,
                vec.iter().fold(0usize, |max_item_len, item| {
                    cmp::max(max_item_len, item.to_string().len())
                }),
            )
        });

        let cell_width = max_item_length + 2; // add two for a space on each side
        let grid_width = self.values[0].len();

        // write top border
        write!(
            f,
            "{}",
            DataGrid::<T>::create_constant_row(grid_width, cell_width, '┌', '┬', '┐', '─').as_str()
        )?;

        let inner_rows = self
            .values
            .iter()
            .map(|current_row| {
                // write blank lines above row
                // let num_blank_lines_above = (cell_width - 1) / 2; // subtract 1 for row where text is
                let num_blank_lines_above = 1;

                let mut string = "".to_string();

                for _ in 0..num_blank_lines_above {
                    string += DataGrid::<T>::create_constant_row(
                        grid_width, cell_width, '│', '│', '│', ' ',
                    )
                    .as_str()
                }

                // write row
                string += format!(
                    "│{}│\n",
                    current_row
                        .iter()
                        .map(|item| {
                            let spaces_before = (cell_width - item.to_string().len()) / 2;
                            let spaces_after =
                                (cell_width - item.to_string().len()) - spaces_before; // subtract here because spaces_before and spaces_after aren't equal if cell_width - item length is odd, and want all cells to be consistent width
                            format!(
                                "{}{}{}",
                                " ".repeat(spaces_before),
                                item,
                                " ".repeat(spaces_after)
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("│")
                )
                .as_str();

                // write blank lines below row
                // let num_blank_lines_below = (cell_width - 1) - num_blank_lines_above; // subtract here for the same reason as above
                let num_blank_lines_below = 1;
                for _ in 0..num_blank_lines_below {
                    string += DataGrid::<T>::create_constant_row(
                        grid_width, cell_width, '│', '│', '│', ' ',
                    )
                    .as_str()
                }

                string
            })
            .collect::<Vec<String>>()
            .join(
                DataGrid::<T>::create_constant_row(grid_width, cell_width, '├', '┼', '┤', '─')
                    .as_str(),
            );

        write!(f, "{}", inner_rows)?;

        // write bottom border
        write!(
            f,
            "{}",
            DataGrid::<T>::create_constant_row(grid_width, cell_width, '└', '┴', '┘', '─')
        )?;

        Ok(())
    }
}

// utility functions for display trait
impl<T> DataGrid<T>
where
    T: Clone,
    T: Display,
{
    /// Creates a constant row of text for the grid with specified formatting.
    ///
    /// This function generates a row of text with a specified number of cells, each cell having a
    /// specified width and containing the same filler character. The row is formatted with opening,
    /// joining, and closing characters.
    ///
    /// # Arguments
    ///
    /// - `number_of_cells`: The number of cells in the row.
    /// - `cell_width`: The width of each cell, including spaces.
    /// - `opening_char`: The character used at the beginning of the row.
    /// - `joining_char`: The character used to join cells within the row.
    /// - `closing_char`: The character used at the end of the row.
    /// - `filler_char`: The character used to fill each cell.
    ///
    /// # Returns
    ///
    /// A `String` containing the generated row of text.
    ///
    fn create_constant_row(
        number_of_cells: usize,
        cell_width: usize,
        opening_char: char,
        joining_char: char,
        closing_char: char,
        filler_char: char,
    ) -> String {
        format!(
            "{}{}{}\n",
            opening_char,
            (0..number_of_cells)
                .map(|_| filler_char.to_string().repeat(cell_width))
                .collect::<Vec<String>>()
                .join(joining_char.to_string().as_str()),
            closing_char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let expected = DataGrid {
            values: vec![vec![0; 4]; 4],
        };
        let actual = DataGrid::new(4, 4, 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_row_valid_index() {
        let matrix = DataGrid::new(3, 3, 0);
        let expected = Some(vec![0, 0, 0]);
        let actual = matrix.get_row(1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_row_invalid_index() {
        let matrix = DataGrid::new(3, 3, 0);
        let expected = None;
        let actual = matrix.get_row(4);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_column_valid_index() {
        let matrix = DataGrid::new(3, 3, 0);
        let expected = Some(vec![0, 0, 0]);
        let actual = matrix.get_column(1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_column_invalid_index() {
        let matrix = DataGrid::new(3, 3, 0);
        let expected = None;
        let actual = matrix.get_column(4);

        assert_eq!(expected, actual);
    }

    #[test]
    fn update_row_valid_index() {
        let mut matrix = DataGrid::new(3, 3, 0);
        let data = vec![1, 1, 1];
        let expected = Ok(());
        let actual = matrix.update_row(1, data);

        assert_eq!(expected, actual);
        assert_eq!(matrix.get_row(1), Some(vec![1, 1, 1]));
    }

    #[test]
    fn update_row_invalid_index() {
        let mut matrix = DataGrid::new(3, 3, 0);
        let data = vec![1, 1, 1];
        let expected = Err(MatrixError::IndexNotFound);
        let actual = matrix.update_row(4, data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn update_column_valid_index() {
        let mut matrix = DataGrid::new(3, 3, 0);
        let data = vec![1, 1, 1];
        let expected = Ok(());
        let actual = matrix.update_column(1, data);

        assert_eq!(expected, actual);
        assert_eq!(matrix.get_column(1), Some(vec![1, 1, 1]));
    }

    #[test]
    fn update_column_invalid_index() {
        let mut matrix = DataGrid::new(3, 3, 0);
        let data = vec![1, 1, 1];
        let expected = Err(MatrixError::IndexNotFound);
        let actual = matrix.update_column(4, data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_transpose() {
        let grid: DataGrid<i32> =
            DataGrid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
        let transposed_grid: DataGrid<i32> = grid.transpose();
        let expected_grid: DataGrid<i32> =
            DataGrid::try_from(vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]).unwrap();

        // Assert that the transposed grid matches the expected grid
        assert_eq!(transposed_grid, expected_grid);
    }
}
