#[macro_use]
extern crate serde_derive;

extern crate notify;
extern crate time;
extern crate toml;

use std::thread;
use std::time::Duration;

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

use time::get_time;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

mod bar;
mod block;
mod module;
mod blocks;
mod util;
mod wm;

use bar::Bar;
use module::Module;
use block::Block;
use blocks::*;
use util::{Align, WindowManagers, run_i32, run_bg};

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Config {
    bar: CBar,
    module: Option<Vec<CModule>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct CBar {
    update_interval: u64,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
    wm: Option<String>,
    block: Option<Vec<CBlock>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct CModule {
    align: Option<String>,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
    block: Option<Vec<CBlock>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct CBlock {
    kind: String,
    icon: Option<String>,
    icons: Option<Vec<String>>,
    icon_align: Option<String>,
    active_icon: Option<String>,
    device: Option<String>,
    command: Option<String>,
    format: Option<String>,
    max_chars: Option<usize>,
}

fn create_config() -> PathBuf {
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("Couldn't get home directory!"),
    };

    let folder = format!("{}{}", home.display(), "/.config/rustabari");
    let file = format!("{}{}", folder, "/config.toml");
    let conf_dir = Path::new(folder.as_str());
    let conf_file = Path::new(file.as_str());

    // Create config if it doesn't exist

    if !conf_dir.exists() {
        match fs::create_dir(conf_dir) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't create config directory! Error: {}", e),
        }
    }

    if !conf_file.exists() {
        match fs::File::create(conf_file) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't create config file! Error: {}", e),
        }
    }

    conf_file.to_path_buf()
}

fn parse_config() -> Config {
    let path = create_config();
    let mut file = fs::File::open(path).unwrap_or_else(|e| {
        panic!("Could not open config file! Error: {}", e);
    });

    let mut conf_text = String::new();

    file.read_to_string(&mut conf_text).unwrap_or_else(|e| {
        panic!("Could not read config file! Error: {}", e);
    });

    let config: Config = toml::from_str(conf_text.as_str()).unwrap_or_else(|e| {
        panic!("Could not parse config file! Error: {}", e);
    });

    config
}

fn align<T: Into<String>>(align_string: T) -> Align {
    match align_string.into().as_ref() {
        "left" => Align::Left,
        "center" => Align::Center,
        "right" => Align::Right,
        _ => Align::None,
    }
}

fn build_block(block: &CBlock) -> Box<Block> {
    return match block.kind.as_ref() {
        "battery" => {
            let mut battery = Battery::new();

            // Add icon(s)
            if let Some(ref icon_align) = block.icon_align {
                if let Some(ref icon) = block.icon {
                    battery.add_icon(icon.as_str(), align(icon_align.to_owned()));
                } else if let Some(ref icons) = block.icons {
                    battery.add_icons([
                        icons[0].as_str(),
                        icons[1].as_str(),
                        icons[2].as_str(),
                    ], align(icon_align.to_owned()));
                }
            }

            Box::new(battery)
        },
        "date" => {
            // Date needs a format
            if let Some(ref format) = block.format {
                let mut date = Date::new(format);

                if let Some(ref icon_align) = block.icon_align {
                    if let Some(ref icon) = block.icon {
                        date.add_icon(icon, align(icon_align.to_owned()));
                    }
                }

                Box::new(date)
            } else {
                panic!("Block 'date' requires field 'format'!");
            }
        },
        "music" => {
            let mut music = Music::new();

            if let Some(ref icon_align) = block.icon_align {
                if let Some(ref icon) = block.icon {
                    music.add_icon(icon, align(icon_align.to_owned()));
                }
            }

            if let Some(ref command) = block.command {
                music.set_command(command);
            }

            Box::new(music)
        },
        "wifi" => {
            let mut wifi = Wifi::new();

            if let Some(ref icon_align) = block.icon_align {
                if let Some(ref icon) = block.icon {
                    wifi.add_icon(icon, align(icon_align.to_owned()));
                } else if let Some(ref icons) = block.icons {
                    wifi.add_icons([
                        icons[0].as_str(),
                        icons[1].as_str(),
                        icons[2].as_str(),
                    ], align(icon_align.to_owned()));
                }
            }

            if let Some(ref device) = block.device {
                wifi.set_device(device);
            }

            Box::new(wifi)
        },
        "workspaces" => {
            let mut wsp = Wsp::new();

            if let (&Some(ref icon), &Some(ref active_icon)) = (&block.icon, &block.active_icon) {
                wsp.set_icon(icon.as_str());
                wsp.set_active_icon(active_icon.as_str());
            } else {
                panic!("Block 'workspaces' requires fields 'icon' and 'active_icon'!");
            }

            Box::new(wsp)
        },
        "title" => {
            let mut max_chars: usize = 50;

            if let Some(ref max) = block.max_chars {
                max_chars = *max;
            }

            Box::new(Title::new(max_chars))
        },
        "custom" => {
            let mut custom = Custom::new();

            if let Some(ref command) = block.command {
                custom.set_command(command.to_owned());
            }

            Box::new(custom)
        },
        _ => panic!("Unrecognized kind \"{}\"", block.kind),
    }
}

