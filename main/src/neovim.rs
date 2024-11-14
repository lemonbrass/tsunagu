use rmpv::{encode, Value};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use utilcro::{retry, retry_optional};

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
        stream.set_nodelay(true).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        Self {
            stream,
            message_id: 0,
        }
    }

    #[retry(25)]
    pub fn get_current_mode(&mut self) -> Result<String, String> {
        let request = self.build_request("nvim_get_mode", vec![]);
        if let Some(response) = self.send_request(request) {
            if let Some(mode) = self.parse_mode(response) {
                return Ok(mode);
            } else {
                return Err("Couldnt parse mode".to_string());
            }
        } else {
            return Err("Couldnt fetch mode".to_string());
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

    #[retry_optional(25)]
    fn send_request(&mut self, request: Value) -> Option<Value> {
        let mut buf = Vec::new();
        if encode::write_value(&mut buf, &request).is_err() || self.stream.write_all(&buf).is_err()
        {
            return None;
        }

        let mut response_buf = [0u8; 4096];

        match self.stream.read(&mut response_buf) {
            Ok(_) if !response_buf.is_empty() => {
                if let Ok(response) = rmpv::decode::read_value(&mut &response_buf[..]) {
                    return Some(response);
                }
                None
            }
            Ok(_) => {
                eprintln!("Connection closed with no data");
                None
            }
            Err(e) => {
                eprintln!("Read error: {:?}", e);
                None
            }
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
}
