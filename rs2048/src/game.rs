use crate::board::Board;

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

pub enum GameError {}

impl Game {
    // Game is intended to be immutable. This function will consume the Game and return a new one.
    pub fn handle_event(self, event: GameEvent) -> Result<Game, GameError> {
        match event {
            GameEvent::SwipeUp => {
                todo!()
            }
            GameEvent::SwipeDown => {
                todo!()
            }
            GameEvent::SwipeLeft => {
                todo!()
            }
            GameEvent::SwipeRight => {
                todo!()
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
}
