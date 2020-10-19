use termion::clear;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, ListItem, List};
use tui::text::{Span, Spans, Text};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use std::io::{self, Write};

struct App {
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            messages: Vec::new(),
        }
    }
}


fn draw_terminal(app: &App) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
            
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block1 = Block::default().borders(Borders::ALL);
        let messages: Vec<ListItem> = app.messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("you: {}", m)))];
                    ListItem::new(content)
                })
                .collect();
        
        let block2 = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(block1, chunks[1]);
        f.render_widget(block2, chunks[0]);
        f.set_cursor(chunks[1].x + 1, chunks[1].y + 2);
        
    })
}


fn main() -> Result<(), io::Error> {
    let mut app = App::default();
    let app_ref = &mut app;
    println!("{}", clear::All);
    let start = vec![
            "Press ".to_string(),
        ];
    draw_terminal(app_ref);

    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read Line");
        app_ref.messages.push(message);
        println!("{}", clear::All);
        draw_terminal(app_ref);
        
    }
}
