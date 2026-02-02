use std::{fs, path::Path};

use crossterm::event::{read, Event::Key, KeyCode};
use log::{debug, info};

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

    pub fn from_file(text: String, path: String) -> std::io::Result<Self> {
        let mut buffer = Buffer::from_text(text);
        buffer.path = Some(path);

        let buffers: Vec<Buffer> = vec![buffer];
        Ok(Editor {
            buffers,
            ..Editor::new()?
        })
    }

    pub fn run(&mut self) {
        self.draw_current_buffer();

        loop {
            let mode: EditorMode = self.mode;
            if let Ok(Key(key)) = read() {
                let current_buf = self.get_current_buffer_mut().expect("Should have a default current buffer");
                let buffer_len = current_buf.lines.len();
                let cursor_row = current_buf.cursor.row;
                let cursor_col = current_buf.cursor.col;
                let current_row_size = if let Some(current_row) = current_buf.get_current_row() {
                    current_row.len()
                } else {
                    0
                };

                match mode {
                    EditorMode::Normal => {
                        match key.code {
                            KeyCode::Tab => self.save_current_buffer_to_disk(),
                            KeyCode::Esc => break,
                            KeyCode::Left | KeyCode::Char('h') => current_buf.cursor.move_by(crate::constants::Direction::Left, 1),
                            KeyCode::Right | KeyCode::Char('l') => {
                                if cursor_col < current_row_size { // Prevents cursor from going out of line bounds
                                    current_buf.cursor.move_by(crate::constants::Direction::Right, 1);
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                info!("{cursor_row}, {buffer_len}");
                                if cursor_row < buffer_len {
                                    current_buf.cursor.move_by(crate::constants::Direction::Down, 1)
                                }
                            },
                            KeyCode::Up | KeyCode::Char('k') => current_buf.cursor.move_by(crate::constants::Direction::Up, 1),
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
                            KeyCode::Right => {
                                if cursor_col < current_row_size { // Prevents cursor from going out of line bounds
                                    current_buf.cursor.move_by(crate::constants::Direction::Right, 1);
                                }
                            }
                            KeyCode::Up => current_buf.cursor.move_by(crate::constants::Direction::Up, 1),
                            KeyCode::Down => {
                                info!("{cursor_row}, {buffer_len}");
                                if cursor_row < buffer_len {
                                    current_buf.cursor.move_by(crate::constants::Direction::Down, 1)
                                }
                            },
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

    //  TODO: use future IO manager struct to directly read from file. Move logic from main to a separate struct inside the editor
    pub fn add_buffer_from_file(&mut self, text: String, path: String) -> &Buffer {
        let mut buffer = Buffer::from_text(text);
        buffer.path = Some(path);
        self.add_buffer_from(buffer)
    }

    pub fn draw_current_buffer(&mut self) {
        if let Some(current_buffer) = self.buffers.get_mut(self.current_buffer_index) {
            self.screen.draw_buffer(current_buffer);
        }
    }

    //  TODO: make it more efficient, don't write it from start each time, incremental
    // saving
    //  TODO: move this to a separate IO manager (read and write, different STDIN / STDOUT)
    fn save_current_buffer_to_disk(&self) {
        let buffer = self.get_current_buffer();
        if let Some(buffer) = buffer {
            if let Some(path) = &buffer.path {
                let mut contents = String::new();
                for line in &buffer.lines {
                    contents.push_str(line);
                    contents.push('\n');
                }
                fs::write(Path::new(path), contents);
                debug!("Wrote file")
            } else {
                //  TODO: Prompt to create a new file, dialog to insert file name
                debug!("no path")
            }
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
