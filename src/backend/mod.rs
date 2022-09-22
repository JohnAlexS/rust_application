pub mod event_handler;
pub mod key_map;
pub mod calendar;

use crate::backend::event_handler::Ticks;
use crate::backend::key_map::Key;
use crate::app::App;
use crate::backend::calendar::calender_year::Week;

#[derive(PartialEq)]
pub enum State{
    Continue,
    End,
    //Error
}


pub fn event(input: Ticks, app: &mut App, week: &mut Week) -> State{
    match input{
        Ticks::Input(Key::Down) => {
            week.next_date();
            app.list_handler.next();
            self::State::Continue
        },
        Ticks::Input(Key::Up) => {
            week.previous_date();
            app.list_handler.last();
            self::State::Continue
        },
        Ticks::Input(Key::Enter) => {

            self::State::Continue
        }
        Ticks::Input(Key::Ctrl('c')) => self::State::End,
        _ => self::State::Continue
    }
}





