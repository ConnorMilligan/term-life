use std::io::{self};
use crossterm::{cursor, style::{self, Colors, Print, SetColors, SetForegroundColor}, QueueableCommand};
use crate::game::Game;

const MENU_OPTIONS: [(&str, &str); 3] = [
    ("q", "Quit the game"),
    ("<", "Decrease TPS"),
    (">", "Increase TPS")
];

pub fn clear_canvas(game: &mut Game) -> io::Result<()> {
    let (width, height) = crossterm::terminal::size()?;

    for h in 1..height-1 {
        for w in 1..width-width/6-1 {
            if game.test.0 != w && game.test.1 != h {
                game.stdout.queue(cursor::MoveTo(w, h))?
                    .queue(Print(" "))?;
            }
        }
    }
    Ok(())
}

pub fn draw_menu(game: &mut Game) -> io::Result<()> {
    let (width, height) = crossterm::terminal::size()?;
    
    // Draw the border
    game.stdout.queue(SetForegroundColor(style::Color::DarkGrey))?;
    for h in 0..height {
        game.stdout.queue(cursor::MoveTo(0, h))?
            .queue(Print("█"))?
            .queue(cursor::MoveTo(width, h))?
            .queue(Print("█"))?; 
    }
    for w in 0..width {
        if w < 2 || w >= 16 {
            game.stdout.queue(cursor::MoveTo(w, 0))?
                .queue(Print("█"))?;
        }
        game.stdout.queue(cursor::MoveTo(w, height))?
            .queue(Print("█"))?;
    }
    
    // Draw the menu
    for h in 1..height-1 {
        game.stdout.queue(cursor::MoveTo(0, h))?
            .queue(cursor::MoveTo(width-width/6, h))?
            .queue(Print("█"))?; 
    }

    // Draw the title
    game.stdout.queue(cursor::MoveTo(2, 0))?
        .queue(SetColors(Colors::new(style::Color::Black, style::Color::Grey)))?
        .queue(Print(" Game of Life "))?;

    // Draw the options
    for (i, (key, description)) in MENU_OPTIONS.iter().enumerate() {
        game.stdout.queue(cursor::MoveTo(width-width/6+1, i as u16 + 1))?
            .queue(SetColors(Colors::new(style::Color::Green, style::Color::Black)))?
            .queue(Print(key))?
            .queue(cursor::MoveTo(width-width/6+4, i as u16 + 1))?
            .queue(SetColors(Colors::new(style::Color::White, style::Color::Black)))?
            .queue(Print(format!(": {}", description)))?;
            
    }

    game.stdout.queue(cursor::MoveTo(width-10, 0))?
        .queue(SetColors(Colors::new(style::Color::Yellow, style::Color::DarkGrey)))?
        .queue(Print(format!(" TPS: {} ", game.tps)))?;

    game.stdout.queue(SetColors(Colors::new(style::Color::White, style::Color::Black)))?;
    Ok(())
}
