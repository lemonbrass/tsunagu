pub mod keybinds;
pub mod neovim;
pub mod timer;
use keybinds::KeyListener;
use neovim::Session;

use ratatui::crossterm::event::{self, KeyEventKind};
use std::{env, io, thread, time::Duration};

const FPS: usize = 60;
const FRAME_TIME: usize = 1000 / FPS;

fn main() {
    let addr = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No ip specified, listening on 127.0.0.1:6666");
        "127.0.0.1:6666".to_string()
    });
    start_plug(addr).expect("Error in plugin during runtime.");
}

fn start_plug(addr: String) -> io::Result<()> {
    let nvim = Session::connect(&addr);
    let mut terminal = ratatui::init();
    let mut keylistener = KeyListener::new(nvim);
    terminal.clear()?;
    loop {
        thread::sleep(Duration::from_millis(FRAME_TIME.try_into().unwrap()));
        terminal.draw(|_frame| {})?;

        if event::poll(Duration::from_millis(FRAME_TIME.try_into().unwrap())).unwrap() {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if keylistener.listen(key.code) {
                        break;
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
