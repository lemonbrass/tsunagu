use neovim_lib::{Neovim, NeovimApi, Session};
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
    let mut session = Session::new_tcp(&addr).unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    let _ = start_plug(&mut nvim);
}

fn start_plug(nvim: &mut Neovim) -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut mode;
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
                    mode = get_current_mode(nvim);
                    if let KeyCode::Char(ch) = key.code {
                        if ch == '₹' {
                            break;
                        }
                        batch += &ch.to_string();
                        if batch.len() > BUFF_SIZE {
                            feed_batch(nvim, &batch, &mode);
                            batch.clear();
                        }
                    }
                    match key.code {
                        KeyCode::Enter => {
                            feed_batch(nvim, &batch, &mode);
                            feed_enter_key(nvim, &mode);
                        }
                        KeyCode::Esc => {
                            feed_batch(nvim, &batch, &mode);
                            feed_escape_key(nvim, &mode);
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

fn feed_batch(nvim: &mut Neovim, batch: &str, mode: &str) {
    let _ = nvim.feedkeys(batch, mode, false);
}

fn feed_enter_key(nvim: &mut Neovim, mode: &str) {
    while nvim.feedkeys("\n", mode, true).is_err() {}
}

fn feed_escape_key(nvim: &mut Neovim, mode: &str) {
    while nvim.feedkeys("\x1b", mode, true).is_err() {}
}

fn get_current_mode(nvim: &mut Neovim) -> String {
    if let Ok(mode_info) = nvim.get_mode() {
        mode_info
            .iter()
            .find(|(key, _)| key.as_str().unwrap() == "mode")
            .map(|(_, value)| value.as_str().unwrap().to_string())
            .unwrap_or_default()
    } else {
        get_current_mode(nvim)
    }
}
