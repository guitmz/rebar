use std::process::Command;

use block::Block;
use util::Align;

pub struct Date {
    pub icon: Option<(String, Align)>,
    pub format: String,
}

impl Date {
    pub fn new(format: &str, icon: Option<(&str, Align)>) -> Date {
        // If an icon is passed, convert it to String
        if let Some(x) = icon {
            Date {
                icon: Some((String::from(x.0), x.1)),
                format: String::from(format),
            }
        } else {
            Date {
                icon: None,
                format: String::from(format),
            }
        }
    }

    fn get_date(&self) -> String {
        let datecmd = Command::new("date")
            .arg(format!("+{}", self.format))
            .output().unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e);
            });

        let date_cow = String::from_utf8_lossy(&datecmd.stdout);

        let mut date = date_cow.to_owned().to_string();
        let len = date.len();

        // Remove newline from date
        date.truncate(len - 1);

        date
    }
}

impl Block for Date {
    fn new(icon: Option<(&str, Align)>) -> Date {
        // If an icon is passed, convert it to String
        if let Some(x) = icon {
            Date {
                icon: Some((String::from(x.0), x.1)),
                format: String::new()
            }
        } else {
            Date {
                icon: None,
                format: String::new()
            }
        }
    }

    fn output(&self) -> String {
        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match align {
                &Align::Right => return format!("{} {}", self.get_date(), icon),
                _ => return format!("{} {}", icon, self.get_date()),
            }
        }

        self.get_date()
    }
}
