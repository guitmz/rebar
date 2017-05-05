use block::Block;
use util::Align;

pub struct Module {
    blocks: Vec<Box<Block>>,
    align: Option<String>,
    separator: Option<String>,
    background: Option<String>,
    foreground: Option<String>,
    global_background: Option<String>,
    global_foreground: Option<String>,
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
            foreground: None,
            global_background: None,
            global_foreground: None,
        }
    }

    pub fn set_background(&mut self, color: &str) {
        self.background = Some(String::from(color));
    }

    pub fn set_foreground(&mut self, color: &str) {
        self.foreground = Some(String::from(color));
    }

    pub fn set_global_background(&mut self, color: &str) {
        self.global_background = Some(String::from(color));
    }

    pub fn set_global_foreground(&mut self, color: &str) {
        self.global_foreground = Some(String::from(color));
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
            res.push_str(&format!("%{{B{}}}", bg));
        } else if let Some(ref bg) = self.global_background {
            res.push_str(&format!("%{{B{}}}", bg));
        }

        if let Some(ref fg) = self.foreground {
            res.push_str(&format!("%{{F{}}}", fg));
        } else if let Some(ref fg) = self.global_foreground {
            res.push_str(&format!("%{{F{}}}", fg));
        }

        if let Some(ref align) = self.align {
            res.push_str(align);
        }

        res.push_str(&out);

        res
    }
}
