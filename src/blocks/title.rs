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
        if title.len() > 50 {
            return title[0..50].to_string() + "...";
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
