use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};

pub fn start_tui() {
    // Initialize the TUI application
    // This is where you would set up your terminal UI, handle input, and display output.
    // For example, you might use a library like `crossterm` or `tui-rs` to create a user interface.

    println!("Welcome to the Zenocode TUI!");

    // Here you would typically enter your main event loop,
    // handling user input and updating the display accordingly.

    // Example placeholder for main loop
    loop {
        // Handle user input and update the UI
        // This is where you would integrate with the provider registry and other components.

        // For now, we just break out of the loop to end the program.
        let _ = main();
        break;
    }

    println!("Exiting Zenocode TUI.");
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(f: &mut Frame) {
    let size = f.area();

    // Split main screen into: top bar, main area, and bottom bar
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Top nav bar
            Constraint::Min(10),   // Main prompt + response area
            Constraint::Length(1), // Bottom status bar
        ])
        .split(size);

    // ───── 1. Top Nav Bar ─────
    let nav_block = Paragraph::new("ZENOCODE   ▸ New ▸ Help ▸ Models ▸ Share ▸ Editor ▸ Redo")
        .block(Block::default().borders(Borders::BOTTOM).title("Zenocode"))
        .alignment(Alignment::Center);
    f.render_widget(nav_block, layout[0]);

    // ───── 2. Main Area ─────
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Prompt input
            Constraint::Min(7),    // AI output
        ])
        .split(layout[1]);

    // Prompt input
    let prompt = Paragraph::new("> How do I write a Rust trait for async behavior?")
        .block(Block::default().borders(Borders::ALL).title("Prompt"));
    f.render_widget(prompt, main_chunks[0]);

    // AI response
    let ai_response = Paragraph::new(
        "You can define a trait with async functions using `async-trait`.\n\n\
         #[async_trait]\n\
         trait MyAsyncTrait {\n\
             async fn do_thing(&self);\n\
         }\n\n\
         Let me know if you'd like help implementing this!",
    )
    .block(Block::default().borders(Borders::ALL).title("AI Response"));
    f.render_widget(ai_response, main_chunks[1]);

    // ───── 3. Bottom Status Bar ─────
    let status = Paragraph::new("Session: rust-project-zen | LSP: rust-analyzer | Model: GPT-4")
        .alignment(Alignment::Center);
    f.render_widget(status, layout[2]);
}
