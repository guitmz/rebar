use std::process::Command;

use block::Block;

pub struct Battery {
    pub icon: Option<String>,
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

    // Remove end comma
    battery.truncate(len - 1);

    battery
}

impl Block for Battery {
    fn new(icon: Option<&str>) -> Battery {
        // If an icon is passed, convert it to String
        if let Some(x) = icon {
            Battery {
                icon: Some(String::from(x))
            }
        } else {
            Battery {
                icon: None
            }
        }
    }

    fn output(&self) -> String {
        match self.icon {
            Some(ref icon) => {
                format!("{} {}", icon, get_battery())
            },

            None => {
                format!("{}", get_battery())
            }
        }
    }
}
