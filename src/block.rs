use util::Align;

pub trait Block {
    fn new(icon: Option<(&str, Align)>) -> Self where Self: Sized;
    fn output(&self) -> String;
}
