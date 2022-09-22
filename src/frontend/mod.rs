use tui::backend::Backend;
use tui::Frame;
use tui::layout::Direction::{Horizontal, Vertical};
use tui::layout::{Constraint, Layout};
use crate::app::App;
use crate::backend::calendar::calender_year::Week;

pub mod ui;

pub fn draw_ui<B>(rect: &mut Frame<B>, app: &mut App, week: &mut Week)
    where
        B: Backend,
{
    let size = rect.size();
    ui::check_size(&size);

    let o = week.current_day as usize;

    let date: &String = week.week_map.get(o).unwrap();

    let chunks = Layout::default()
        .direction(Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(size);

    let body_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[0]);

    let left_body = ui::draw_schedule(app, date);
    rect.render_widget(left_body, body_chunks[1]);


    let list = ui::draw_list(app);
    rect.render_stateful_widget(list, body_chunks[0], &mut app.list_handler.state);

}