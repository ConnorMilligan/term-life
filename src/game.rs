use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::menu::draw_menu;

pub struct Game {
    pub stdout: std::io::Stdout,
    pub running: bool,
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
    draw_menu(game)?;
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

fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
