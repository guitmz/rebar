use block::Block;
use blocks::wm::*;
use wm::WindowManager;
use util::WindowManagers;

pub struct Wsp {
    wm: Box<WindowManager>,
    icon: String,
    active_icon: String,
}

impl Wsp {
    pub fn new() -> Wsp {
        Wsp {
            wm: Box::new(Bspwm::new()),
            icon: String::new(),
            active_icon: String::new(),
        }
    }

    pub fn set_wm(&mut self, wm: WindowManagers) {
        self.wm = match wm {
            // I3 => Box::new(I3::new()),
            _ => Box::new(Bspwm::new()),
        };
    }

    pub fn set_icon<T: Into<String>>(&mut self, icon: T) {
        self.icon = icon.into();
    }

    pub fn set_active_icon<T: Into<String>>(&mut self, active_icon: T) {
        self.active_icon = active_icon.into();
    }
}

impl Block for Wsp {
    fn new() -> Wsp {
        Wsp::new()
    }

    fn output(&self) -> String {
        let current = self.wm.current_desktop();
        let num = self.wm.num_desktops();

        let mut out = String::new();

        for desktop in 0..num {
            if desktop == current {
                out.push_str(&self.active_icon);
            } else {
                out.push_str(&self.icon);
            }

            // Padding after each except end
            if desktop < num - 1 {
                out.push_str(" ");
            }
        }

        out
    }
}
