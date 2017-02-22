use block::Block;
use util::Align;

pub struct Module {
    blocks: Vec<Box<Block>>,
    align: String,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
}

impl Module {
   pub fn new(align: Align) -> Module {
       Module {
           blocks: Vec::new(),
           align: format!("%{{{}}}", match align {
               Align::Left => 'l',
               Align::Center => 'c',
               Align::Right => 'r',
           }),
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

   pub fn add<T: Block + 'static>(&mut self, block: T) {
       self.blocks.push(Box::new(block));
   }

   pub fn add_boxed(&mut self, block: Box<Block>) {
       self.blocks.push(block);
   }

   pub fn output(&self) -> String {
       let mut out = String::new();

       for i in 0..self.blocks.len() {
           let block = &self.blocks[i];

           out.push_str(&block.output());

           // Only print separator if not last block
           if i < self.blocks.len() - 1 {
               match self.separator.to_owned() {
                   Some(s) => out.push_str(s.as_str()),
                   None => out.push(' '),
               }
           }
       }

       let mut res = String::new();

       if let Some(ref bg) = self.background {
           res.push_str(&format!("%{{B{}}}", bg));
       }

       if let Some(ref fg) = self.foreground {
           res.push_str(&format!("%{{F{}}}", fg));
       }

       res.push_str(&self.align.to_owned());
       res.push_str(&out);

       res
   }
}
