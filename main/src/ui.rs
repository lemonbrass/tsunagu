use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct UI {
    addr: String,
}

impl UI {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn render_ui(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(frame.area());

        self.render_top_bar(frame, chunks[0]);
    }

    fn render_top_bar(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let top_bar = Paragraph::new(Span::raw(format!("Connected to {}", self.addr)))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(top_bar, area);
    }
}
