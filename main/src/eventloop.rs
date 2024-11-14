use crate::keybinds::KeyListener;
use crate::ui::UI;
use ratatui::crossterm::event::{self, KeyEventKind};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;
use std::time::Duration;

type ShouldExit = bool;
const FRAME_TIME: u64 = 1000 / 30;

pub struct EventLoop {
    ui: UI,
    keylistener: KeyListener,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl EventLoop {
    pub fn init(addr: String) -> Self {
        let keylistener = KeyListener::new(addr.clone());
        let mut terminal = ratatui::init();
        terminal.clear().unwrap();
        Self {
            ui: UI::new(addr),
            keylistener,
            terminal,
        }
    }

    pub fn update(&mut self) -> ShouldExit {
        self.terminal.draw(|f| self.ui.render_ui(f)).unwrap();

        if event::poll(Duration::from_millis(FRAME_TIME)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    if self.keylistener.listen(key.code) {
                        return true;
                    }
                    self.ui.update(key.code);
                }
            }
        }

        false
    }

    pub fn stop(&mut self) {
        ratatui::restore();
    }
}
