use crate::board::{Board, TileType};
use crate::game::GameError::AddRandomTileError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    score: u32,
    is_game_over: bool,
    game_over_reason: Option<String>,
}

pub enum GameEvent {
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    Undo,
    SaveGame,
    LoadGame,
    NewGame,
}

#[derive(Debug)]
pub enum GameError {
    AddRandomTileError,
}

impl Game {
    // Game is intended to be immutable. This function will consume the Game and return a new one.
    pub fn handle_event(mut self, event: GameEvent) -> Result<Game, GameError> {
        match event {
            GameEvent::SwipeUp => {
                let before = self.clone();
                self.board.merge_up();
                if self.board != before.board {
                    self.board.add_random_tile().or(Err(AddRandomTileError))?;
                }
                Ok(self)
            }
            GameEvent::SwipeDown => {
                let before = self.clone();
                self.board.merge_down();
                if self.board != before.board {
                    self.board.add_random_tile().or(Err(AddRandomTileError))?;
                }
                Ok(self)
            }
            GameEvent::SwipeLeft => {
                let before = self.clone();
                self.board.merge_left();
                if self.board != before.board {
                    self.board.add_random_tile().or(Err(AddRandomTileError))?;
                }
                Ok(self)
            }
            GameEvent::SwipeRight => {
                let before = self.clone();
                self.board.merge_right();
                if self.board != before.board {
                    self.board.add_random_tile().or(Err(AddRandomTileError))?;
                }
                Ok(self)
            }
            GameEvent::Undo => {
                todo!()
            }
            GameEvent::SaveGame => {
                todo!()
            }
            GameEvent::LoadGame => {
                todo!()
            }
            GameEvent::NewGame => Game::start_new_game(),
        }
    }
    pub fn start_new_game() -> Result<Game, GameError> {
        let mut game = Game {
            board: Board::new(4),
            score: 0,
            is_game_over: false,
            game_over_reason: None,
        };
        game.board.add_random_tile().unwrap();
        Ok(game)
    }

    pub fn read_board_state(&self) -> &Vec<Vec<TileType>> {
        self.board.get_data_for_display()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}
