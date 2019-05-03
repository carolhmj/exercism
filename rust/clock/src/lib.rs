use std::fmt;
use std::error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Clock {
    m: i32,
}

#[derive(Debug, Clone)]
pub struct ClockError;

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while parsing clock")
    }
}

impl error::Error for ClockError {
    fn description(&self) -> &str {
        "error while parsing clock"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            m: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            m: self.m + minutes,
        }
    }

    fn to_hours_and_minutes(&self) -> (i32, i32) {
        let wrkm: i32 = self.m % 1440 + 1440;
        let h: i32 = ((wrkm / 60) % 24).abs();
        let m: i32 = wrkm % 60;
        (h, m)
    }

    fn from_str(s: &str) -> Option<Clock> {
        let mut split = s.split(":");
        
        let h : i32 = split.next()?.parse().ok()?;
        let m : i32 = split.next()?.parse().ok()?;
            
        Some(Clock::new(h, m))
    }
}

impl FromStr for Clock {
    type Err = ClockError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Clock::from_str(s).ok_or(ClockError)
    }
}

impl TryFrom<&str> for Clock {
   type Error = ClockError;

   fn try_from(value: &str) -> Result<Self, Self::Error> {
        Clock::from_str(value).ok_or(ClockError)
   }
}


impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.to_hours_and_minutes() == other.to_hours_and_minutes()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (h, m) = self.to_hours_and_minutes();
        write!(f, "{:02}:{:02}", h, m)
    }
}