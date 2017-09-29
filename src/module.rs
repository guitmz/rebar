use block::Block;
use util::{Align, opacity_to_hex};

pub struct Module {
    blocks: Vec<Box<Block>>,
    align: Option<String>,
    separator: Option<String>,
    background: Option<String>,
    background_opacity: Option<String>,
    foreground: Option<String>,
    foreground_opacity: Option<String>,
}

impl Module {
    pub fn new(align: Align) -> Module {
        Module {
            blocks: Vec::new(),
            align: match align {
                Align::Left => Some("%{l}".to_string()),
                Align::Center => Some("%{c}".to_string()),
                Align::Right => Some("%{r}".to_string()),
                Align::None => None,
            },
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

    pub fn add_separator(&mut self, sep: &str) {
        self.separator = Some(String::from(sep));
    }

    pub fn add(&mut self, block: Box<Block>) {
        self.blocks.push(block);
    }

    pub fn output(&self) -> String {
        let mut out = String::new();

        // Add each block
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
            if let Some(ref bgo) = self.background_opacity {
                let argb = String::from("#") + bgo + &bg[1..];
                res.push_str(&format!("%{{B{}}}", argb));
            } else {
                res.push_str(&format!("%{{B{}}}", bg));
            }
        }

        if let Some(ref fg) = self.foreground {
            if let Some(ref fgo) = self.foreground_opacity {
                let argb = String::from("#") + fgo + &fg[1..];
                res.push_str(&format!("%{{F{}}}", argb));
            } else {
                res.push_str(&format!("%{{F{}}}", fg));
            }
        }

        if let Some(ref align) = self.align {
            res.push_str(align);
        }

        res.push_str(&out);

        res
    }
}
