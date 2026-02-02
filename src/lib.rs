use constants::{Direction, ESC};
use log::{debug, info};

mod constants;
pub mod screen;
pub mod editor;

#[derive(Clone, Default)]
pub enum CursorStyle {
    BlinkingBlock,

    #[default]
    SteadyBlock,

    BlinkingUnderline,
    SteadyUnderline,
    BlinkingBar,
    SteadyBar,
}

impl CursorStyle {
    // https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Functions-using-CSI-_-ordered-by-the-final-character_s_, 
    // "Set cursor style" section
    fn value(&self) -> u8 {
        match self {
            Self::BlinkingBlock => 1,
            Self::SteadyBlock => 2,
            Self::BlinkingUnderline => 3,
            Self::SteadyUnderline => 4,
            Self::BlinkingBar => 5,
            Self::SteadyBar => 6,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Cursor {
    row: usize,
    col: usize,
    visible: bool,
    style: CursorStyle,
}

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
        self.render_style();
        self.render_invisibility();
    }

    fn render_invisibility(&self) {
        if self.visible {
            print!("{ESC}[?25h");
        } else {
            print!("{ESC}[?25l"); // Become invisible
        }
    }

    fn render_style(&self) {
        print!("{ESC}[{} q", self.style.value());
    }

    pub fn set_style (&mut self, style: CursorStyle) {
        self.style = style;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        let (row, col) = (0, 0);
        let mut cursor = Cursor {
            row,
            col,
            visible: true,
            style: CursorStyle::default(),
        };
        cursor.set_visibility(true);
        cursor.jump(row, col);
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
        cursor.jump(1, 1); // Starts from column and row 1 for better rendering

        let mut ghost_cursor = Cursor::default();
        ghost_cursor.set_visibility(false);
        info!("{} {}", cursor.row, cursor.col);

        Self {
            lines: vec![String::new()],
            cursor,
            ghost_cursor,
            path: None,
        }
    }
}

impl Buffer {
    pub fn from_text(text: String) -> Self {
        Self {
            lines: text.lines().map(String::from).collect(),
            ..Default::default()
        }
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cursor.set_style(style);
    }

    fn insert_char(&mut self, char: char) {
        let cursor = &mut self.cursor;
        let (row, col) = (cursor.row, cursor.col);
        if let Some(line) = self.lines.get_mut(row.saturating_sub(1)) {
            line.insert(col.saturating_sub(1), char);
        } else {
            self.lines.push(String::from(char));
        }
        cursor.move_by(Direction::Right, 1);
    }

    fn remove_char(&mut self) {
        let cursor = &mut self.cursor;
        let (row, col) = (cursor.row, cursor.col);

        // Cursor row starts from one, but lines indexes start from 0
        // TODO: add an abstraction layer for this problem
        let line_row = row.saturating_sub(1);
        if let Some(line) = self.lines.get_mut(line_row) {
            if line.is_empty() {
                if line_row >= 1 {
                    self.lines.remove(line_row);
                    let top_row = line_row.saturating_sub(1);
                    if let Some(top_line) = self.lines.get(top_row) { // Line Row has become the index of the top line
                        cursor.jump(line_row, top_line.len() + 1);
                    }
                }
                return;
            }

            line.pop();
            if cursor.col > 1 { // Bounds the column to 1 (see the constructor for the reasons)
                cursor.jump_to_col(col.saturating_sub(1));
            }
        }
    }

    fn new_line(&mut self) {
        let cursor = &mut self.cursor;
        self.lines.push(String::new());
        cursor.jump(cursor.row + 1, 1);
        debug!("New Line...\n{:#?}", self.lines);
    }

    fn get_row_index_from_cursor(&self) -> usize {
        self.cursor.row.saturating_sub(1)
    }

    fn get_current_row(&self) -> Option<&String> {
        self.lines.get(self.cursor.row.saturating_sub(1))
    }

    fn get_current_row_mut(&mut self) -> Option<&mut String> {
        self.lines.get_mut(self.cursor.row.saturating_sub(1))
    }

    fn insert_text(&mut self, text: String) {
        let cursor = &self.cursor;
        let (row, col) = (cursor.row, cursor.col);
        if let Some(line) = self.lines.get_mut(row) {
            line.insert_str(col, text.as_str());
        } else {
            self.lines.push(text);
        }
    }
}
