use std::process::Command;

pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub enum WindowManagers {
    Bspwm,
    I3,
}

pub fn run_command<T: Into<String>>(cmd: T) -> String {
    let command = Command::new("bash")
        .arg("-c")
        .arg(cmd.into())
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

pub fn run_i32<T: Into<String>>(cmd: T) -> i32 {
    let result = run_command(cmd.into());

    result.parse::<i32>().unwrap_or_else(|e| {
        panic!("Parsing error: {}", e);
    })
}

#[allow(unused_must_use)]
pub fn run_bg<T: Into<String>>(cmd: T) {
    Command::new("bash")
        .arg("-c")
        .arg(cmd.into())
        .spawn();
}
