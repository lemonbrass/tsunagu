use rmpv::encode;
use rmpv::Value;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub struct Session {
    stream: TcpStream,
    message_id: u32,
}

impl Session {
    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Self {
            stream,
            message_id: 0,
        })
    }

    pub fn feedkeys(&mut self, keys: &str, mode: &str) -> io::Result<()> {
        let request = Value::Array(vec![
            Value::from(0),
            Value::from(self.next_message_id()),
            Value::from("nvim_feedkeys"),
            Value::Array(vec![
                Value::from(keys),
                Value::from(mode),
                Value::from(false),
            ]),
        ]);

        let mut buf = Vec::new();
        encode::write_value(&mut buf, &request)?;
        self.stream.write_all(&buf)?;
        Ok(())
    }

    pub fn get_current_mode(&mut self) -> io::Result<String> {
        let request = Value::Array(vec![
            Value::from(0),
            Value::from(self.next_message_id()),
            Value::from("nvim_get_mode"),
            Value::Array(vec![]),
        ]);

        let mut buf = Vec::new();
        encode::write_value(&mut buf, &request)?;
        self.stream.write_all(&buf)?;

        let mut response_buf = [0u8; 4096];
        let mut n = self.stream.read(&mut response_buf);
        while n.is_err() {
            n = self.stream.read(&mut response_buf);
        }
        let response = rmpv::decode::read_value(&mut &response_buf[..n.unwrap()]).unwrap();

        println!("{:?}", response);

        if let Value::Array(items) = response {
            if items.len() == 4 {
                let error = &items[2];
                let result = &items[3];

                if !error.is_nil() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Error from Neovim: {:?}", error),
                    ));
                }

                if let Value::Map(map) = result {
                    if let Some((_, Value::String(mode))) =
                        map.iter().find(|(k, _)| k.as_str() == Some("mode"))
                    {
                        return Ok(mode.to_string());
                    }
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid response format",
        ))
    }

    fn next_message_id(&mut self) -> u32 {
        let id = self.message_id;
        self.message_id += 1;
        id
    }
}
