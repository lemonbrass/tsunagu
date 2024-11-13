use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub struct UI {
    addr: String,
    keypressed: String,
}

impl UI {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            keypressed: "".into(),
        }
    }

    pub fn update(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(ch) => {
                self.keypressed.push(ch);
            }
            KeyCode::Backspace => {
                let _ = self.keypressed.pop();
            }
            KeyCode::Enter => {
                self.keypressed.push('\n');
            }
            KeyCode::Esc => {
                self.keypressed = "".into();
            }
            _ => {}
        }
    }

    pub fn render_ui(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .flex(ratatui::layout::Flex::Center)
            .split(frame.area());

        self.render_top_bar(frame, chunks[0]);
        self.render_keypress(frame, chunks[1]);
    }

    fn render_top_bar(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let top_bar = Paragraph::new(Span::raw(format!("Connected to {}", self.addr)))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false });

        frame.render_widget(top_bar, area);
    }
    fn render_keypress(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let lines = self
            .keypressed
            .split('\n')
            .map(|line| Line::raw(line))
            .collect::<Vec<_>>();
        let text = Paragraph::new(Text::from(lines))
            .block(Block::bordered().borders(Borders::ALL))
            .wrap(Wrap { trim: false });

        frame.render_widget(text, area);
    }
}
