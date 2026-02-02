use std::{env, fs::{self, File}, io};

use crossterm::terminal::enable_raw_mode;
use simplelog::{CombinedLogger, Config, WriteLogger};
use text_editor::{Buffer, editor::Editor, screen::Screen};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let _ = CombinedLogger::init(vec![
        WriteLogger::new(simplelog::LevelFilter::Debug, Config::default(), File::create("./debug.log")?),
    ]);

    let mut args = env::args();
    args.next(); // Skips binary name
    let args: Vec<String> = args.collect();

    let mut editor: Editor = if args.is_empty() {
        Editor::new()
    } else {
        let path = args.first().expect("Should have first argument");
        if let Ok(text) = fs::read_to_string(path) {
            Editor::from_buffer(Buffer::from_text(text))
        } else {
            print!("File {path} not found");
            Editor::new()
        }
    }?;

    editor.run();
    Screen::flush();

    Ok(())
}
