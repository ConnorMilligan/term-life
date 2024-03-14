use std::io::{self, Write};
use crossterm::{
    cursor,
    style,
    QueueableCommand
};
use crate::util::read_char;

struct Game {
    stdout: std::io::Stdout,
    running: bool,
}

pub fn game_loop() -> io::Result<()> {
    let mut game = Game {
        stdout: std::io::stdout(),
        running: true,
    };

    while game.running {
        draw(&mut game)?;
        take_input(&mut game)?;
    }

    Ok(())
}

fn draw(game: &mut Game) -> io::Result<()> {
    game.stdout.queue(cursor::MoveTo(0, 0))?;
    game.stdout.queue(style::Print("Hello, world!"))?;
    game.stdout.flush()?;
    Ok(())
}

fn take_input(game: &mut Game) -> io::Result<()> {
    match read_char()? {
        'q' => {
            game.running = false;
        }
        _ => {}
    }
    Ok(())
}
