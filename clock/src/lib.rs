use std::fmt::{Display, Debug};

pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let init = Clock {
            hours: 0,
            minutes: 0,
        };
        init.add_minutes(minutes + hours * 60)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;
        let mut new_hours = self.hours;
        while new_minutes >= 60 {
            new_minutes -= 60;
            new_hours += 1;
        }
        while new_minutes < 0 {
            new_minutes += 60;
            new_hours -= 1;
        }
        while new_hours >= 24 {
            new_hours -= 24;
        }
        while new_hours < 0 {
            new_hours += 24;
        }
        Self { hours: new_hours, minutes: new_minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
