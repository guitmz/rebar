use std::process::Command;

use block::Block;

pub struct Music {
    pub icon: Option<String>,
    pub command: String,
}

impl Music {
    pub fn new(command: Option<&str>, icon: Option<&str>) -> Music {
        let mut cmd = String::from("mpc current");

        if let Some(x) = command {
            cmd = String::from(x);
        }

        // If an icon is passed, convert it to String
        if let Some(x) = icon {
            Music {
                icon: Some(String::from(x)),
                command: cmd,
            }
        } else {
            Music {
                icon: None,
                command: cmd,
            }
        }
    }

    fn get_song(&self) -> String {
        let cmd_split = self.command.split(" ");
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
    fn new(icon: Option<&str>) -> Music {
        Music::new(Some("mpc current"), icon)
    }

    fn output(&self) -> String {
        self.get_song()
    }
}
