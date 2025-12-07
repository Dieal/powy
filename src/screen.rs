use std::io::{Write};

use crate::{constants::{Direction, ESC}, Buffer};

#[allow(dead_code)]
#[derive(Default)]
pub struct Screen {
    width: u16,
    height: u16,
}

#[allow(dead_code)]
impl Screen {
    pub fn new() -> std::io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Self::erase();
        Self::flush();
        Ok(Screen {
            width,
            height,
        })
    }

    pub fn draw_buffer(&mut self, buffer: &mut Buffer) {
        Self::erase();

        let ghost_cursor = &mut buffer.ghost_cursor;
        ghost_cursor.render();
        ghost_cursor.jump(0, 0);
        for line in &buffer.lines {
            print!("{}", line);
            ghost_cursor.jump_to_col(0);
            ghost_cursor.move_by(Direction::Down, 1);
        }
        buffer.cursor.render();
        Self::flush();
    }

    pub fn flush() {
        let _ = std::io::stdout().flush();
    }

    pub fn erase() {
        print!("{}[2J", ESC);
    }
}
