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
    pub fn new(updates: u64, sep: Option<&str>) -> Bar {
        let get_sep = |s| {
            if let Some(x) = s {
                Some(String::from(x))
            } else {
                None
            }
        };

        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            groups: Vec::new(),
            separator: get_sep(sep),
        }
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }

    pub fn add_module(&mut self, group: Module) {
        self.groups.push(group);
    }

    pub fn display(&self) {
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

            for group in self.groups.iter() {
                println!("{}", group.output(self.separator.to_owned()));
            }

            thread::sleep(Duration::from_millis(self.update_interval));
        }
    }
}
