use std::fmt;

#[derive(Debug)]
pub struct Clock {
    m: i32,
}

#[derive(PartialEq)]
struct HM {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            m: hours*60 + minutes,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock {
            m: self.m + minutes,
        }
    }

    fn to_hours_and_minutes(&self) -> HM {
        let wrkm : i32 = if self.m > 0 { self.m } else { (self.m.abs()/1440+1)*1440 + self.m };
        let h : i32 = ((wrkm / 60) % 24).abs();
        let m : i32 = wrkm % 60;
        HM {
            h,
            m,
        }
    }

    pub fn to_string(&self) -> String {
        let hm = self.to_hours_and_minutes();
        format!("{:02}:{:02}", hm.h, hm.m)
    }

    pub fn from(s: String) -> Clock {
        let mut split = s.split(":");
        let h : i32 = split.next().unwrap().parse().unwrap();
        let m : i32 = split.next().unwrap().parse().unwrap();
        Clock::new(h, m)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.to_hours_and_minutes() == other.to_hours_and_minutes()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hm = self.to_hours_and_minutes();
        write!(f, "{}:{}", hm.h, hm.m)
    }
}