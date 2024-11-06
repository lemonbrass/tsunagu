use ratatui::crossterm::event::KeyCode;
//use rmpv::Value;

use crate::neovim::Session;

pub struct KeyListener {
    buffer: String,
    nvim: Session,
    mode: String,
    //keybinds: Value,
}

impl KeyListener {
    pub fn new(mut nvim: Session) -> KeyListener {
        let mode = nvim.get_current_mode().expect("Couldnt get currrent mode");
        KeyListener {
            buffer: "".to_string(),
            /*keybinds: nvim
            .get_all_keybinds(&mode)
            .expect("Couldnt get all keybinds"),
            */
            mode,
            nvim,
        }
    }

    // here, return type bool tells whether to exif or not.
    pub fn listen(&mut self, key: KeyCode) -> bool {
        if let KeyCode::Char(ch) = key {
            self.update_mode(ch);
        }
        match key {
            KeyCode::Enter => {
                self.nvim.feedkeys_try("\n", &self.mode);
            }
            KeyCode::Esc => {
                self.nvim.feedkeys_try("\x1b", &self.mode);
            }
            KeyCode::Backspace => {
                self.nvim.feedkeys_try("\x08", &self.mode);
            }
            KeyCode::Tab => {
                self.nvim.feedkeys_try("\t", &self.mode);
            }
            KeyCode::Char(ch) => {
                if ch == '₹' {
                    return true; // exit
                }
                self.send_key(ch);
            }
            _ => {}
        }
        return false;
    }

    fn update_mode(&mut self, key: char) {
        self.mode = self.nvim.get_mode_try();
        if self.mode == "i" || self.mode == "x" {
            return;
        }
        match key {
            'i' | 't' => self.mode = key.to_string(),
            'v' => self.mode = "x".to_string(),
            _ => return,
        }
    }

    pub fn send_key(&mut self, key: char) {
        self.nvim.feedkeys_try(&key.to_string(), &self.mode);
    }
}
