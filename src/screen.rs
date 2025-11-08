use std::io::{Write};

use crate::{constants::ESC, Buffer, Cursor};

#[allow(dead_code)]
#[derive(Default)]
pub struct Screen {
    width: u16,
    height: u16,
    buffer: Option<Buffer>,
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
            buffer: None,
        })
    }

    pub fn set_current_buffer(&mut self, buffer: Buffer) {
        self.buffer = Some(buffer);
    }

    pub fn remove_current_buffer(&mut self) {
        self.buffer = None;
    }

    pub fn draw_current_buffer(&mut self) {
        if let Some(buffer) = &mut self.buffer {
            Self::erase();
            
            let cursor = &mut buffer.cursor;
            for cell in &buffer.cells {
                cursor.jump(cell.row, cell.col);
                print!("{}", cell.char);
            }
        }
        Self::flush();
    }

    pub fn flush() {
        let _ = std::io::stdout().flush();
    }

    pub fn erase() {
        print!("{}[2J", ESC);
    }
}
