pub mod neovim;
use neovim::Session;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
};
use std::{env, io, thread, time::Duration};

const FPS: usize = 60;
const FRAME_TIME: usize = 1000 / FPS;
const BUFF_SIZE: usize = 0;

fn main() {
    let addr = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No ip specified, listening on 127.0.0.1:6666");
        "127.0.0.1:6666".to_string()
    });
    plug(addr);
}

fn plug(addr: String) {
    let mut nvim = Session::connect(&addr).expect("Couldn't connect");
    start_plug(&mut nvim).expect("Error in plugin during runtime.");
}

fn start_plug(nvim: &mut Session) -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut mode = nvim
        .get_current_mode()
        .expect("Couldn't fetch current neovim mode");
    let mut batch = String::new();

    loop {
        thread::sleep(Duration::from_millis(FRAME_TIME.try_into().unwrap()));
        terminal.draw(|frame| {
            let keys = Paragraph::new(batch.clone()).white().on_dark_gray();
            frame.render_widget(keys, frame.area());
        })?;

        if event::poll(Duration::from_millis(FRAME_TIME.try_into().unwrap())).unwrap() {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char(ch) = key.code {
                        let mut fallthrough = false;
                        match ch {
                            '₹' => break,
                            'i' => {
                                if mode != "i" {
                                    mode = "i".to_string()
                                } else {
                                    fallthrough = true;
                                }
                            }
                            'v' => {
                                if mode != "i" {
                                    mode = "x".to_string()
                                } else {
                                    fallthrough = true;
                                }
                            }
                            't' => {
                                if mode != "i" {
                                    mode = "t".to_string()
                                } else {
                                    fallthrough = true;
                                }
                            }
                            _ => {
                                fallthrough = true;
                            }
                        }
                        if fallthrough {
                            batch += &ch.to_string();
                            if batch.len() > BUFF_SIZE {
                                feed_batch(nvim, &batch, &mode);
                                batch.clear();
                            }
                        }
                    }
                    match key.code {
                        KeyCode::Enter => {
                            feed_batch(nvim, &batch, &mode);
                            feedkey(nvim, &mode, "\n");
                        }
                        KeyCode::Esc => {
                            feed_batch(nvim, &batch, &mode);
                            feedkey(nvim, &mode, "\x1b");
                        }
                        KeyCode::Backspace => {
                            feed_batch(nvim, &batch, &mode);
                            feedkey(nvim, &mode, "\x08");
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn feed_batch(nvim: &mut Session, batch: &str, mode: &str) {
    let _ = nvim.feedkeys(batch, mode);
}

fn feedkey(nvim: &mut Session, mode: &str, key: &str) {
    while nvim.feedkeys(key, mode).is_err() {}
}
