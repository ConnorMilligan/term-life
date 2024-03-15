mod game;
mod terminal;
mod menu;

use std::io;

use crate::game::*;
use crate::terminal::*;

fn main() -> io::Result<()> {
    setup_terminal()?;

    game_loop()?;

    cleanup_terminal()
}
