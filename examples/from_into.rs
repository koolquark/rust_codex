use std::fmt::Display;

/// After setting options, the builder is used to create a handshake future.
#[derive(Debug)]
struct Time {
    hours: String,
    minutes: String,
}

impl Time {
    fn show(self: &Self) {
        println!("{}:{}", self.hours, self.minutes);
    }
}

impl From<String> for Time {
    fn from(s: String) -> Time {
        let parts = s.split(":").collect::<Vec<&str>>();

        Time {
            hours: String::from(parts[0]),
            minutes: String::from(parts[1]),
        }
    }
}

impl From<&str> for Time {
    fn from(s: &str) -> Time {
        let parts = s.split(":").collect::<Vec<&str>>();
        Time {
            hours: String::from(parts[0]),
            minutes: String::from(parts[1]),
        }
    }
}

impl TryFrom<&str> for Time {
  type Error = &'sattic str; 

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    
  }

}

fn main() {
    let t: Time = Time::from(String::from("10:56"));
    t.show();

    let t: Time = Time::from("10:57");
    t.show();

    let s: String = String::from("11:32");
    let t: Time = s.into();
    t.show();

    let s = "11:33";
    let t: Time = s.into();
    t.show();
}
