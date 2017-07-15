use std::thread;
use std::time::Duration;
use time::get_time;

use block::Block;
use module::Module;
use util::{WindowManagers, run_bg, run_i32};

pub struct Bar {
    pub update_interval: u64,
    blocks: Vec<Box<Block>>,
    modules: Vec<Module>,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
}

impl Bar {
    pub fn new(updates: u64) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            modules: Vec::new(),
            separator: None,
            background: None,
            foreground: None,
        }
    }

    pub fn set_background(&mut self, color: &str) {
        self.background = Some(String::from(color));
    }

    pub fn set_foreground(&mut self, color: &str) {
        self.foreground = Some(String::from(color));
    }

    pub fn set_separator(&mut self, sep: &str) {
        self.separator = Some(String::from(sep));
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }

    pub fn add_boxed(&mut self, block: Box<Block>) {
        self.blocks.push(block);
    }

    pub fn add_module(&mut self, group: Module) {
        self.modules.push(group);
    }

    pub fn run(&self) {
        // Print background and foreground
        if let Some(ref bg) = self.background {
            print!("%{{B{}}}", bg);
        }

        if let Some(ref fg) = self.foreground {
            print!("%{{F{}}}", fg);
        }

        // Print blocks added to bar
        for i in 0..self.blocks.len() {
            let block = &self.blocks[i];

            print!("{}", block.output());

            // Only print separator if not last block
            if i < self.blocks.len() - 1 {
                if let Some(ref s) = self.separator {
                    print!("{}", s);
                }
            }
        }

        // Print each module
        for module in &self.modules {
            print!("{}", module.output());
        }

        println!("");
    }

    pub fn display(&self) {
        loop {
            self.run();

            thread::sleep(Duration::from_secs(self.update_interval));
        }
    }

    pub fn subscribe(&self, wsp: WindowManagers) {
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

                self.run();
            } else if elapsed != previous && elapsed as u64 % self.update_interval == 0 {
                previous = elapsed;

                self.run();
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}
