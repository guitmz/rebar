use wm::WindowManager;
use util::run_i32;

pub struct Bspwm {
    num: i32,
}

impl WindowManager for Bspwm {
    fn new() -> Bspwm {
        Bspwm {
            num: run_i32("bspc query -D | wc -l"),
        }
    }

    fn num_desktops(&self) -> i32 {
        self.num
    }

    fn active_desktops(&self) -> Vec<i32> {
        let mut active = Vec::new();

        for i in 0..self.num {
            let num = run_i32(format!("bspc query -N -d {} | wc -l", i));

            if num > 0 || i == self.current_desktop() + 1 {
                active.push(i + 1);
            }
        }

        active
    }

    fn current_desktop(&self) -> i32 {
        let current = run_i32("bspc query -D -d | tail -c 2 | bc");

        current - 2
    }
}
