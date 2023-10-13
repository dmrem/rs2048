use crate::board::TileType;
use crate::game::{Game, GameError, GameEvent};
use crate::user_interface::MainMenuOption::{LoadGame, NewGame, Quit};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{Color, StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, event, queue, style, terminal, ExecutableCommand, QueueableCommand};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::{cmp, io};

#[derive(Debug, Eq, PartialEq)]
enum MainMenuOption {
    NewGame,
    LoadGame,
    Quit,
}

/// This is the entrypoint to the game.
///
/// This function initializes the TUI and starts the main menu event loop.
///
/// # Arguments
///
/// * `writer` - A mutable reference to an `io::Write` implementor for writing to the terminal.
///
/// # Returns
///
/// Returns an `io::Result` that indicates success or failure.
pub fn start_app<W: io::Write>(writer: &mut W) -> io::Result<()> {
    writer.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(
        writer,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;
    writer.flush()?;

    main_menu_loop(writer)?;
    writer.execute(terminal::LeaveAlternateScreen)?; // todo if program throws error, this line doesn't execute, and terminal stays in curses mode when the shell regains control
    Ok(())
}

/// Main loop for the game's main menu.
///
/// This function handles user input and navigation within the main menu.
///
/// # Arguments
///
/// * `writer` - A mutable reference to an `io::Write` implementor for writing to the terminal.
///
/// # Returns
///
/// Returns an `io::Result` that indicates success or failure.
fn main_menu_loop<W: io::Write>(writer: &mut W) -> io::Result<()> {
    let mut selected_option = NewGame;
    loop {
        render_main_menu(writer, &selected_option)?;

        match event::read()? {
            Event::Key(KeyEvent {
                code: c,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            }) => {
                match c {
                    KeyCode::Up => match selected_option {
                        NewGame => selected_option = Quit,
                        LoadGame => selected_option = NewGame,
                        Quit => selected_option = LoadGame,
                    },
                    KeyCode::Down => match selected_option {
                        NewGame => selected_option = LoadGame,
                        LoadGame => selected_option = Quit,
                        Quit => selected_option = NewGame,
                    },
                    KeyCode::Enter => {
                        match selected_option {
                            NewGame => {
                                writer.execute(Clear(ClearType::All))?;
                                game_loop(writer, Game::start_new_game())?;
                            }
                            LoadGame => {
                                unimplemented!()
                            }
                            Quit => {
                                return Ok(()); // breaks loop and allows cleanup code to run
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(_, _) => {
                writer.execute(Clear(ClearType::All))?;
                continue;
            }
            _ => {}
        }
        sleep(Duration::from_millis(100));
    }
}

/// Renders the main menu on the terminal.
///
/// This function draws the main menu options and highlights the selected option. All parameters
/// such as positions, sizes, etc are hardcoded and immutable.
///
/// # Arguments
///
/// * `writer` - A mutable reference to an `io::Write` implementor for writing to the terminal.
/// * `selected_option` - The currently selected main menu option. This option will be drawn in
///   yellow.
///
/// # Returns
///
/// Returns an `io::Result` that indicates success or failure.
fn render_main_menu<W: io::Write>(
    writer: &mut W,
    selected_option: &MainMenuOption,
) -> io::Result<()> {
    const MENU_BOX_WIDTH: u16 = 16;
    const MENU_BOX_HEIGHT: u16 = 5;

    let size = terminal::size()?;
    let menu_box_left_x = (size.0 - MENU_BOX_WIDTH) / 2;
    let menu_box_right_x = (size.0 + MENU_BOX_WIDTH) / 2 - 1;
    let menu_box_top_y = (size.1 - MENU_BOX_HEIGHT) / 2;
    let menu_box_bottom_y = (size.1 + MENU_BOX_HEIGHT) / 2 - 1;

    // draw box
    for y in menu_box_top_y..=menu_box_bottom_y {
        for x in menu_box_left_x..=menu_box_right_x {
            if (y == menu_box_top_y || y == menu_box_bottom_y)
                || (x == menu_box_left_x || x == menu_box_right_x)
            {
                let printed_char: char = match (x, y) {
                    (x, y) if (x == menu_box_left_x && y == menu_box_top_y) => '┌',
                    (x, y) if (x == menu_box_right_x && y == menu_box_top_y) => '┐',
                    (x, y) if (x == menu_box_left_x && y == menu_box_bottom_y) => '└',
                    (x, y) if (x == menu_box_right_x && y == menu_box_bottom_y) => '┘',
                    (x, _) if (x == menu_box_left_x || x == menu_box_right_x) => '│',
                    (_, y) if (y == menu_box_top_y || y == menu_box_bottom_y) => '─',
                    _ => unreachable!(),
                };
                queue!(
                    writer,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(printed_char.white())
                )?;
            }
        }
    }

    // draw text
    queue!(
        writer,
        cursor::MoveTo(menu_box_left_x + 1, menu_box_top_y + 1),
        style::SetForegroundColor(if *selected_option == NewGame {
            style::Color::Yellow
        } else {
            style::Color::White
        }),
        style::Print(get_padded_string("New Game", (MENU_BOX_WIDTH - 2) as usize)),
        cursor::MoveTo(menu_box_left_x + 1, menu_box_top_y + 2),
        style::SetForegroundColor(if *selected_option == LoadGame {
            style::Color::Yellow
        } else {
            style::Color::White
        }),
        style::Print(get_padded_string("Load", (MENU_BOX_WIDTH - 2) as usize)),
        cursor::MoveTo(menu_box_left_x + 1, menu_box_top_y + 3),
        style::SetForegroundColor(if *selected_option == Quit {
            style::Color::Yellow
        } else {
            style::Color::White
        }),
        style::Print(get_padded_string("Quit", (MENU_BOX_WIDTH - 2) as usize)),
    )?;

    writer.flush()?;

    Ok(())
}

/// Returns a padded string with specified width.
///
/// This function takes some text and pads it with spaces on both sides to
/// achieve the desired width. It ensures that the text is centered within the width.
/// If the text is longer than the desired width, just returns the text.
///
/// # Arguments
///
/// * `text` - The text to pad.
/// * `width` - The desired width of the padded string.
///
/// # Returns
///
/// A `String` containing the padded text.
fn get_padded_string(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text.to_string();
    }
    let num_spaces_on_left = (width - text.len()) / 2;
    let num_spaces_on_right = width - (num_spaces_on_left + text.len());
    format!(
        "{}{}{}",
        " ".repeat(num_spaces_on_left),
        text,
        " ".repeat(num_spaces_on_right)
    )
}

fn game_loop<W: io::Write>(
    writer: &mut W,
    initial_game_state: Result<Game, GameError>,
) -> io::Result<()> {
    render_everything_except_board(writer)?;
    let mut game_state = initial_game_state;

    loop {
        match &game_state {
            Err(err) => {
                render_game_state_error(writer, err);
            }
            Ok(game) => {
                render_board(writer, game)?;
            }
        }
        match event::read()? {
            Event::Key(KeyEvent {
                code: c,
                kind: KeyEventKind::Press,
                ..
            }) => match c {
                KeyCode::Up => {
                    game_state = game_state.unwrap().handle_event(GameEvent::SwipeUp);
                }
                KeyCode::Left => {
                    game_state = game_state.unwrap().handle_event(GameEvent::SwipeLeft);
                }
                KeyCode::Right => {
                    game_state = game_state.unwrap().handle_event(GameEvent::SwipeRight);
                }
                KeyCode::Down => {
                    game_state = game_state.unwrap().handle_event(GameEvent::SwipeDown);
                }
                KeyCode::Char('q') => {
                    writer.execute(Clear(ClearType::All))?;
                    break;
                }
                KeyCode::Char('r') => {
                    game_state = game_state.unwrap().handle_event(GameEvent::NewGame);
                }
                _ => {}
            },
            Event::Resize(_, _) => {
                let game = game_state.unwrap();
                render_everything_except_board(writer)?;
                render_board(writer, &game)?;
                game_state = Ok(game);
            }
            _ => {}
        }
        sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn render_everything_except_board<W: io::Write>(writer: &mut W) -> io::Result<()> {
    writer.queue(Clear(ClearType::All))?;

    let size = terminal::size()?;
    let controls = " Arrow Keys: Merge  R: Restart  Q: Quit";
    queue!(
        writer,
        cursor::MoveTo(0, size.1),
        style::SetBackgroundColor(Color::White),
        style::SetForegroundColor(Color::Black),
        style::Print(format!(
            "{}{}",
            controls,
            " ".repeat(size.0 as usize - controls.chars().count())
        )),
        style::ResetColor
    )?;

    //todo draw score

    writer.flush()?;
    Ok(())
}

fn render_board<W: io::Write>(writer: &mut W, game: &Game) -> io::Result<()> {
    let game_state = game.read_board_state();
    let max_item_length = game_state.iter().fold(0usize, |max_row_len, vec| {
        cmp::max(
            max_row_len,
            vec.iter().fold(0usize, |max_item_len, item| {
                cmp::max(max_item_len, (2u32.pow(*item as u32)).to_string().len())
            }),
        )
    });

    let size = terminal::size()?;

    let cell_width = max_item_length + 2; // add two for a space on each side
    let grid_width = game_state[0].len();

    let board_height = game_state.len() * 4; // in rows
    let board_width = (cell_width + 1) * grid_width + 1; // in columns

    let board_left_side_x_pos = (size.0 - board_width as u16) / 2;
    let board_top_side_y_pos = (size.1 - board_height as u16) / 2;

    for (index, row) in game_state.iter().enumerate() {
        queue!(
            writer,
            cursor::MoveTo(
                board_left_side_x_pos,
                board_top_side_y_pos + (4 * index as u16) + 1
            ),
            style::Print(create_data_row_without_text(cell_width, '│', '│', '│', row)),
            cursor::MoveTo(
                board_left_side_x_pos,
                board_top_side_y_pos + (4 * index as u16) + 2
            ),
            style::Print(create_data_row(cell_width, '│', '│', '│', row)),
            cursor::MoveTo(
                board_left_side_x_pos,
                board_top_side_y_pos + (4 * index as u16) + 3
            ),
            style::Print(create_data_row_without_text(cell_width, '│', '│', '│', row)),
            cursor::MoveTo(
                board_left_side_x_pos,
                board_top_side_y_pos + (4 * index as u16) + 4
            ),
            style::Print(create_constant_row(
                grid_width, cell_width, '├', '┼', '┤', '─'
            )),
        )?;
    }

    // draw top and bottom borders
    queue!(
        writer,
        cursor::MoveTo(board_left_side_x_pos, board_top_side_y_pos),
        style::Print(create_constant_row(grid_width, cell_width, '┌', '┬', '┐', '─').as_str()),
        cursor::MoveTo(
            board_left_side_x_pos,
            board_top_side_y_pos + board_height as u16
        ),
        style::Print(create_constant_row(grid_width, cell_width, '└', '┴', '┘', '─').as_str())
    )?;

    Ok(())
}

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

fn create_data_row(
    cell_width: usize,
    opening_char: char,
    joining_char: char,
    closing_char: char,
    data: &[TileType],
) -> String {
    format!(
        "{}{}{}\n",
        opening_char.white().on_black(),
        data.iter()
            .map(|&tile| format_tile_for_display_with_number(tile, cell_width).to_string())
            .collect::<Vec<String>>()
            .join(joining_char.white().on_black().to_string().as_str()),
        closing_char.white().on_black()
    )
}

fn create_data_row_without_text(
    cell_width: usize,
    opening_char: char,
    joining_char: char,
    closing_char: char,
    data: &[TileType],
) -> String {
    format!(
        "{}{}{}\n",
        opening_char.white().on_black(),
        data.iter()
            .map(|&tile| format_tile_for_display_without_number(tile, cell_width).to_string())
            .collect::<Vec<String>>()
            .join(joining_char.white().on_black().to_string().as_str()),
        closing_char.white().on_black()
    )
}

fn format_tile_for_display_without_number(
    tile: TileType,
    cell_width: usize,
) -> StyledContent<String> {
    let padded_string = " ".repeat(cell_width);
    match tile {
        0 => padded_string.on_black(),
        1 => padded_string.on_white(),
        2 => padded_string.on_white(),
        3 => padded_string.on_yellow(),
        4 => padded_string.on_yellow(),
        5 => padded_string.on_yellow(),
        6 => padded_string.on_red(),
        7 => padded_string.on_red(),
        8 => padded_string.on_red(),
        9 => padded_string.on_magenta(),
        10 => padded_string.on_magenta(),
        11 => padded_string.on_magenta(),
        12 => padded_string.on_cyan(),
        13 => padded_string.on_cyan(),
        14 => padded_string.on_cyan(),
        15 => padded_string.on_green(),
        16 => padded_string.on_green(),
        _ => padded_string.on_green(),
    }
}

fn format_tile_for_display_with_number(tile: TileType, cell_width: usize) -> StyledContent<String> {
    let number_as_string = if tile == 0 {
        " ".to_string()
    } else {
        2u32.pow(tile as u32).to_string()
    };

    let spaces_before = (cell_width - number_as_string.len()) / 2;
    let spaces_after = (cell_width - number_as_string.len()) - spaces_before; // subtract here because spaces_before and spaces_after aren't equal if cell_width - item length is odd, and want all cells to be consistent width
    let padded_string = format!(
        "{}{}{}",
        " ".repeat(spaces_before),
        number_as_string,
        " ".repeat(spaces_after)
    );
    match tile {
        0 => padded_string.white().on_black(),
        1 => padded_string.black().on_white(),
        2 => padded_string.black().on_white(),
        3 => padded_string.black().on_yellow(),
        4 => padded_string.black().on_yellow(),
        5 => padded_string.black().on_yellow(),
        6 => padded_string.white().on_red(),
        7 => padded_string.white().on_red(),
        8 => padded_string.white().on_red(),
        9 => padded_string.black().on_magenta(),
        10 => padded_string.black().on_magenta(),
        11 => padded_string.black().on_magenta(),
        12 => padded_string.black().on_cyan(),
        13 => padded_string.black().on_cyan(),
        14 => padded_string.black().on_cyan(),
        15 => padded_string.black().on_green(),
        16 => padded_string.black().on_green(),
        _ => padded_string.black().on_green(),
    }
}

fn render_game_state_error<W: io::Write>(writer: &mut W, e: &GameError) -> ! {
    // this function always exits the program anyway, so if printing the error fails
    // we just panic
    queue!(
        writer,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print("Cannot continue the game. Error: "),
    )
    .unwrap();
    let substrings: Vec<String> = format!("{:#?}", e)
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    for str in substrings {
        queue!(writer, cursor::MoveDown(1), style::Print(str)).unwrap();
    }
    queue!(
        writer,
        cursor::MoveDown(1),
        style::Print("Press any key to exit the game.")
    )
    .unwrap();
    writer.flush().unwrap();

    loop {
        if let Ok(Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            writer
                .execute(terminal::LeaveAlternateScreen)
                .expect("Couldn't leave alternate screen buffer");
            exit(1);
        }
    }
}
