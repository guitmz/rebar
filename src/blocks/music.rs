use std::process::Command;

use block::Block;
use util::Align;

#[derive(Default)]
pub struct Music {
    pub icon: Option<(String, Align)>,
    pub command: String,
}

impl Music {
    pub fn new() -> Music {
        Music {
            icon: None,
            command: String::from("mpc current"),
        }
    }

    pub fn add_icon(&mut self, icon: &str, align: Align) {
        self.icon = Some((String::from(icon), align));
    }

    pub fn set_command(&mut self, cmd: &str) {
        self.command = String::from(cmd);
    }

    fn get_song(&self) -> String {
        let cmd_split = self.command.split(' ');
        let mut cmds: Vec<&str> = cmd_split.collect();

        let music_cmd = Command::new(cmds.remove(0))
            .args(&cmds)
            .output().unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e);
            });

        let song_cow = String::from_utf8_lossy(&music_cmd.stdout);

        let mut song = song_cow.to_owned().to_string();
        let len = song.len();

        // Remove newline from song
        if len > 0 {
            song.truncate(len - 1);
        }

        song
    }
}

impl Block for Music {
    fn new() -> Music {
        Music::new()
    }

    fn output(&self) -> String {
        if let Some(ref x) = self.icon {
            let (ref icon, ref align) = *x;

            match *align {
                Align::Right => return format!("{} {}", self.get_song(), icon),
                _ => return format!("{} {}", icon, self.get_song()),
            }
        }

        self.get_song()
    }
}
