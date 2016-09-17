use std::process::Command;

use block::Block;
use util::Align;

pub struct Battery {
    pub icon: Option<(String, Align)>,
}

fn get_battery() -> String {
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

impl Block for Battery {
    fn new(icon: Option<(&str, Align)>) -> Battery {
        // If an icon is passed, convert it to String
        let get_icon = |i: Option<(&str, Align)>| {
            if let Some(x) = i {
                Some((String::from(x.0), x.1))
            } else {
                None
            }
        };

        Battery {
            icon: get_icon(icon)
        }
    }

    fn output(&self) -> String {
        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match align {
                &Align::Right => return format!("{}% {}", get_battery(), icon),
                _ => return format!("{} {}%", icon, get_battery()),
            }
        }

        get_battery()
    }
}
