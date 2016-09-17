use std::thread;
use std::time::Duration;

use block::Block;
use module::Module;

pub struct Bar {
    update_interval: u64,
    blocks: Vec<Box<Block>>,
    groups: Vec<Module>,
    separator: Option<String>,
}

impl Bar {
    pub fn new(updates: u64) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            groups: Vec::new(),
            separator: None,
        }
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
            for i in 0..self.blocks.len() {
                let ref block = self.blocks[i];

                println!("{}", block.output());

                // Only print separator if not last black
                if i < self.blocks.len() - 1 {
                    if let Some(ref s) = self.separator {
                        println!("{}", s);
                    }
                }
            }

            for group in self.groups.iter_mut() {
                if let Some(ref s) = self.separator {
                    group.add_separator(s);
                }

                println!("{}", group.output());
            }

            thread::sleep(Duration::from_millis(self.update_interval));
        }
    }
}
