// main.rs is temporary, for testing
extern crate barrust;

use barrust::bar::Bar;
use barrust::block::Block;
use barrust::module::Module;
use barrust::blocks::Date;
use barrust::blocks::Battery;
use barrust::util::Align;

fn main() {
    // Initialize a new bar with the update interval set to 1000ms
    let mut bar = Bar::new(1000);

    let time = Date::new(None, "%r");
    let date = Date::new(None, "%Y-%m-%d");

    let mut module = Module::new(Align::Left);
    module.add(time);
    module.add(date);

    bar.add_module(module);
    bar.display();
}
