pub trait WindowManager {
    fn new() -> Self where Self: Sized;

    // Get total desktops
    fn num_desktops(&self) -> i32;

    // Get populated desktops
    fn active_desktops(&self) -> Vec<i32>;

    // Get currently selected desktop
    fn current_desktop(&self) -> i32;
}
