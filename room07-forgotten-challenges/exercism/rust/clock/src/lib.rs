use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = (hours + minutes / 60 - if minutes < 0 { 1 } else { 0 }).rem_euclid(24);
        let minutes = minutes.rem_euclid(60);

        Self {
            hours: if hours >= 0 { hours } else { 24 - hours },
            minutes: if minutes >= 0 { minutes } else { 60 - minutes },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let cumuled_minutes = self.minutes + minutes;
        dbg!(cumuled_minutes);
        let cumuled_hours =
            self.hours + (cumuled_minutes / 60 - if cumuled_minutes < 0 { 1 } else { 0 });

        let hours = cumuled_hours.rem_euclid(24);
        let minutes = cumuled_minutes.rem_euclid(60);

        Self {
            hours: if hours >= 0 { hours } else { 24 - hours },
            minutes: if minutes >= 0 { minutes } else { 60 - minutes },
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
