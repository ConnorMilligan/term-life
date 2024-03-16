use std::{io::{self, Write}, time::Duration};
use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind},
    QueueableCommand,
};

use crate::menu::{clear_canvas, draw_menu};

pub struct Game {
    pub stdout: std::io::Stdout,
    pub running: bool,
    pub tps: u16,
    pub tick: u16,
    pub test: (u16, u16),
}

pub fn game_loop() -> io::Result<()> {
    let mut game = Game {
        stdout: std::io::stdout(),
        running: true,
        tps: 10,
        tick: 0,
        test: (1, 5),
    };

    while game.running {
        take_input(&mut game)?;
        tick(&mut game)?;
        draw(&mut game)?;
    }

    Ok(())
}

fn draw(game: &mut Game) -> io::Result<()> {
    clear_canvas(game)?;
    draw_menu(game)?;
    game.stdout.queue(crossterm::cursor::MoveTo(game.test.0, game.test.1))?
        .queue(crossterm::style::Print("X"))?
        .flush()?;
    Ok(())
}

fn tick(game: &mut Game) -> io::Result<()> {
    if game.tick < game.tps {
        game.tick += 1;
    }
    else {
        game.tick = 0;
        if game.test.0 < 20 {
            game.test.0 += 1;
        } else {
            game.test.0 = 1;
        }
    }
    Ok(())
}

fn take_input(game: &mut Game) -> io::Result<()> {
    match read_char()? {
        'q' => {
            game.running = false;
        }
        '>' => {
            if game.tps < 10 {
                game.tps += 1;
            }
        }
        '<' => {
            if game.tps > 1 {
                game.tps -= 1;
            }
        }
        _ => {}
    }
    Ok(())
}

fn read_char() -> std::io::Result<char> {
    loop {
        if poll(Duration::from_millis(100))? {
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
        else {
            return Ok('~');
        }
    }
}
