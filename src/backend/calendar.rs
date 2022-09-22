pub mod calender_year{
    use std::collections::HashMap;
    use crate::app::App;

    pub struct Week{
        pub week_map: Vec<String>,
        pub current_day: i32
    }

    impl Week {
        pub fn new(app: &mut App)-> Week{
            let mut week = Vec::new();
            let month_31: Vec<u32> = (1..32).collect();
            let month_30: Vec<u32> = (1..31).collect();
            let month_28: Vec<u32> = (1..29).collect();
            let year: HashMap<u32, &Vec<u32>> = HashMap::from([
                (1, &month_31),
                (2, &month_28),
                (3, &month_31),
                (4, &month_30),
                (5, &month_31),
                (6, &month_30),
                (7, &month_31),
                (8, &month_31),
                (9, &month_30),
                (10, &month_31),
                (11, &month_30),
                (12, &month_31),
            ]);

            let start_wday = app.weekdays_map.get(&app.date.3).unwrap();
            let mut start_day = app.date.2;
            let mut start_month = app.date.1;
            let mut start_year = app.date.0;
            let mut vec: &&Vec<u32>;


            week.push(format!("{}/{}/{}", start_month, start_day, start_year));

            let mut i = *start_wday;
            let mut j = *start_wday;
            loop {
                if i > 0 {
                    if start_day == 1 {
                        start_month = {
                            if start_month == 1 {
                                start_year -= 1;
                                12
                            } else {
                                start_month - 1
                            }
                        };
                        vec = year.get(&start_month).unwrap();
                        start_day = *vec.last().unwrap();
                    } else {
                        start_day -= 1;
                    }
                    let date = format!("{}/{}/{}", start_month, start_day, start_year);
                    week.insert(0, date);
                    i -= 1;
                }
                else{
                    break;
                }
            }

            let mut start_day = app.date.2;
            let mut start_month = app.date.1;
            let mut start_year = app.date.0;
            let mut vec = year.get(&start_month).unwrap();

            loop {
                if j < 6{
                    if &start_day == vec.last().unwrap() {
                        start_month = {
                            if start_month == 12{
                                start_year += 1;
                                1
                            }
                            else{
                                start_month+1
                            }
                        };
                        vec = year.get(&start_month).unwrap();
                        start_day = 1;
                    }
                    else{
                        start_day+=1;
                    }
                    let date = format!("{}/{}/{}", start_month, start_day, start_year);
                    print!("{}", date);
                    week.push(date);
                    j+=1;
                }
                else{
                    break;
                }
            }
            let start = *start_wday;

            Week{
                week_map: week,
                current_day: start
            }
        }

        pub fn next_date(&mut self) {
            self.current_day = {
                if self.current_day == 6{
                    0
                }
                else {
                    self.current_day + 1
                }
            };
        }

        pub fn previous_date(&mut self) {
            self.current_day = {
                if self.current_day == 0{
                    6
                }
                else {
                    self.current_day - 1
                }
            };
        }
    }
}