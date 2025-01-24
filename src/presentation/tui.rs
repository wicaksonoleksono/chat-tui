use std::io;
use std::time::Duration;
use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use crate::presentation::controllers::ChatController;
use crate::application::services::ChatApi;

pub struct TuiApp<T: ChatApi> {
    controller: ChatController<T>,
    input_buffer: String,
    show_help: bool,
    mode: TuiMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TuiMode {
    Normal,
    ModelChange,
}

impl<T: ChatApi> TuiApp<T> {
    pub fn new(controller: ChatController<T>) -> Self {
        Self {
            controller,
            input_buffer: String::new(),
            show_help: false,
            mode: TuiMode::Normal,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Enable raw mode so we can capture keystrokes without echoing them
        enable_raw_mode()?;

        // Prepare the stdout for TUI rendering
        let mut stdout = io::stdout();

        // Clear the screen and move cursor to top-left to avoid overlap
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        // Create the Crossterm backend and ratatui Terminal
        let backend = CrosstermBackend::new(&mut stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // Draw the UI
            terminal.draw(|f| {
                let size = f.size();

                // Divide screen: top (help or conversation), middle (status), bottom (input)
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(5),   // conversation or help
                        Constraint::Length(1),// status line
                        Constraint::Length(3),// input
                    ])
                    .split(size);

                // 1) Render either the help screen or conversation
                if self.show_help {
                    // Help instructions
                    let text = Text::from(vec![
                        Spans::from("Help & Key Bindings:"),
                        Spans::from("  • Press [Enter] to send a message"),
                        Spans::from("  • Press [Esc]   to quit"),
                        Spans::from("  • Press [m]     to change model (e.g., 'nemotron')"),
                        Spans::from("  • Press [h]     to toggle this help panel"),
                    ]);
                    let help_block = Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .block(Block::default().borders(Borders::ALL).title("Help"));
                    f.render_widget(help_block, chunks[0]);
                } else {
                    // Conversation
                    let messages: Vec<Spans> = self.controller
                        .conversation
                        .messages
                        .iter()
                        .map(|m| {
                            Spans::from(vec![
                                Span::styled(
                                    format!("{}: {}", m.sender, m.content),
                                    Style::default().fg(Color::White),
                                ),
                            ])
                        })
                        .collect();

                    let msg_block = Paragraph::new(messages)
                        .wrap(Wrap { trim: false })
                        .block(Block::default().borders(Borders::ALL).title("Conversation"));
                    f.render_widget(msg_block, chunks[0]);
                }

                // 2) Status line (displays current model, short help prompt)
                let status_text = Spans::from(vec![
                    Span::styled(" Model: ", Style::default().fg(Color::Yellow)),
                    Span::raw(self.controller.conversation.current_model.clone()),
                    Span::raw(" | Press [h] for help "),
                ]);
                let status_block = Paragraph::new(status_text)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(status_block, chunks[1]);

                // 3) Input area or model-change prompt
                let input_title = match self.mode {
                    TuiMode::Normal => "Your message (Enter to send)",
                    TuiMode::ModelChange => "Change Model (Enter to confirm)",
                };
                let input_block = Paragraph::new(self.input_buffer.as_ref())
                    .block(Block::default().borders(Borders::ALL).title(input_title));
                f.render_widget(input_block, chunks[2]);
            })?;

            // Handle key events with a small poll timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.mode {
                        TuiMode::Normal => match key_event.code {
                            KeyCode::Char('h') => {
                                self.show_help = !self.show_help;
                            }
                            KeyCode::Char('m') => {
                                self.mode = TuiMode::ModelChange;
                                self.input_buffer.clear();
                            }
                            KeyCode::Char(c) => {
                                self.input_buffer.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input_buffer.pop();
                            }
                            KeyCode::Enter => {
                                let user_input = self.input_buffer.trim().to_string();
                                if !user_input.is_empty() {
                                    let _ = self.controller.on_user_message(&user_input);
                                }
                                self.input_buffer.clear();
                            }
                            KeyCode::Esc => {
                                // Quit the TUI entirely
                                break;
                            }
                            _ => {}
                        },
                        TuiMode::ModelChange => match key_event.code {
                            KeyCode::Char(c) => {
                                self.input_buffer.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input_buffer.pop();
                            }
                            KeyCode::Enter => {
                                let new_model = self.input_buffer.trim().to_string();
                                if !new_model.is_empty() {
                                    let _ = self.controller.on_change_model(&new_model);
                                }
                                self.input_buffer.clear();
                                self.mode = TuiMode::Normal;
                            }
                            KeyCode::Esc => {
                                // Cancel model change, go back to normal mode
                                self.input_buffer.clear();
                                self.mode = TuiMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        // Disable raw mode when done
        disable_raw_mode()?;
        Ok(())
    }
}
