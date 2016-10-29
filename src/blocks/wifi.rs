use std::process::Command;

use block::Block;
use util::Align;

pub struct Wifi {
    pub icon: Option<(String, Align)>,
    pub icons: Option<(Vec<String>, Align)>,
    pub ssid_cmd: String,
    pub strength_cmd: String,
    pub device: String,
}

impl Wifi {
    pub fn new() -> Wifi {
        Wifi {
            icon: None,
            icons: None,
            ssid_cmd: String::from("iw dev {} link | grep SSID: | cut -d' ' -f2"),
            strength_cmd: String::from("iw dev {} link | grep signal: | cut -d' ' -f2"),
            device: String::from("wlan0"),
        }
    }

    // Can accept a single static icon
    pub fn add_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
    }

    // Can also accept 3 different icons, arranged from low to high connection strength
    pub fn add_icons(&mut self, icons: &[&str; 3], align: Align) {
        let mut m_icons: Vec<String> = Vec::new();

        for icon in icons.iter() {
            m_icons.push(String::from(*icon));
        }

        self.icons = Some((m_icons, align));
    }

    pub fn set_device(&mut self, device: &str) {
        self.device = String::from(device);
    }

    pub fn set_ssid_cmd(&mut self, cmd: &str) {
        self.ssid_cmd = String::from(cmd);
    }

    pub fn set_strength_cmd(&mut self, cmd: &str) {
        self.strength_cmd = String::from(cmd);
    }
}

fn run_command(cmd: String) -> String {
    let command = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output().unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e);
        });

    let cmd_cow = String::from_utf8_lossy(&command.stdout);

    let mut out = cmd_cow.to_string();
    let len = out.len();

    // Remove newline
    if len > 0 {
        out.truncate(len - 1);
    }

    out
}

impl Block for Wifi {
    fn new() -> Wifi {
        Wifi::new()
    }

    fn output(&self) -> String {
        let ssid_cmd = str::replace(self.ssid_cmd.as_str(), "{}", self.device.as_str());
        let ssid = run_command(ssid_cmd);

        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match align {
                &Align::Right => return format!("{} {}", ssid, icon),
                _ => return format!("{} {}", icon, ssid),
            }
        }

        if let Some(ref x) = self.icons {
            let (ref icons, ref align) = *x;

            let strength_cmd = str::replace(self.strength_cmd.as_str(), "{}",
                                            self.device.as_str());
            let strength = run_command(strength_cmd)
                               .parse::<i32>()
                               .unwrap_or_else(|e| {
                                   // If not connected to wifi, don't panic
                                   if ssid.is_empty() {
                                       return 0;
                                   }

                                   panic!("Couldn't parse strength. Error: {}", e);
                               });

            let icon: usize;

            if strength > -50 {
                icon = 2;
            } else if strength > -70 {
                icon = 1;
            } else {
                icon = 0;
            }

            match align {
                &Align::Right => return format!("{} {}", ssid, icons[icon]),
                _ => return format!("{} {}", icons[icon], ssid),
            }
        }

        String::new()
    }
}
