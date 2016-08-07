extern crate barrust;

use barrust::block::Block;
use barrust::module::Module;
use barrust::blocks::Date;
use barrust::util::Align;

#[test]
fn test_module() {
    let mut module = Module::new(Align::Left);

    let time = Date::new(None, "%r");
    let date = Date::new(None, "%Y-%m-%d");

    let time_output = time.output();
    let date_output = date.output();

    module.add(time);
    module.add(date);

    assert_eq!(module.output(), format!("%{{l}}{}{}", time_output, date_output));
}
