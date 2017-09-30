extern crate rebar;

use rebar::block::Block;
use rebar::module::Module;
use rebar::blocks::Date;
use rebar::util::Align;

#[test]
fn test_module() {
    let mut module = Module::new(Align::Left);

    let time = Date::new("%r");
    let date = Date::new("%Y-%m-%d");

    let time_output = time.output();
    let date_output = date.output();

    module.add(time);
    module.add(date);

    assert_eq!(module.output(), format!("%{{l}}{} {}", time_output, date_output));
}
