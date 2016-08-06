use std::thread;
use std::time::Duration;

use block::Block;
use blockgroup::BlockGroup;

pub struct Bar {
    update_interval: u64,
    blocks: Vec<Box<Block>>,
    groups: Vec<BlockGroup>,
}

impl Bar {
    pub fn new(updates: u64) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }

    pub fn add_blockgroup(&mut self, group: BlockGroup) {
        self.groups.push(group);
    }

    pub fn display(&self) {
        loop {
            for block in self.blocks.iter() {
                println!("{}", block.output());
            }

            for group in self.groups.iter() {
                println!("{}", group.output());
            }

            thread::sleep(Duration::from_millis(self.update_interval));
        }
    }
}
