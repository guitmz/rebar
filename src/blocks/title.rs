use block::Block;
use util::run_command;

pub struct Title {

}

impl Title {
    pub fn new() -> Title {
        Title {
            
        }
    }

    pub fn get_title(&self) -> String {
        let title = run_command("xdotool getwindowname $(xdotool getactivewindow)");
        
        // Truncate long titles
        if title.chars().count() > 50 {
            let mut end = 50;

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
        Title::new()
    }

    fn output(&self) -> String {
        self.get_title()
    }
}
