// main.rs is temporary, for testing
extern crate barrust;

use barrust::bar::Bar;
use barrust::block::Block;
use barrust::blockgroup::BlockGroup;
use barrust::blocks::Battery;
use barrust::util::Align;

fn main() {
    // Initialize a new bar with the update interval set to 1000ms
    let mut bar = Bar::new(1000);

    let battery = Battery::new(None);

    //bar.add_block(battery);
    //bar.display();

    let mut group = BlockGroup::new(Align::Center);
    group.add(battery);

    bar.add_blockgroup(group);
    bar.display();
}
