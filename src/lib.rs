use constants::{Direction, ESC};

mod constants;
pub mod screen;
pub mod editor;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Cursor {
    row: u16,
    column: u16,
}

impl Cursor {
    pub fn move_by(&mut self, direction: Direction, offset: u16) {
        match direction {
            Direction::Up => self.row = self.row.saturating_sub(offset),
            Direction::Down => self.row = self.row.saturating_add(offset),
            Direction::Left => self.column = self.column.saturating_sub(offset),
            Direction::Right => self.column = self.column.saturating_add(offset),
        }
    }

    pub fn jump(&mut self, row: u16, col: u16) {
        print!("{}[{row};{col}H", ESC);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Cell {
    pub char: char,
    pub row: u16,
    pub col: u16
}

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Buffer {
    pub cells: Vec<Cell>,
    cursor: Cursor,
    path: Option<String>,
}

impl Buffer {
    fn from_text(text: String) -> Buffer {
        let mut cells: Vec<Cell> = Vec::new();
        let (mut row, mut col) = (0, 1);
        for line in text.lines() {
            col = 1;
            for char in line.chars() {
                cells.push(Cell { char, row, col, });
                col += 1;
            }
            row += 1;
        }

        Buffer {
            cells,
            cursor: Cursor::default(),
            path: None,
        }
    }
}

