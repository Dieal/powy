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

    pub fn add_buffer_from(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
    }

    pub fn set_screen_buffer(&mut self, buffer: Buffer) {
        self.screen.set_current_buffer(buffer);
    }

    pub fn add_buffer_from_text(&mut self, text: String) -> &Buffer {
        todo!();
        // self.buffers.push(buffer);
        self.buffers.last().expect("Should exist")
    }

    pub fn remove_buffer(&mut self, index: usize) -> Option<Buffer> {
        if index < self.buffers.len() {
            return Some(self.buffers.remove(index));
        }
        None
    }
}
