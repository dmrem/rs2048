use std::io;

mod board;
mod game;
mod user_interface;

fn main() {
    user_interface::start_app(&mut io::stdout()).unwrap();
}
