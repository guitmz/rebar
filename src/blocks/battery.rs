use std::process::Command;

use block::Block;
use util::Align;

pub struct Battery {
    pub icon: Option<(String, Align)>,
}

impl Battery {
    pub fn new() -> Battery {
        Battery {
            icon: None,
        }
    }

    pub fn add_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
    }

    fn get_battery(&self) -> String {
        // Get battery percentage using acpi command
        let acpi = Command::new("acpi")
            .arg("-b")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e);
            });

        let battery_cow = String::from_utf8_lossy(&acpi.stdout);
        let mut battery = battery_cow.split_whitespace().nth(3).unwrap().to_string();
        let len = battery.len();

        // Remove end comma and percent sign
        battery.truncate(len - 2);

        battery
    }
}

impl Block for Battery {
    fn new() -> Battery {
        Battery::new()
    }

    fn output(&self) -> String {
        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match align {
                &Align::Right => return format!("{}% {}", self.get_battery(), icon),
                _ => return format!("{} {}%", icon, self.get_battery()),
            }
        }

        self.get_battery()
    }
}
