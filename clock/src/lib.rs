use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    internal: i32,
}

impl Clock {
    const DAY_MINUTES: i32 = (60 * 24);

    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = hours * 60 + minutes;

        Clock::with_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let internal = self.internal + minutes;

        Clock::with_minutes(internal)
    }

    fn hours(&self) -> i32 {
        self.internal / 60
    }

    fn minutes(&self) -> i32 {
        self.internal % 60
    }

    fn with_minutes(minutes: i32) -> Self {
        let mut minutes = minutes % Clock::DAY_MINUTES;
        if minutes < 0 {
            minutes += Clock::DAY_MINUTES;
        }
        Clock { internal: minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
