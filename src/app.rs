use std::collections::HashMap;
use chrono::{Datelike, Weekday};
use tui::widgets::{ListItem, ListState};

#[derive(Debug)]
pub struct Activity{
    pub day: String,
    pub title: String,
    pub description: String,
}

pub struct ListWidgetHandler<'a> {
    pub state: ListState,
    pub list: Vec<ListItem<'a>>
}

impl<'a> ListWidgetHandler<'a>{
    pub fn new(vec: Vec<ListItem<'a>>) -> ListWidgetHandler{
        ListWidgetHandler{
            state: ListState::default(),
            list: vec
        }
    }

    pub fn next(&mut self){
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn last(&mut self){
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


pub struct App<'a>{
    pub date: (i32, u32, u32, Weekday),
    pub weekdays_map: HashMap<Weekday, i32>,
    pub list_handler: ListWidgetHandler<'a>,
}

impl<'a> App<'a>{
    pub fn new() -> App<'a>{

        let day_list = vec![
            ListItem::new("Sunday"),
            ListItem::new("Monday"),
            ListItem::new("Tuesday"),
            ListItem::new("Wednesday"),
            ListItem::new("Thursday"),
            ListItem::new("Friday"),
            ListItem::new("Saturday")
        ];

        let mut week_list = ListWidgetHandler::new(day_list);
        let current_date = chrono::Local::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();
        let dow = current_date.weekday();
        let weekdays = HashMap::from([
            (Weekday::Sun, 0),
            (Weekday::Mon, 1),
            (Weekday::Tue, 2),
            (Weekday::Wed, 3),
            (Weekday::Thu, 4),
            (Weekday::Fri, 5),
            (Weekday::Sat, 6),
        ]);

        week_list.state.select(Some(*weekdays.get(&dow).unwrap() as usize));



        App{
            date: (year, month, day, dow),
            weekdays_map: weekdays,
            list_handler: week_list,
        }
    }
}