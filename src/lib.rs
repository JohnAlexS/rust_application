mod frontend;
mod backend;
mod app;

use std::io;
use std::time::Duration;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::{Terminal};
use eyre::Result;
use crate::backend::event_handler::EventHandler;
use crate::backend::calendar::calender_year::Week;
use crate::backend::{event, State};


pub async fn start_main_loop() -> Result<()>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let mut app = app::App::new();
    let mut week = Week::new(&mut app);

    let poll_time = Duration::from_millis(500);
    let mut handle = EventHandler::new_handler(poll_time);

    loop{
        terminal.draw(|rect| {
            frontend::draw_ui(rect, &mut app, &mut week);
        })?;

        if let State::End = event(handle.read_next().await, &mut app, &mut week){
            handle.end();
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}