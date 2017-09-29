use block::Block;
use module::Module;
use util::opacity_to_hex;

pub struct Bar {
    pub update_interval: u64,
    blocks: Vec<Box<Block>>,
    modules: Vec<Module>,
    separator: Option<String>,
    background: Option<String>,
    background_opacity: Option<String>,
    foreground: Option<String>,
    foreground_opacity: Option<String>,
}

impl Bar {
    pub fn new(updates: u64) -> Bar {
        Bar {
            update_interval: updates,
            blocks: Vec::new(),
            modules: Vec::new(),
            separator: None,
            background: None,
            background_opacity: None,
            foreground: None,
            foreground_opacity: None,
        }
    }

    pub fn set_background(&mut self, color: &str) {
        self.background = Some(String::from(color));
    }

    pub fn set_background_opacity(&mut self, opacity: u32) {
        self.background_opacity = Some(opacity_to_hex(opacity));
    }

    pub fn set_foreground(&mut self, color: &str) {
        self.foreground = Some(String::from(color));
    }

    pub fn set_foreground_opacity(&mut self, opacity: u32) {
        self.foreground_opacity = Some(opacity_to_hex(opacity));
    }

    pub fn set_separator(&mut self, sep: &str) {
        self.separator = Some(String::from(sep));
    }

    pub fn add_block(&mut self, block: Box<Block>) {
        self.blocks.push(block);
    }

    pub fn add_module(&mut self, group: Module) {
        self.modules.push(group);
    }

    pub fn run(&self) {
        // Print background and foreground
        if let Some(ref bg) = self.background {
            if let Some(ref bgo) = self.background_opacity {
                let argb = String::from("#") + bgo + &bg[1..];
                print!("%{{B{}}}", argb);
            } else {
                print!("%{{B{}}}", bg);
            }
        } else {
            print!("%{{B-}}");
        }

        if let Some(ref fg) = self.foreground {
            if let Some(ref fgo) = self.foreground_opacity {
                let argb = String::from("#") + fgo + &fg[1..];
                print!("%{{F{}}}", argb);
            } else {
                print!("%{{F{}}}", fg);
            }
        } else {
            print!("%{{F-}}");
        }

        // Print blocks added to bar
        for i in 0..self.blocks.len() {
            let block = &self.blocks[i];

            print!("{}", block.output());

            // Only print separator if not last block
            if i < self.blocks.len() - 1 {
                if let Some(ref s) = self.separator {
                    print!("{}", s);
                }
            }
        }

        // Print each module
        for module in &self.modules {
            print!("{}", module.output());
        }

        println!("");
    }
}
