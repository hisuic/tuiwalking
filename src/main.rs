mod frames;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Terminal,
};

const TICK_RATE: Duration = Duration::from_millis(200);

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    Terminal::new(backend)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut frame_index: usize = 0;
    let mut last_tick = Instant::now();
    let mut x_pos: u16 = 0;

    loop {
        terminal.draw(|f| {
            let area = f.area();

            let chunks = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area);

            let art = frames::FRAMES[frame_index];
            let padding = " ".repeat(x_pos as usize);
            let shifted: String = art
                .lines()
                .map(|line| format!("{}{}", padding, line))
                .collect::<Vec<_>>()
                .join("\n");

            let walker = Paragraph::new(shifted)
                .style(Style::default().fg(Color::Green));
            f.render_widget(walker, chunks[0]);

            let ground = Paragraph::new("─".repeat(area.width as usize))
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(ground, chunks[1]);

            let help = Paragraph::new("Press 'q' to quit")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(help, chunks[2]);
        })?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            frame_index = (frame_index + 1) % frames::FRAMES.len();
            let max_x = terminal.size()?.width.saturating_sub(10);
            x_pos = (x_pos + 1) % max_x.max(1);
            last_tick = Instant::now();
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let result = run(&mut terminal);
    restore_terminal()?;
    result
}
