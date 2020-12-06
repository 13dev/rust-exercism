use std::fmt::*;

#[derive(Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (hours * 60) + minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: self.minutes + minutes,
        }
    }
    pub fn get_hours_minutes(&self) -> (u32, u32) {
        (
            self.minutes.div_euclid(60).rem_euclid(24) as u32,
            self.minutes.rem_euclid(60) as u32,
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.get_hours_minutes() == other.get_hours_minutes()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (hours, minutes) = self.get_hours_minutes();
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}