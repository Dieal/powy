use std::{io, thread::sleep, time::Duration};

use crossterm::terminal::enable_raw_mode;
use text_editor::{editor::Editor, screen::Screen, Buffer, Cell};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let text = String::from("Prova 12345");
    let mut editor = Editor::new()?;
    editor.add_buffer_from_text(text);
    editor.set_screen_buffer_from_index(0);
    editor.draw_current_buffer();
    Screen::flush();
    sleep(Duration::from_secs(3));

    Ok(())
}
