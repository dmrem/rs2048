use data_grid::DataGrid;

fn main() {
    let mut x = DataGrid::new(4, 4, 0);
    x.update_column(2, vec![1, 2, 3, 4]).unwrap();

    println!("{}", x);
}
