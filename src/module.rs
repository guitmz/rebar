use block::Block;
use util::Align;

pub struct Module {
    blocks: Vec<Box<Block>>,
    align: String,
    separator: Option<String>,
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
           separator: None,
       }
   }

   pub fn add_separator(&mut self, sep: &str) {
       self.separator = Some(String::from(sep));
   }

   pub fn add<T: Block + 'static>(&mut self, block: T) {
       self.blocks.push(Box::new(block));
   }

   pub fn output(&self) -> String {
       let mut out = String::new();

       for i in 0..self.blocks.len() {
           let ref block = self.blocks[i];

           out.push_str(&block.output());

           // Only print separator if not last black
           if i < self.blocks.len() - 1 {
               match self.separator.to_owned() {
                   Some(s) => out.push_str(s.as_str()),
                   None => out.push(' '),
               }
           }
       }

       let mut align = self.align.to_owned();
       align.push_str(&out);

       align
   }
}
