use constants::{Direction, ESC};
use log::{debug, info};

mod constants;
pub mod screen;
pub mod editor;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Cursor {
    row: usize,
    col: usize,
    visible: bool,
}

// TODO: add cursor styles enum

impl Cursor {
    pub fn move_by(&mut self, direction: Direction, offset: usize) {
        match direction {
            Direction::Up => self.row = self.row.saturating_sub(offset),
            Direction::Down => self.row = self.row.saturating_add(offset),
            Direction::Left => self.col = self.col.saturating_sub(offset),
            Direction::Right => self.col = self.col.saturating_add(offset),
        }
    }

    pub fn set_visibility(&mut self, visibile: bool) {
        self.visible = visibile;
        self.render_invisibility();
    }

    pub fn jump(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        print!("{}[{row};{col}H", ESC);
    }

    pub fn jump_to_row(&mut self, row: usize) {
        self.jump(row, self.col);
    }

    pub fn jump_to_col(&mut self, col: usize) {
        self.jump(self.row, col);
    }

    // Function that moves terminal cursor to the current state
    pub fn render(&mut self) {
        self.jump(self.row, self.col);
        self.render_invisibility();
    }

    fn render_invisibility(&self) {
        if self.visible {
            print!("{ESC}[?25h");
        } else {
            print!("{ESC}[?25l"); // Become invisible
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        let (row, col) = (0, 0);
        let mut cursor = Cursor {
            row,
            col,
            visible: true,
        };
        cursor.set_visibility(true);
        cursor.jump(row, col);
        print!("{ESC}[6 q");
        cursor
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Buffer {
    pub lines: Vec<String>,
    cursor: Cursor, // Cursor used by the user
    ghost_cursor: Cursor, // Cursor used to render the UI, needs to be invisible
    path: Option<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        let mut cursor = Cursor::default();
        cursor.jump_to_col(1); // Starts from column 1 so that the cursor is in front of the character

        let mut ghost_cursor = Cursor::default();
        ghost_cursor.set_visibility(false);

        Self {
            lines: vec![String::new()],
            cursor,
            ghost_cursor,
            path: None,
        }
    }
}

impl Buffer {
    fn from_text(text: String) -> Self {
        Self {
            lines: text.lines().map(String::from).collect(),
            ..Default::default()
        }
    }

    pub fn insert_char(&mut self, char: char) {
        let cursor = &mut self.cursor;
        let (row, col) = (cursor.row, cursor.col);
        if let Some(line) = self.lines.get_mut(row) {
            line.insert(col - 1, char);
        } else {
            self.lines.push(String::from(char));
        }
        cursor.move_by(Direction::Right, 1);
    }

    fn remove_char(&mut self) {
        let cursor = &mut self.cursor;
        let (row, col) = (cursor.row, cursor.col);
        if let Some(line) = self.lines.get_mut(row) {
            line.pop();
            if cursor.col > 1 { // Bounds the column to 1 (see the constructor for the reasons)
                cursor.jump_to_col(col.saturating_sub(1));
            }
        }
    }

    pub fn insert_text(&mut self, text: String) {
        let cursor = &self.cursor;
        let (row, col) = (cursor.row, cursor.col);
        if let Some(line) = self.lines.get_mut(row) {
            line.insert_str(col, text.as_str());
        } else {
            self.lines.push(text);
        }
    }
}
