use crate::neovim::Session;
use ratatui::crossterm::event::KeyCode;

//const DURATION: u64 = 500;

pub struct KeyListener {
    //    timer: Option<Timer>,
    //    buffer: String,
    //    possible_keys: Vec<Vec<char>>,
    nvim: Session,
    mode: String,
    //    keybinds: Vec<Keybind>,
}

impl KeyListener {
    pub fn new(addr: String) -> KeyListener {
        let mut nvim = Session::connect(&addr);
        let mode = nvim.get_current_mode().unwrap();
        KeyListener {
            //            timer: None,
            //            buffer: "".to_string(),
            //            possible_keys: Vec::new(),
            //            keybinds: nvim.get_all_keybinds(""),
            mode,
            nvim,
        }
    }

    // here, return type bool tells whether to exit or not.
    pub fn listen(&mut self, key: KeyCode) -> bool {
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
            // KeyCode::Modifier(modkeycode) => {
            //     if let ModifierKeyCode::LeftControl = modkeycode {
            //         self.handle_autocomp('C');
            //     }
            // }
            _ => return false,
        }
        false
    }

    fn update_mode(&mut self) {
        self.mode = self.nvim.get_current_mode().unwrap();
    }

    pub fn send_key(&mut self, key: char) {
        self.nvim.feedkeys(&key.to_string(), &self.mode);
        // if self.mode == "i" {
        //     self.nvim.feedkeys(&key.to_string(), &self.mode);
        // } else {
        //     self.handle_autocomp(key);
        // }
    }

    // fn handle_autocomp(&mut self, key: char) {
    //     if self.possible_keys.is_empty() {
    //         for (i, keymap) in self.keybinds.iter().enumerate() {
    //             if self.mode == keymap.mode && keymap.lhs[0] == key {
    //                 self.possible_keys.push(keymap.lhs.clone());
    //                 self.buffer += &key.to_string();
    //                 self.timer = Some(Timer::new(DURATION));
    //                 return;
    //             }
    //         }
    //         self.nvim.feedkeys(&self.buffer, &self.mode);
    //         self.nvim.feedkeys(&key.to_string(), &self.mode);
    //         return;
    //     } else {
    //         for keymap in &self.possible_keys {
    //             if keymap.iter().collect::<String>() == self.buffer {
    //                 self.nvim.feedkeys(&self.buffer, &self.mode);
    //                 self.possible_keys.clear();
    //                 self.timer = None;
    //                 return;
    //             } else if keymap.len() >= self.buffer.len() && keymap[self.buffer.len()] == key {
    //                 self.buffer += &key.to_string();
    //                 self.timer = Some(Timer::new(DURATION))
    //             }
    //         }
    //    }
    //}
}
