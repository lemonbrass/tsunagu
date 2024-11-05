use std::net::TcpStream;
use std::io::{self, Write, Read};
use rmp::encode;
use rmpv::Value;

pub struct NeovimSession {
    stream: TcpStream,
    message_id: u32,
}

impl NeovimSession {
    // Creates a new Neovim session by connecting to a specified address
    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Self {
            stream,
            message_id: 0,
        })
    }

    // Sends keystrokes to Neovim using the `nvim_feedkeys` command
    pub fn feedkeys(&mut self, keys: &str, mode: &str) -> io::Result<()> {
        let request = Value::Array(vec![
            Value::from(0),                     // Type (0 for Request)
            Value::from(self.next_message_id()),// Unique message ID
            Value::from("nvim_feedkeys"),       // Method name
            Value::Array(vec![
                Value::from(keys),              // Keys to feed
                Value::from(mode),              // Mode
                Value::from(false),             // Escape CSI
            ]),
        ]);

        let mut buf = Vec::new();
        encode::write_value(&mut buf, &request)?;
        self.stream.write_all(&buf)?;
        Ok(())
    }

    // Fetches the current mode in Neovim using the `nvim_get_mode` command
    pub fn get_current_mode(&mut self) -> io::Result<String> {
        let request = Value::Array(vec![
            Value::from(0),                     // Type (0 for Request)
            Value::from(self.next_message_id()),// Unique message ID
            Value::from("nvim_get_mode"),       // Method name
            Value::Array(vec![]),               // Empty params for this command
        ]);

        let mut buf = Vec::new();
        encode::write_value(&mut buf, &request)?;
        self.stream.write_all(&buf)?;

        // Read and parse response from Neovim
        let mut response_buf = [0u8; 4096];
        let n = self.stream.read(&mut response_buf)?;
        let response = rmpv::decode::read_value(&mut &response_buf[..n])?;

        // Extract mode information from the response
        if let Value::Array(mut items) = response {
            if items.len() > 2 {
                if let Value::Map(map) = &items[2] {
                    if let Some((_, Value::String(mode))) = map.iter().find(|(k, _)| k.as_str() == Some("mode")) {
                        return Ok(mode.to_string());
                    }
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response format"))
    }

    // Generates the next unique message ID for each request
    fn next_message_id(&mut self) -> u32 {
        let id = self.message_id;
        self.message_id += 1;
        id
    }
}

fn main() -> io::Result<()> {
    // Replace with Neovim's address (e.g., "127.0.0.1:6666" for TCP or a UNIX socket path)
    let mut session = NeovimSession::connect("127.0.0.1:6666")?;

    // Example usage of `feedkeys`
    session.feedkeys("iHello, Neovim!", "n")?;

    // Example usage of `get_current_mode`
    match session.get_current_mode() {
        Ok(mode) => println!("Current mode: {}", mode),
        Err(e) => eprintln!("Failed to fetch current mode: {}", e),
    }

    Ok(())
}
