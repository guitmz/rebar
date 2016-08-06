use block::Block;
use util::Align;

pub struct BlockGroup {
    blocks: Vec<Box<Block>>,
    align: String,
}

impl BlockGroup {
   pub fn new(align: Align) -> BlockGroup {
       let alignchar;

       match align {
           Align::Left => alignchar = 'l',
           Align::Center => alignchar = 'c',
           Align::Right => alignchar = 'r',
       }

       BlockGroup {
           blocks: Vec::new(),
           align: format!("%{{{}}}", alignchar),
       }
   }

   pub fn add<T: Block + 'static>(&mut self, block: T) {
       self.blocks.push(Box::new(block));
   }

   pub fn output(&self) -> String {
       let mut out = self.align.to_owned();

       for block in self.blocks.iter() {
           out = format!("{}{}", out, block.output());
       }

       out
   }
}
