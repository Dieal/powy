use std::{env, fs, io, thread::sleep, time::Duration};

use crossterm::terminal::enable_raw_mode;
use text_editor::{editor::Editor, screen::Screen};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut editor = Editor::new()?;
    let mut args = env::args();
    args.next(); // Skips binary name
    let args: Vec<String> = args.collect();

    if !args.is_empty() {
        let path = args.first().expect("Should have first argument");
        if let Ok(text) = fs::read_to_string(path) {
            editor.add_buffer_from_text(text);
            editor.set_screen_buffer_from_index(0);
        } else {
            print!("File {path} not found");
        }
    }
    editor.run();
    Screen::flush();

    Ok(())
}
