use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;

pub(crate) fn draw_schedule<'a>(_app: &mut App, date: &String) -> Paragraph<'a>{

    let title = format!("Schedule for {}", date);
    Paragraph::new(" ")
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
        )
}

pub(crate) fn draw_list<'a>(app: &mut App<'a>) -> List<'a>{
    let vec: Vec<ListItem> = app.list_handler.list.clone();
    List::new(vec)
        .block(
            Block::default()
                .title("Your week")
                .borders(Borders::ALL)
        )
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Red))
}

pub(crate) fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}