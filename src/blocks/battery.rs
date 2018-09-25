use std::process::Command;

use block::Block;
use util::{Align, run_bg};

#[derive(Default)]
pub struct Battery {
    icon: Option<(String, Align)>,
    icons: Option<(Vec<String>, Align)>,
    monitor_battery: bool,
    sleep_cooldown: bool,
    warn_cooldown: bool,
}

impl Battery {
    pub fn new(monitor: bool) -> Battery {
        Battery {
            icon: None,
            icons: None,
            monitor_battery: monitor,
            sleep_cooldown: true,
            warn_cooldown: true,
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
        // Get battery percentage /sys/class/power_supply
        let acpi = Command::new("cat")
            .arg("/sys/class/power_supply/BAT0/capacity")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e);
            });

        let battery_cow = String::from_utf8_lossy(&acpi.stdout);

        battery_cow.trim().to_string()
    }

    // Monitors battery usage.
    fn monitor(&mut self) {
        let battery = self.get_battery()
            .parse::<i32>()
            .unwrap_or_else(|e| {
                panic!("Couldn't parse battery {}. Error: {}", self.get_battery(), e);
            });

        // If <= 2%, hybrid suspend (to RAM and disk)
        // If <= 5%, warn
        let sleep_pct = 2;
        let warning_pct = 5;

        // Reset cooldowns when charged above warning_pct
        if battery > warning_pct {
            self.sleep_cooldown = false;
            self.warn_cooldown = false;
        }

        if battery <= sleep_pct && !self.sleep_cooldown {
            self.sleep_cooldown = true;
            run_bg("systemctl hybrid-sleep");
        } else if battery <= warning_pct && !self.warn_cooldown {
            self.warn_cooldown = true;
            run_bg("notify-send -u critical 'Battery low!'");
        }
    }
}

impl Block for Battery {
    fn new() -> Battery {
        Battery::new(false)
    }

    fn output(&self) -> String {
        let battery_string = self.get_battery();
        let battery = battery_string.parse::<i32>()
            .unwrap_or_else(|e| {
                panic!("Couldn't parse battery. Error: {}", e);
            });

        if let Some((ref icon, ref align)) = self.icon {
            match *align {
                Align::Right => return format!("{}% {}", battery_string, icon),
                _ => return format!("{} {}%", icon, battery_string),
            }
        }

        if let Some((ref icons, ref align)) = self.icons {
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

        battery_string
    }

    fn tasks(&mut self) {
        if self.monitor_battery {
            self.monitor();
        }
    }
}
