// main.rs is temporary, for testing
extern crate rustabari;

use rustabari::bar::Bar;
use rustabari::module::Module;
use rustabari::blocks::Date;
use rustabari::blocks::Battery;
use rustabari::blocks::Music;
use rustabari::blocks::Title;
use rustabari::blocks::Wifi;
use rustabari::blocks::Wsp;
use rustabari::util::Align;
use rustabari::util::Workspaces;

fn main() {
    // Initialize a new bar with the update interval set to 1s
    let mut bar = Bar::new(1);
    bar.add_separator(" | ");

    let mut battery = Battery::new();
    battery.add_icons(["\u{e1fd}", "\u{e1fe}", "\u{e1ff}"], Align::Right);

    let mut date = Date::new("%a %b %d");
    date.add_icon("\u{e1cd}", Align::Left);

    let mut time = Date::new("%I:%M %p");
    time.add_icon("\u{e015}", Align::Left);

    let mut music = Music::new();
    music.add_icon("\u{e1a6}", Align::Left);
    music.set_command("gpmdp-remote current");

    let mut wifi = Wifi::new();
    wifi.add_icons(["\u{e0f1}", "\u{e0f2}", "\u{e0f3}"], Align::Left);
    wifi.set_device("wlp2s0");

    let mut wsp = Wsp::new();
    wsp.set_icon("\u{e001}");
    wsp.set_active_icon("\u{e000}");

    let mut mod1 = Module::new(Align::Left);
    mod1.add(wsp);

    let title = Title::new();

    let mut mod2 = Module::new(Align::Center);
    mod2.add(title);

    let mut module = Module::new(Align::Right);
    module.add_separator(" | ");
    module.add(wifi);
    module.add(music);
    module.add(battery);
    module.add(date);
    module.add(time);

    bar.add_module(mod1);
    bar.add_module(mod2);
    bar.add_module(module);
    bar.subscribe(Workspaces::Bspwm);
}
