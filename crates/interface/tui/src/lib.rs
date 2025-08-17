use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute, terminal,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint as C, Direction as D, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};

#[derive(Debug, Clone, Copy)]
enum Tab {
    Chat,
    Files,
    Tasks,
    Settings,
}
impl Tab {
    const ALL: [Tab; 4] = [Tab::Chat, Tab::Files, Tab::Tasks, Tab::Settings];
    fn title(&self) -> &'static str {
        match self {
            Tab::Chat => "Chat",
            Tab::Files => "Files",
            Tab::Tasks => "Tasks",
            Tab::Settings => "Settings",
        }
    }
}

struct App {
    active_tab: usize,
    editor_text: String,
    console_lines: Vec<String>,
    started_at: Instant,
    model_name: String,
    branch: String,
    latency_ms: u64,
    show_help: bool,
}

impl App {
    fn new() -> Self {
        Self {
            active_tab: 0,
            editor_text: String::from("// Start typing…"),
            console_lines: vec!["boot: ok".into(), "lsp: connected".into()],
            started_at: Instant::now(),
            model_name: "gpt‑4.1‑mini".into(),
            branch: "feat/lsp-wire".into(),
            latency_ms: 42,
            show_help: true,
        }
    }
    fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % Tab::ALL.len();
    }
    fn prev_tab(&mut self) {
        self.active_tab = (self.active_tab + Tab::ALL.len() - 1) % Tab::ALL.len();
    }
}

pub async fn main() -> Result<()> {
    // terminal init
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let res = run(&mut terminal).await;

    // restore
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;

    res
}

async fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // small poll interval keeps UI responsive without busy loop
        if event::poll(Duration::from_millis(33))? {
            match event::read()? {
                Event::Key(key) => {
                    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                    match (key.code, ctrl) {
                        (KeyCode::Char('c'), true) | (KeyCode::Esc, false) => break, // quit
                        (KeyCode::Tab, false) => app.next_tab(),
                        (KeyCode::BackTab, false) => app.prev_tab(),
                        (KeyCode::Char('h'), true) => app.show_help = !app.show_help,
                        (KeyCode::Char('j'), true) => app.console_lines.push("job queued".into()),
                        (KeyCode::Char('l'), true) => app.console_lines.clear(),
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {}
                Event::Mouse(_) => {}
                Event::FocusGained | Event::FocusLost | Event::Paste(_) => {}
            }
        }

        // Example: update latency ticker
        app.latency_ms = 30 + ((app.started_at.elapsed().as_millis() / 250) % 40) as u64;
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area().inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    // Top (header) / Middle (content) / Bottom (console)
    let rows = Layout::default()
        .direction(D::Vertical)
        .constraints([C::Length(3), C::Min(5), C::Length(7)])
        .split(size);

    draw_header(f, rows[0], app);
    draw_body(f, rows[1], app);
    draw_console(f, rows[2], app);

    if app.show_help {
        draw_help(f);
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let left = format!(" Zenocode ▸ {} ", Tab::ALL[app.active_tab].title());
    let right = format!(
        " model:{}  branch:{}  latency:{}ms ",
        app.model_name, app.branch, app.latency_ms
    );

    let block = Block::default().borders(Borders::ALL).title(" Status ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split header horizontally: left status + right status
    let cols = Layout::default()
        .direction(D::Horizontal)
        .constraints([C::Percentage(60), C::Percentage(40)])
        .split(inner);

    let left_par = Paragraph::new(Line::from(Span::raw(left)));
    let right_par =
        Paragraph::new(Line::from(Span::raw(right))).alignment(ratatui::layout::Alignment::Right);

    f.render_widget(left_par, cols[0]);
    f.render_widget(right_par, cols[1]);
}

fn draw_body(f: &mut Frame, area: Rect, app: &App) {
    // Left nav (tabs) | Main pane
    let cols = Layout::default()
        .direction(D::Horizontal)
        .constraints([C::Length(18), C::Min(30)])
        .split(area);

    draw_nav(f, cols[0], app);
    draw_main(f, cols[1], app);
}

fn draw_nav(f: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = Tab::ALL.iter().map(|t| Line::from(t.title())).collect();
    let tabs = Tabs::new(titles)
        .select(app.active_tab)
        .block(Block::default().borders(Borders::ALL).title(" Menu "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED));
    f.render_widget(tabs, area);

    // Bonus: a quick list under tabs (e.g., recent files)
    let below = Layout::default()
        .direction(D::Vertical)
        .constraints([C::Length(3), C::Min(1)])
        .split(area);

    let items = vec![
        ListItem::new("README.md"),
        ListItem::new("src/main.rs"),
        ListItem::new("crates/engine/src/lib.rs"),
    ];
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .title(" Recent "),
    );
    f.render_widget(list, below[1]);
}

fn draw_main(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(match Tab::ALL[app.active_tab] {
            Tab::Chat => " Chat ",
            Tab::Files => " Editor ",
            Tab::Tasks => " Tasks ",
            Tab::Settings => " Settings ",
        });
    let inner = block.inner(area);
    f.render_widget(block, area);

    // In v0.1, a simple Paragraph; later, swap for a code viewer
    let text = match Tab::ALL[app.active_tab] {
        Tab::Chat => "Assistant: How can I help?\n> You: ",
        Tab::Files => &app.editor_text,
        Tab::Tasks => "- [ ] Index repo\n- [ ] Generate types\n- [ ] Run tests",
        Tab::Settings => {
            "• Model: gpt‑4.1‑mini\n• Temperature: 0.2\n• API endpoint: http://localhost:11434"
        }
    };
    let p = Paragraph::new(text).wrap(Wrap { trim: false });
    f.render_widget(p, inner);
}

fn draw_console(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().borders(Borders::ALL).title(" Console ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines: Vec<Line> = app
        .console_lines
        .iter()
        .rev() // newest at bottom visually
        .take((inner.height as usize).saturating_sub(2))
        .cloned()
        .map(Line::from)
        .collect();

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(para, inner);
}

fn draw_help(f: &mut Frame) {
    let area = centered_rect(60, 40, f.area());
    let block = Block::default().borders(Borders::ALL).title(" Help ");
    f.render_widget(Clear, area); // <- clears background
    f.render_widget(&block, area);
    let content = Paragraph::new(
        "Keys:\n\
         • Tab / Shift+Tab: switch tabs\n\
         • Ctrl+H: toggle help\n\
         • Ctrl+J: add console line\n\
         • Ctrl+L: clear console\n\
         • Esc / Ctrl+C: quit",
    )
    .wrap(Wrap { trim: false });
    f.render_widget(content, block.inner(area));
}

fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(D::Vertical)
        .constraints([
            C::Percentage((100 - pct_y) / 2),
            C::Percentage(pct_y),
            C::Percentage((100 - pct_y) / 2),
        ])
        .split(r);

    let horiz = Layout::default()
        .direction(D::Horizontal)
        .constraints([
            C::Percentage((100 - pct_x) / 2),
            C::Percentage(pct_x),
            C::Percentage((100 - pct_x) / 2),
        ])
        .split(vert[1]);

    horiz[1]
}
