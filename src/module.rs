use block::Block;
use util::Align;

pub struct Module {
    blocks: Vec<Box<Block>>,
    align: String,
}

impl Module {
   pub fn new(align: Align) -> Module {
       let alignchar;

       match align {
           Align::Left => alignchar = 'l',
           Align::Center => alignchar = 'c',
           Align::Right => alignchar = 'r',
       }

       Module {
           blocks: Vec::new(),
           align: format!("%{{{}}}", alignchar),
       }
   }

   pub fn add<T: Block + 'static>(&mut self, block: T) {
       self.blocks.push(Box::new(block));
   }

   pub fn output(&self) -> String {
       let mut out = String::new();

       for block in self.blocks.iter() {
           out.push_str(&block.output());
           out.push(' ');
       }

       let mut align = self.align.to_owned();
       align.push_str(&out);

       align
   }
}
