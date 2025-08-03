use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use std::{io, sync::mpsc};
use tui::{backend::CrosstermBackend, Terminal};
use zenocode_core::{get_provider, Config, CoreError};

struct App {
    input: String,
    output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load config (simplified for now)
    let config = Config::load().map_err(|e| CoreError::ConfigError(e.to_string()))?;
    
    // Get provider from registry (default to openai)
    let provider = get_provider(&config.provider, &config)
        .await
        .map_err(|e| CoreError::ProviderError(e.to_string()))?;

    let mut app = App {
        input: String::new(),
        output: String::new(),
    };

    loop {
        terminal.draw(|f| {
            let chunks = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .margin(1)
                .constraints([tui::layout::Constraint::Percentage(30), tui::layout::Constraint::Percentage(70)].as_ref())
                .split(f.size());

            let input_block = tui::widgets::Block::default()
                .title("Input")
                .borders(tui::widgets::Borders::ALL);
            let input_widget = tui::widgets::Paragraph::new(app.input.as_ref())
                .block(input_block);
            f.render_widget(input_widget, chunks[0]);

            let output_block = tui::widgets::Block::default()
                .title("Response")
                .borders(tui::widgets::Borders::ALL);
            let output_widget = tui::widgets::Paragraph::new(app.output.as_ref())
                .block(output_block);
            f.render_widget(output_widget, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Enter => {
                        let mut stream = provider.stream(&app.input).await?;
                        app.output.clear();
                        while let Some(chunk) = stream.recv().await {
                            let chunk = chunk?;
                            app.output.push_str(&chunk);
                            app.output.push(' ');
                            terminal.draw(|f| {
                                let chunks = tui::layout::Layout::default()
                                    .direction(tui::layout::Direction::Vertical)
                                    .margin(1)
                                    .constraints([tui::layout::Constraint::Percentage(30), tui::layout::Constraint::Percentage(70)].as_ref())
                                    .split(f.size());

                                let output_block = tui::widgets::Block::default()
                                    .title("Response")
                                    .borders(tui::widgets::Borders::ALL);
                                let output_widget = tui::widgets::Paragraph::new(app.output.as_ref())
                                    .block(output_block);
                                f.render_widget(output_widget, chunks[1]);
                            })?;
                        }
                        app.input.clear();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}