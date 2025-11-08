use std::{io, thread::sleep, time::Duration};

use crossterm::terminal::enable_raw_mode;
use text_editor::{editor::Editor, Buffer, Cell};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut editor = Editor::new()?;
    let screen = editor.get_screen_mut();
    let mut buffer = Buffer::default();
    buffer.cells.push(Cell {
        char: 'c',
        row: 1,
        col: 1,
    });

    buffer.cells.push(Cell {
        char: 'a',
        row: 1,
        col: 2,
    });

    buffer.cells.push(Cell {
        char: 'o',
        row: 1,
        col: 3,
    });
    screen.set_current_buffer(buffer);
    screen.draw_current_buffer();
    sleep(Duration::from_secs(3));

    Ok(())
}
