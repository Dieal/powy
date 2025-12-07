use std::{env, fs::{self, File}, io};

use crossterm::terminal::enable_raw_mode;
use log::info;
use simplelog::{CombinedLogger, Config, WriteLogger};
use text_editor::{editor::Editor, screen::Screen};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let _ = CombinedLogger::init(vec![
        WriteLogger::new(simplelog::LevelFilter::Debug, Config::default(), File::create("./debug.log")?),
    ]);

    let mut editor = Editor::new()?;
    let mut args = env::args();
    args.next(); // Skips binary name
    let args: Vec<String> = args.collect();

    if !args.is_empty() {
        let path = args.first().expect("Should have first argument");
        if let Ok(text) = fs::read_to_string(path) {
            editor.add_buffer_from_text(text);
        } else {
            print!("File {path} not found");
        }
    }
    editor.run();
    Screen::flush();

    Ok(())
}
