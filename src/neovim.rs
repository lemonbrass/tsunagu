use rmpv::{encode, Value};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Keybind {
    pub lhs: Vec<char>,
    pub rhs: String,
    pub mode: String,
    pub noremap: bool,
}

#[derive(Debug)]
pub struct Session {
    stream: TcpStream,
    message_id: u32,
}

impl Session {
    pub fn connect(address: &str) -> Self {
        let stream = TcpStream::connect(address).expect("Failed to connect");
        Self {
            stream,
            message_id: 0,
        }
    }

    pub fn get_all_keybinds(&mut self, mode: &str) -> Vec<Keybind> {
        loop {
            let request = self.build_request("nvim_get_keymap", vec![Value::from(mode)]);
            if let Some(response) = self.send_request(request) {
                if let Some(keybinds) = self.parse_keybinds(response, mode) {
                    return keybinds;
                }
            }
        }
    }

    pub fn get_current_mode(&mut self) -> String {
        loop {
            let request = self.build_request("nvim_get_mode", vec![]);
            if let Some(response) = self.send_request(request) {
                if let Some(mode) = self.parse_mode(response) {
                    return mode;
                }
            }
        }
    }

    pub fn feedkeys(&mut self, keys: &str, mode: &str) {
        let request = self.build_request(
            "nvim_feedkeys",
            vec![Value::from(keys), Value::from(mode), Value::from(false)],
        );
        self.send_request(request);
    }

    fn next_message_id(&mut self) -> u32 {
        let id = self.message_id;
        self.message_id += 1;
        id
    }

    fn build_request(&mut self, method: &str, params: Vec<Value>) -> Value {
        Value::Array(vec![
            Value::from(0),
            Value::from(self.next_message_id()),
            Value::from(method),
            Value::Array(params),
        ])
    }

    fn send_request(&mut self, request: Value) -> Option<Value> {
        loop {
            let mut buf = Vec::new();
            if encode::write_value(&mut buf, &request).is_ok()
                && self.stream.write_all(&buf).is_ok()
            {
                let mut response_buf = [0u8; 65535];
                if let Ok(n) = self.stream.read(&mut response_buf) {
                    if let Ok(response) = rmpv::decode::read_value(&mut &response_buf[..n]) {
                        return Some(response);
                    }
                }
            }
        }
    }

    fn parse_keybinds(&self, response: Value, default_mode: &str) -> Option<Vec<Keybind>> {
        if let Value::Array(items) = response {
            if items.len() == 4 && items[2].is_nil() {
                if let Value::Array(keymaps) = &items[3] {
                    return Some(
                        keymaps
                            .iter()
                            .filter_map(|keymap| self.parse_keybind(keymap, default_mode))
                            .collect(),
                    );
                }
            }
        }
        None
    }

    fn parse_keybind_lhs(&self, lhs: String) -> Vec<char> {
        let mut res = Vec::new();
        for ch in lhs.chars().into_iter() {
            match ch {
                '-' | '<' | '>' => {}
                _ => res.push(ch),
            }
        }
        res
    }

    fn parse_keybind(&self, keymap: &Value, default_mode: &str) -> Option<Keybind> {
        if let Value::Map(map) = keymap {
            let lhs = self.get_str_from_map(map, "lhs")?;
            let lhs = self.parse_keybind_lhs(lhs.to_string());
            let rhs = self.get_str_from_map(map, "rhs")?;
            let noremap = self.get_bool_from_map(map, "noremap").unwrap_or(false);
            let mode = self
                .get_str_from_map(map, "mode")
                .unwrap_or(default_mode)
                .to_string();

            Some(Keybind {
                lhs,
                rhs: rhs.to_string(),
                noremap,
                mode,
            })
        } else {
            None
        }
    }

    fn parse_mode(&self, response: Value) -> Option<String> {
        if let Value::Array(items) = response {
            if items.len() == 4 && items[2].is_nil() {
                if let Value::Map(map) = &items[3] {
                    return self.get_str_from_map(map, "mode").map(String::from);
                }
            }
        }
        None
    }

    fn get_str_from_map<'a>(&self, map: &'a [(Value, Value)], key: &str) -> Option<&'a str> {
        map.iter()
            .find_map(|(k, v)| if k.as_str()? == key { v.as_str() } else { None })
    }

    fn get_bool_from_map(&self, map: &[(Value, Value)], key: &str) -> Option<bool> {
        map.iter().find_map(|(k, v)| {
            if k.as_str()? == key {
                v.as_bool()
            } else {
                None
            }
        })
    }
}
