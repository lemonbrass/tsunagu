pub mod eventloop;
pub mod keybinds;
pub mod neovim;
pub mod ui;
use eventloop::EventLoop;
use std::env;

fn main() {
    let addr = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No ip specified, listening on 127.0.0.1:6666");
        "127.0.0.1:6666".to_string()
    });

    let mut app = EventLoop::init(addr);

    loop {
        if app.update() {
            break;
        }
    }

    app.stop();
}
