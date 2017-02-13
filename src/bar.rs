use std::thread;
use std::time::Duration;
use time::{get_time};

use block::Block;
use module::Module;
use util::Workspaces;
use util::{run_i32, run_bg};

pub struct Bar {
    update_interval: u64,
    blocks: Vec<Box<Block>>,
    groups: Vec<Module>,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
}

impl Bar {
    pub fn new(updates: u64) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            groups: Vec::new(),
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

    pub fn add_separator(&mut self, sep: &str) {
        self.separator = Some(String::from(sep));
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }

    pub fn add_module(&mut self, group: Module) {
        self.groups.push(group);
    }

    fn run(&mut self) {
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
        for group in &mut self.groups {
            print!("{}", group.output());
        }

        println!("");
    }

    pub fn display(&mut self) {
        loop {
            self.run();

            thread::sleep(Duration::from_secs(self.update_interval));
        }
    }

    pub fn subscribe(&mut self, wsp: Workspaces) {
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

            if len != file_length {
                file_length = len;

                self.run();
            } else if elapsed != previous && elapsed as u64 % self.update_interval == 0 {
                previous = elapsed;

                self.run();
            }
        }
    }
}
