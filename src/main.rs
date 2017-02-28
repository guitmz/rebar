#[macro_use]
extern crate serde_derive;

extern crate time;
extern crate toml;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

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
use util::{Align, WindowManagers};

#[derive(Debug, Deserialize)]
struct Config {
    bar: CBar,
    module: Option<Vec<CModule>>,
}

#[derive(Debug, Deserialize)]
struct CBar {
    update_interval: u64,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
    wm: Option<String>,
    block: Option<Vec<CBlock>>,
}

#[derive(Debug, Deserialize)]
struct CModule {
    align: String,
    separator: Option<String>,
    block: Option<Vec<CBlock>>,
}

#[derive(Debug, Deserialize)]
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

fn align(align_string: &String) -> Align {
    match align_string.as_ref() {
        "center" => Align::Center,
        "right" => Align::Right,
        _ => Align::Left,
    }
}

fn build_block(block: &CBlock) -> Box<Block> {
    return match block.kind.as_ref() {
        "battery" => {
            let mut battery = Battery::new();

            // Add icon(s)
            if let Some(ref icon_align) = block.icon_align {
                if let Some(ref icon) = block.icon {
                    battery.add_icon(icon.as_str(), align(&icon_align));
                } else if let Some(ref icons) = block.icons {
                    battery.add_icons([
                        icons[0].as_str(),
                        icons[1].as_str(),
                        icons[2].as_str(),
                    ], align(&icon_align));
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
                        date.add_icon(icon, align(&icon_align));
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
                    music.add_icon(icon, align(&icon_align));
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
                    wifi.add_icon(icon, align(&icon_align));
                } else if let Some(ref icons) = block.icons {
                    wifi.add_icons([
                        icons[0].as_str(),
                        icons[1].as_str(),
                        icons[2].as_str(),
                    ], align(&icon_align));
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
        _ => panic!("Unrecognized kind \"{}\"", block.kind),
    }
}

fn build_module(cmodule: &CModule) -> Module {
    let mut module = Module::new(align(&cmodule.align));

    if let Some(ref sep) = cmodule.separator {
        module.add_separator(sep.as_str());
    }

    if let Some(ref blocks) = cmodule.block {
        for block in blocks {
            module.add_boxed(build_block(&block));
        }
    }

    module
}

fn main() {
    let config: Config = parse_config();

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

    // Set up modules
    let mut modules: Vec<Module> = Vec::new();

    if let Some(ref cmodules) = config.module {
        for module in cmodules {
            modules.push(build_module(module));
        }
    }

    // Add modules
    for module in modules {
        bar.add_module(module);
    }

    // Run
    if let Some(ref wm) = config.bar.wm {
        match wm.as_ref() {
            "bspwm" => bar.subscribe(WindowManagers::Bspwm),
            _ => bar.display(),
        }
    }
}
