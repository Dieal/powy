use crossterm::event::{self, read, Event::Key, KeyCode, ModifierKeyCode};

use crate::{Buffer, CursorStyle, screen::Screen};

#[derive(Default, Copy, Clone)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
    // Visual
}

#[derive(Default)]
pub struct Editor {
    screen: Screen,
    buffers: Vec<Buffer>,
    current_buffer_index: usize,
    mode: EditorMode,
}

impl Editor {
    pub fn new() -> std::io::Result<Self> {
        let buffers: Vec<Buffer> = vec![Buffer::default()];
        Ok(Editor {
            screen: Screen::new()?,
            buffers,
            current_buffer_index: 0,
            mode: EditorMode::Normal,
        })
    }
    
    pub fn from_buffer(buffer: Buffer) -> std::io::Result<Self> {
        let buffers: Vec<Buffer> = vec![buffer];
        Ok(Editor {
            buffers,
            ..Editor::new()?
        })
    }

    pub fn run(&mut self) {
        self.draw_current_buffer();

        //  TODO: Add normal, insert and visual mode
        loop {
            let mode: EditorMode = self.mode;
            if let Ok(Key(key)) = read() {
                let current_buf = self.get_current_buffer_mut().expect("Should have a default current buffer");
                match mode {
                    EditorMode::Normal => {
                        match key.code {
                            KeyCode::Esc => break,
                            KeyCode::Left | KeyCode::Char('h') => current_buf.cursor.move_by(crate::constants::Direction::Left, 1),
                            KeyCode::Right | KeyCode::Char('l') => current_buf.cursor.move_by(crate::constants::Direction::Right, 1), // TODO: Add Check if it's possible
                            KeyCode::Char('i') => self.set_mode(EditorMode::Insert),

                            // TODO: vim commands
                            KeyCode::Char(_c) => {
                            },
                            _ => (),
                        }
                    },
                    EditorMode::Insert => {
                        match key.code {
                            KeyCode::Esc => self.set_mode(EditorMode::Normal),
                            KeyCode::Left => current_buf.cursor.move_by(crate::constants::Direction::Left, 1),
                            KeyCode::Right => current_buf.cursor.move_by(crate::constants::Direction::Right, 1), // TODO: Add Check if it's possible
                            KeyCode::Backspace => current_buf.remove_char(),
                            KeyCode::Enter => current_buf.new_line(),
                            KeyCode::Char(c) => current_buf.insert_char(c), // Update buffer with character
                            _ => (),
                        }

                    }
                }
            }
            self.draw_current_buffer();
        }
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn get_buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }

    pub fn add_buffer(&mut self) -> &Buffer {
        self.buffers.push(Buffer::default());
        self.buffers.last().expect("Should exist")
    }

    pub fn add_buffer_from(&mut self, buffer: Buffer) -> &Buffer{
        self.buffers.push(buffer);
        self.buffers.last().expect("Should exist")
    }

    pub fn add_buffer_from_text(&mut self, text: String) -> &Buffer {
        self.add_buffer_from(Buffer::from_text(text))
    }

    pub fn draw_current_buffer(&mut self) {
        if let Some(current_buffer) = self.buffers.get_mut(self.current_buffer_index) {
            self.screen.draw_buffer(current_buffer);
        }
    }

    pub fn get_current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.buffers.get_mut(self.current_buffer_index)
    }

    pub fn get_current_buffer(&self) -> Option<&Buffer> {
        self.buffers.get(self.current_buffer_index)
    }

    pub fn remove_buffer(&mut self, index: usize) -> Option<Buffer> {
        if index < self.buffers.len() {
            return Some(self.buffers.remove(index));
        }
        None
    }

    fn set_mode(&mut self, mode: EditorMode) {
        if let Some(buffer) = self.get_current_buffer_mut() {
            match mode {
                EditorMode::Normal => buffer.set_cursor_style(CursorStyle::SteadyBlock),
                EditorMode::Insert => buffer.set_cursor_style(CursorStyle::SteadyBar),
            }
        }
        self.mode = mode;
    }
}
