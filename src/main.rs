// main.rs is temporary, for testing
extern crate rustabari;

use rustabari::bar::Bar;
use rustabari::module::Module;
use rustabari::blocks::Date;
use rustabari::blocks::Battery;
use rustabari::blocks::Music;
use rustabari::util::Align;

fn main() {
    // Initialize a new bar with the update interval set to 1000ms
    let mut bar = Bar::new(1000);
    bar.add_separator(" | ");

    let mut battery = Battery::new();
    battery.add_icon("\u{e1ff}", Align::Right);

    let mut date = Date::new("%a %b %d");
    date.add_icon("\u{e1cd}", Align::Left);

    let mut time = Date::new("%I:%M %p");
    time.add_icon("\u{e015}", Align::Left);

    let mut music = Music::new();
    music.add_icon("\u{e1a6}", Align::Left);

    let mut module = Module::new(Align::Center);
    module.add_separator(" | ");
    module.add(music);
    module.add(battery);
    module.add(date);
    module.add(time);

    bar.add_module(module);
    bar.display();
}
