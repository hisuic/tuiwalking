mod frames;
mod scenery;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::Widget,
    Terminal,
};

const TICK_RATE: Duration = Duration::from_millis(150);
const BG_COLOR: Color = Color::Rgb(15, 20, 60);
const BUILDING_COLOR: Color = Color::Rgb(50, 55, 80);
const WINDOW_COLOR: Color = Color::Rgb(220, 200, 100);
const GROUND_COLOR: Color = Color::Rgb(210, 195, 150);
const GROUND_DARK: Color = Color::Rgb(180, 165, 120);
const WALKER_COLOR: Color = Color::Rgb(240, 240, 240);
const HELP_COLOR: Color = Color::Rgb(100, 100, 130);

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

/// Custom widget that composes the full scene.
struct SceneWidget {
    frame_index: usize,
    scroll_offset: usize,
}

impl Widget for SceneWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let h = area.height as usize;
        if h < 4 || w < 20 {
            return;
        }

        // Layout: sky+buildings | walker row area | ground (2 rows) | help (1 row)
        let ground_rows = 2usize;
        let help_rows = 1usize;
        let scene_height = h.saturating_sub(ground_rows + help_rows);
        let walker_lines: Vec<&str> = frames::FRAMES[self.frame_index].lines().collect();
        let walker_h = walker_lines.len();

        // Buildings fill the sky area
        let building_area_height = scene_height.saturating_sub(walker_h);
        let skyline = scenery::render_skyline(
            w as u16,
            building_area_height.max(1) as u16,
            self.scroll_offset,
        );

        // Fill entire background with dark blue
        for y in 0..h {
            for x in 0..w {
                let cell = &mut buf[(area.x + x as u16, area.y + y as u16)];
                cell.set_char(' ');
                cell.set_bg(BG_COLOR);
                cell.set_fg(Color::White);
            }
        }

        // Draw buildings
        for (row_idx, row_str) in skyline.iter().enumerate() {
            let y = area.y + row_idx as u16;
            for (col, ch) in row_str.chars().enumerate() {
                if col >= w {
                    break;
                }
                let x = area.x + col as u16;
                if ch != ' ' {
                    let cell = &mut buf[(x, y)];
                    let (fg, bg) = match ch {
                        '▪' | '░' => (WINDOW_COLOR, BUILDING_COLOR),
                        _ => (BUILDING_COLOR, BUILDING_COLOR),
                    };
                    cell.set_char(ch);
                    cell.set_fg(fg);
                    cell.set_bg(bg);
                }
            }
        }

        // Draw walker centered horizontally, positioned just above ground
        let walker_x_start = (w / 2).saturating_sub(frames::FRAME_WIDTH as usize / 2);
        let walker_y_start = scene_height.saturating_sub(walker_h);
        for (i, line) in walker_lines.iter().enumerate() {
            let y = area.y + (walker_y_start + i) as u16;
            if y >= area.y + area.height {
                break;
            }
            for (j, ch) in line.chars().enumerate() {
                let x = area.x + (walker_x_start + j) as u16;
                if x >= area.x + area.width {
                    break;
                }
                if ch != ' ' {
                    let cell = &mut buf[(x, y)];
                    cell.set_char(ch);
                    cell.set_fg(WALKER_COLOR);
                    // Keep whatever bg was there (building or sky)
                }
            }
        }

        // Draw ground
        for row in 0..ground_rows {
            let y = area.y + scene_height as u16 + row as u16;
            if y >= area.y + area.height {
                break;
            }
            for col in 0..w {
                let x = area.x + col as u16;
                let cell = &mut buf[(x, y)];
                if row == 0 {
                    // Top ground row - textured
                    let shifted = (col + self.scroll_offset) % 4;
                    if shifted == 0 {
                        cell.set_char('░');
                        cell.set_fg(GROUND_DARK);
                        cell.set_bg(GROUND_COLOR);
                    } else {
                        cell.set_char(' ');
                        cell.set_bg(GROUND_COLOR);
                    }
                } else {
                    cell.set_char(' ');
                    cell.set_bg(GROUND_DARK);
                }
            }
        }

        // Help text at bottom
        let help = "Press 'q' to quit";
        let help_y = area.y + area.height - 1;
        let help_x = (w.saturating_sub(help.len())) / 2;
        for (i, ch) in help.chars().enumerate() {
            let x = area.x + (help_x + i) as u16;
            if x < area.x + area.width {
                let cell = &mut buf[(x, help_y)];
                cell.set_char(ch);
                cell.set_fg(HELP_COLOR);
                cell.set_bg(BG_COLOR);
            }
        }
    }
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut frame_index: usize = 0;
    let mut last_tick = Instant::now();
    let mut scroll_offset: usize = 0;

    loop {
        let fi = frame_index;
        let so = scroll_offset;

        terminal.draw(|f| {
            let area = f.area();
            f.render_widget(SceneWidget { frame_index: fi, scroll_offset: so }, area);
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
            scroll_offset = scroll_offset.wrapping_add(1);
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
