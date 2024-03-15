use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: 0, minutes: 0 }.add_minutes(minutes + (hours * 60))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes + (self.hours * 60);
        let mut new_hours: i32 = total_minutes / 60;
        let mut new_minutes = total_minutes % 60;
        if new_minutes < 0 {
            new_minutes += 60;
            new_hours -= 1;
        }
        new_hours = new_hours % 24;
        if new_hours < 0 {
            new_hours += 24;
        }
        Clock { hours: new_hours, minutes: new_minutes }
    }

}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_hour: String = "0".to_owned();
        let mut display_minutes: String = "0".to_owned();
        if self.hours < 10 {
            display_hour.push_str(self.hours.to_string().as_str());
        } else {
            display_hour = self.hours.to_string();
        }

        if self.minutes < 10 {
            display_minutes.push_str(self.minutes.to_string().as_str());
        } else {
            display_minutes = self.minutes.to_string();
        }

        write!(f, "{}:{}", display_hour, display_minutes)?;
        Ok(())
    }
}