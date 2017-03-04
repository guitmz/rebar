use std::process::Command;

use block::Block;
use util::{Align, run_command};

#[derive(Default)]
pub struct Custom {
    pub icon: Option<(String, Align)>,
    pub command: Option<String>,
}

impl Custom {
    pub fn new() -> Custom {
        Custom {
            icon: None,
            command: None,
        }
    }

    pub fn set_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
    }

    pub fn set_command<T: Into<String>>(&mut self, cmd: T) {
        self.command = Some(cmd.into());
    }
}

impl Block for Custom {
    fn new() -> Custom {
        Custom::new()
    }

    fn output(&self) -> String {
        let mut output = String::new();

        if let Some(ref command) = self.command {
            output = run_command(command.to_owned());
            // println!("{:?}", output);
        }

        output
    }
}
