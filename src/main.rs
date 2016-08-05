// main.rs is temporary, for testing
extern crate barrust;

use barrust::bar::Bar;
use barrust::block::Block;
use barrust::blocks::Battery;

fn main() {
    let mut bar = Bar::new(1);

    let battery = Battery::new(Some("Batt:"));

    bar.add_block(battery);
    bar.display();
}
