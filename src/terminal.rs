use std::io::{self, Write};

use crossterm::{
    cursor,
    terminal,
    style::{self, Colors, SetColors},
    QueueableCommand,
};

pub fn setup_terminal() -> io::Result<()> {
    let mut stdout = io::stdout();

    // enable raw mode
    terminal::enable_raw_mode()?;

    stdout.queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::Hide)?
        .queue(terminal::EnterAlternateScreen)?
        .queue(SetColors(Colors::new(style::Color::White, style::Color::Black)))?
        .flush()?;

    Ok(())
}

pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.queue(cursor::Show)?
        .queue(terminal::LeaveAlternateScreen)?
        .flush()?;

    // disable raw mode
    terminal::disable_raw_mode()?;

    Ok(())
}
