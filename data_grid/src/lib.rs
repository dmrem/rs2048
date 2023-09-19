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

    /// Gets a column from the matrix by its index.
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
}
