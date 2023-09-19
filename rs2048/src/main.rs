use crate::board::Board;

mod board;

fn main() {
    let mut board = Board::new(4);
    board.place_item_in_board(2, 1, 32).unwrap();
    board.place_item_in_board(1, 1, 32).unwrap();
    println!("{}", board);
    board.merge_up();
    print!("{}", board);
}
