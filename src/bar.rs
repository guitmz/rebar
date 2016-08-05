use std::thread;
use std::time::Duration;

use block::Block;

pub struct Bar {
    pub update_interval: i32,
    pub blocks: Vec<Box<Block>>,
}

impl Bar {
    pub fn new(updates: i32) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
        }
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }

    pub fn display(&self) {
        loop {
            for block in self.blocks.iter() {
                println!("{}", block.output());
            }

            thread::sleep(Duration::from_secs(self.update_interval as u64));
        }
    }
}
