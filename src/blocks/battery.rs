use std::process::Command;

use block::Block;
use util::Align;

#[derive(Default)]
pub struct Battery {
    pub icon: Option<(String, Align)>,
    pub icons: Option<(Vec<String>, Align)>,
}

impl Battery {
    pub fn new() -> Battery {
        Battery {
            icon: None,
            icons: None,
        }
    }

    pub fn add_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
    }

    // Can also accept 3 different icons, arranged from low to high battery
    pub fn add_icons(&mut self, icons: [&str; 3], align: Align) {
        let mut m_icons: Vec<String> = Vec::new();

        for icon in icons.iter() {
            m_icons.push(String::from(*icon));
        }

        self.icons = Some((m_icons, align));
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
        if len > 1 {
            battery.truncate(len - 2);
        }

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

            match *align {
                Align::Right => return format!("{}% {}", self.get_battery(), icon),
                _ => return format!("{} {}%", icon, self.get_battery()),
            }
        }

        if let Some(ref x) = self.icons {
            let (ref icons, ref align) = *x;

            let battery = self.get_battery()
                              .parse::<i32>()
                              .unwrap_or_else(|e| {

                                  panic!("Couldn't parse battery. Error: {}", e);
                              });

            let icon: usize;

            if battery > 66 {
                icon = 2;
            } else if battery > 33 {
                icon = 1;
            } else {
                icon = 0;
            }

            match *align {
                Align::Right => return format!("{}% {}", battery, icons[icon]),
                _ => return format!("{} {}%", icons[icon], battery),
            }
        }

        self.get_battery()
    }
}
