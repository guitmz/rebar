extern crate rustabari;

use rustabari::block::Block;
use rustabari::module::Module;
use rustabari::blocks::Date;
use rustabari::util::Align;

#[test]
fn test_module() {
    let mut module = Module::new(Align::Left);

    let time = Date::new("%r", None);
    let date = Date::new("%Y-%m-%d", None);

    let time_output = time.output();
    let date_output = date.output();

    module.add(time);
    module.add(date);

    assert_eq!(module.output(None), format!("%{{l}}{} {}", time_output, date_output));
}
