use block::Block;
use util::run_command;

pub struct Title {
    max_chars: usize,
}

impl Title {
    pub fn new(max: usize) -> Title {
        Title {
            max_chars: max,
        }
    }

    pub fn get_title(&self) -> String {
        let title = run_command("xdotool getwindowname $(xdotool getactivewindow)");

        // Truncate long titles
        if title.chars().count() > self.max_chars {
            let mut end = self.max_chars;

            // Don't end on a space
            while title.chars().nth(end - 1).unwrap() == ' ' {
                end -= 1;
            }

            return title.chars().take(end).collect::<String>() + "...";
        }

        title
    }
}

impl Block for Title {
    fn new() -> Title {
        Title::new(50)
    }

    fn output(&self) -> String {
        self.get_title()
    }
}
