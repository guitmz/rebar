// main.rs is temporary, for testing
extern crate barrust;

use barrust::bar::Bar;
use barrust::block::Block;
use barrust::blocks::Battery;

fn main() {
    let mut bar = Bar::new();

    let battery = Battery::new(None);

    bar.add_block(battery);

    for i in bar.blocks.iter() {
        println!("{}", i.output());
    }
}
