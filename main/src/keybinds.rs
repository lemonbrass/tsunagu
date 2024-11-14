use crate::neovim::Session;
use ratatui::crossterm::event::KeyCode;

type ShouldExit = bool;

pub struct KeyListener {
    nvim: Session,
    mode: String,
}

impl KeyListener {
    pub fn new(addr: String) -> KeyListener {
        let mut nvim = Session::connect(&addr);
        let mode = nvim.get_current_mode().unwrap();
        KeyListener { mode, nvim }
    }

    pub fn listen(&mut self, key: KeyCode) -> ShouldExit {
        self.update_mode();
        match key {
            KeyCode::Enter => {
                self.nvim.feedkeys("\n", &self.mode);
            }
            KeyCode::Esc => {
                self.nvim.feedkeys("\x1b", &self.mode);
            }
            KeyCode::Backspace => {
                self.nvim.feedkeys("\x08", &self.mode);
            }
            KeyCode::Tab => {
                self.nvim.feedkeys("\t", &self.mode);
            }
            KeyCode::Left => {
                self.nvim.feedkeys("h", &self.mode);
            }
            KeyCode::Right => {
                self.nvim.feedkeys("l", &self.mode);
            }
            KeyCode::Up => {
                self.nvim.feedkeys("k", &self.mode);
            }
            KeyCode::Down => {
                self.nvim.feedkeys("j", &self.mode);
            }
            KeyCode::Char(ch) => {
                if ch == '₹' {
                    return true; // exit
                }
                self.send_key(ch);
            }
            _ => return false,
        }
        false
    }

    fn update_mode(&mut self) {
        self.mode = self.nvim.get_current_mode().unwrap();
    }

    pub fn send_key(&mut self, key: char) {
        self.nvim.feedkeys(&key.to_string(), &self.mode);
    }
}
