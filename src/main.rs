// main.rs is temporary, for testing
extern crate rustabari;

use rustabari::bar::Bar;
use rustabari::block::Block;
use rustabari::module::Module;
use rustabari::blocks::Date;
use rustabari::blocks::Battery;
use rustabari::util::Align;

fn main() {
    // Initialize a new bar with the update interval set to 1000ms
    let mut bar = Bar::new(1000);

    let battery = Battery::new(Some("Batt:"));
    let time = Date::new(None, "%I:%M %p");

    let mut module = Module::new(Align::Left);
    module.add(battery);
    module.add(time);

    bar.add_module(module);
    bar.display();
}
