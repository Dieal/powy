use crossterm::event::{self, read, Event::Key, KeyCode};

use crate::{screen::Screen, Buffer};

#[derive(Default)]
pub struct Editor {
    screen: Screen,
    buffers: Vec<Buffer>,
}

impl Editor {
    pub fn new() -> std::io::Result<Self> {
        Ok(Editor {
            screen: Screen::new()?,
            buffers: Vec::new(),
        })
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

    pub fn set_screen_buffer(&mut self, buffer: Buffer) {
        self.screen.set_current_buffer(buffer);
    }

    pub fn set_screen_buffer_from_index(&mut self, index: usize) {
        if let Some(buffer) = self.buffers.get(index) {
            self.screen.set_current_buffer(buffer.clone()); // TODO Make screen accept a reference (need to use lifetime annotations)
        }
    }

    pub fn draw_current_buffer(&mut self) {
        self.screen.draw_current_buffer();
    }

    pub fn remove_buffer(&mut self, index: usize) -> Option<Buffer> {
        if index < self.buffers.len() {
            return Some(self.buffers.remove(index));
        }
        None
    }

    pub fn run(&mut self) {
        self.screen.draw_current_buffer();

        // TODO Add normal, insert and visual mode
        loop {
            if let Ok(Key(key)) = read() {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => (), // Update buffer with character
                    _ => (),
                }
            }
            self.screen.draw_current_buffer();
        }
    }
}
