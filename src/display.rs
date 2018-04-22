//! Handle display routines.

use std::iter;
use std::io::stdin;

use termion::{
    color,
    terminal_size,
};
use termion::cursor::Goto;

pub const DEFAULT_STATUS: &str = "Waiting. Type 'help' to get the commands list.";

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

/// Update the content of the status text bar.
///
/// Args:
///
/// `text` - the text to display into the text bar
pub fn set_status_text(text: &str) {

    println!("{}", Goto(0, get_terminal_height() - 2));
    display_text_bar(text);
    println!("{}", Goto(0, 2));
}

/// Clear the whole terminal content and generate the default content (bars and titles). Refactored as used multiple times.
pub fn clear_screen() {

    /* send a control character to the terminal */
    print!("{}[2J", 27 as char);

    println!("{}", Goto(1, 1));
    const TITLE: &str = "rust-blockchain";
    display_text_bar(TITLE);
}

/// Handles user input and returns that input as a string.
///
/// Returns:
///
/// user input as string
pub fn get_input() -> String {

    println!("{}", Goto(0, get_terminal_height() - 3));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    clear_screen();
    println!("{}", Goto(0, 2));

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
