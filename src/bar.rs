use std::thread;
use std::time::Duration;

use block::Block;
use module::Module;

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

    pub fn display(&mut self) {
        loop {
            // Print background and foreground
            if let Some(ref bg) = self.background {
                print!("%{{B{}}}", bg);
            }

            if let Some(ref fg) = self.foreground {
                print!("%{{F{}}}", fg);
            }

            // Print blocks added to bar
            for i in 0..self.blocks.len() {
                let ref block = self.blocks[i];

                print!("{}", block.output());

                // Only print separator if not last block
                if i < self.blocks.len() - 1 {
                    if let Some(ref s) = self.separator {
                        print!("{}", s);
                    }
                }
            }

            // Print each module
            for group in self.groups.iter_mut() {
                print!("{}", group.output());
            }

            println!("");

            thread::sleep(Duration::from_millis(self.update_interval));
        }
    }
}