fn build_module(cmodule: &CModule) -> Module {
    let mut module = Module::new(align(match cmodule.align {
        Some(ref x) => x,
        None => "none",
    }));

    if let Some(ref sep) = cmodule.separator {
        module.add_separator(sep.as_str());
    }

    if let Some(ref bg) = cmodule.background {
        module.set_background(bg);
    }

    if let Some(ref fg) = cmodule.foreground {
        module.set_foreground(fg);
    }

    if let Some(ref blocks) = cmodule.block {
        for block in blocks {
            module.add_boxed(build_block(&block));
        }
    }

    module
}

fn setup(config: &Config) -> Bar {
    // Set up bar
    let mut bar = Bar::new(config.bar.update_interval);

    if let Some(ref sep) = config.bar.separator {
        bar.set_separator(sep);
    }

    if let Some(ref bg) = config.bar.background {
        bar.set_background(bg);
    }

    if let Some(ref fg) = config.bar.foreground {
        bar.set_foreground(fg);
    }

    // Add blocks
    if let Some(ref blocks) = config.bar.block {
        for block in blocks {
            bar.add_boxed(build_block(&block));
        }
    }

    // Set up and add modules
    if let Some(ref modules) = config.module {
        for cmodule in modules {
            let mut module = build_module(cmodule);

            if let Some(ref bg) = config.bar.background {
                module.set_global_background(bg);
            }

            if let Some(ref fg) = config.bar.foreground {
                module.set_global_foreground(fg);
            }

            bar.add_module(module);
        }
    }

    bar
}

fn display(bar: Bar, rx: &Receiver<DebouncedEvent>) {
    loop {
        bar.run();

        thread::sleep(Duration::from_secs(bar.update_interval));

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {},
        }
    }
}

fn subscribe(bar: Bar, wsp: WindowManagers, rx: &Receiver<DebouncedEvent>) {
    match wsp {
        // Just bspwm for now
        _ => run_bg("bspc subscribe > /tmp/rustabari_subscribe"),
    };

    let inital = get_time().sec;
    let mut previous = 0;
    let mut file_length = run_i32("cat /tmp/rustabari_subscribe | wc -l");

    loop {
        let len = run_i32("cat /tmp/rustabari_subscribe | wc -l");
        let elapsed = get_time().sec - inital;

        // Update on WM action and every `self.update_interval` seconds
        if len != file_length {
            file_length = len;

            bar.run();
        } else if elapsed != previous && elapsed as u64 % bar.update_interval == 0 {
            previous = elapsed;

            bar.run();
        }

        thread::sleep(Duration::from_millis(100));

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {},
        }
    }
}

fn main() {
    let mut config: Config = parse_config();
    let mut bar = setup(&config);

    let (tx, rx): (Sender<DebouncedEvent>, Receiver<DebouncedEvent>) = channel();

    // Monitor config for changes
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))
        .unwrap_or_else(|e| panic!("Error watching config: {}", e));
    let _ = watcher.watch(create_config(), RecursiveMode::NonRecursive);

    loop {
        // TODO: Subprocess lemonbar
        // Run
        if let Some(ref wm) = config.bar.wm {
            match wm.as_ref() {
                "bspwm" => subscribe(bar, WindowManagers::Bspwm, &rx),
                _ => display(bar, &rx),
            }
        }

        config = parse_config();
        bar = setup(&config);
    }
}
