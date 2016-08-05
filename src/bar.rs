use block::Block;

pub struct Bar {
    pub blocks: Vec<Box<Block>>,
}

impl Bar {
    pub fn new() -> Bar {
        Bar {
            blocks: Vec::new(),
        }
    }

    pub fn add_block<T: Block + 'static>(&mut self, block: T) {
        self.blocks.push(Box::new(block));
    }
}
