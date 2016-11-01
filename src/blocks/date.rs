use std::process::Command;

use block::Block;
use util::Align;

#[derive(Default)]
pub struct Date {
    pub icon: Option<(String, Align)>,
    pub format: String,
}

impl Date {
    pub fn new(format: &str) -> Date {
        Date {
            icon: None,
            format: String::from(format),
        }
    }

    pub fn add_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
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
        if len > 0 {
            date.truncate(len - 1);
        }

        date
    }
}

impl Block for Date {
    fn new() -> Date {
        Date {
            icon: None,
            format: String::new()
        }
    }

    fn output(&self) -> String {
        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match *align {
                Align::Right => return format!("{} {}", self.get_date(), icon),
                _ => return format!("{} {}", icon, self.get_date()),
            }
        }

        self.get_date()
    }
}
