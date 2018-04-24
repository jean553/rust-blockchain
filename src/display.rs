//! Handle display routines.

use std::iter;
use std::io::stdin;

use termion::{
    color,
    terminal_size,
    clear,
};
use termion::cursor::Goto;

/// Display the given text into an horizontal bar. Refactored here as it is used when the screen is reset but also when the status bar content is updated.
///
/// Args:
///
/// `text` - the text to display into the text bar
pub fn display_text_bar(text: &str) {

    println!(
        "{}{}{}{}{}{}",
        color::Bg(color::Blue),
        color::Fg(color::White),
        text,
        iter::repeat(' ')
            .take(terminal_size().unwrap().0 as usize - text.len())
            .collect::<String>(),
        color::Bg(color::Reset),
        color::Fg(color::Reset),
    );
}

/// Clear the whole terminal content and generate the default content (bars and titles). Refactored as used multiple times.
pub fn clear_screen() {

    println!("{}", clear::All);

    println!("{}", Goto(1, 1));
    const TITLE: &str = "rust-blockchain";
    display_text_bar(TITLE);

    println!("{}", Goto(0, get_terminal_height() - 3));
    display_text_bar("");

    println!("{}", Goto(0, 2));
}

/// Handles user input and returns that input as a string.
///
/// Returns:
///
/// user input as string
pub fn get_input() -> String {

    set_cursor_into_input();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    clear_screen();

    set_cursor_into_logs();

    input.trim().to_string()
}

/// Determinates the terminal height through an external crate and returns it. Refactored as it is used multiple times. Calculating the height everytime might be a bad idea, but this is the simplest one to prevent passing the height to every function that need it.
///
/// Returns:
///
/// the terminal height
fn get_terminal_height() -> u16 {

    let (_, height) = terminal_size().unwrap();
    height as u16
}

/// Set the cursor position at the logs area.
pub fn set_cursor_into_logs() {
    println!("{}", Goto(0, 2));
}

/// Set the cursor position at the input area.
pub fn set_cursor_into_input() {
    println!("{}", Goto(0, get_terminal_height() - 2));
}
