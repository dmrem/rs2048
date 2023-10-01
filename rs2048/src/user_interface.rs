use crate::game::{Game, GameError};
use crate::user_interface::MainMenuOption::{LoadGame, NewGame, Quit};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, event, queue, style, terminal, ExecutableCommand};
use std::io;

#[derive(Debug, Eq, PartialEq)]
enum MainMenuOption {
    NewGame,
    LoadGame,
    Quit,
}

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
                        return Ok(()); // will eventually start main game loop, not yet implemented
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
    }
}

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
        Print(get_padded_string("New Game", (MENU_BOX_WIDTH - 2) as usize)),
        cursor::MoveTo(menu_box_left_x + 1, menu_box_top_y + 2),
        style::SetForegroundColor(if *selected_option == LoadGame {
            style::Color::Yellow
        } else {
            style::Color::White
        }),
        Print(get_padded_string("Load", (MENU_BOX_WIDTH - 2) as usize)),
        cursor::MoveTo(menu_box_left_x + 1, menu_box_top_y + 3),
        style::SetForegroundColor(if *selected_option == Quit {
            style::Color::Yellow
        } else {
            style::Color::White
        }),
        Print(get_padded_string("Quit", (MENU_BOX_WIDTH - 2) as usize)),
    )?;

    writer.flush()?;

    Ok(())
}

fn get_padded_string(text: &str, width: usize) -> String {
    assert!(text.len() < width);
    let num_spaces_on_left = (width - text.len()) / 2;
    let num_spaces_on_right = width - (num_spaces_on_left + text.len());
    format!(
        "{}{}{}",
        " ".repeat(num_spaces_on_left),
        text,
        " ".repeat(num_spaces_on_right)
    )
}

fn game_loop(initial_game_state: Result<Game, GameError>) {
    let mut game_state = initial_game_state;
    loop {}
}

fn render_game(game_state: Result<Game, GameError>) {
    match game_state {
        Err(e) => todo!(),
        Ok(game) => todo!(),
    }
}
