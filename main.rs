use std::io;
use termion::clear;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    println!("{}", clear::All);
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        match terminal.draw(|f| {
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
            let block2 = Paragraph::new("kleber: hueheuhe").block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(block1, chunks[1]);
            f.render_widget(block2, chunks[0]);
        }) {
            Ok(value) => value,
            Err(err) => println!("error {}", err)
        }
    }
}
