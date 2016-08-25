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
    let mut bar = Bar::new(1000, Some(" | "));

    let battery = Battery::new(Some("Batt:"));
    let time = Date::new("%I:%M %p", None);

    let mut module = Module::new(Align::Center);
    module.add(battery);
    module.add(time);

    bar.add_module(module);
    bar.display();
}
