use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////PRELUDE
fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////TERMINAL INIT
    let mut counter = 0;
    loop {
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("Text").block(Block::new().borders(Borders::ALL)), layout[0]);
        })?;

        // Check for user input every 250 milliseconds
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////TERMINAL EXIT
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}